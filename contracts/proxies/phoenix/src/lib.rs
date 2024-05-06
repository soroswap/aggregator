#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod error;
mod event;
mod storage;
mod protocol_interface;
// mod test;

use storage::{
    put_protocol_address, 
    has_protocol_address, 
    get_protocol_address, 
    extend_instance_ttl, 
    is_initialized, 
    set_initialized, 
    set_admin, 
    get_admin,
    set_paused,
    is_paused
};
pub use error::{SoroswapAggregatorProxyError, CombinedProxyError};
use protocol_interface::{protocol_swap};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedProxyError> {
    if amount < 0 {
        Err(CombinedProxyError::ProxyNegativeNotAllowed)
    } else {
        Ok(())
    }
}

/// Panics if the specified deadline has passed.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `timestamp` - The deadline timestamp to compare against the current ledger timestamp.
fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), CombinedProxyError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(SoroswapAggregatorProxyError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), CombinedProxyError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(CombinedProxyError::ProxyNotInitialized)
    }
}

fn check_is_paused(e: &Env) -> Result<(), CombinedProxyError> {
    if !is_paused(e) {
        Ok(())
    } else {
        Err(CombinedProxyError::ProxyNotInitialized)
    }
}

/*
    AGGREGATOR PROXY SMART CONTRACT INTERFACE:
*/

pub trait AggregatorProxyTrait {

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, admin: Address, protocol_address: Address) -> Result<(), CombinedProxyError>;

    /// Updates the protocol addresses for the aggregator
    fn update_protocol(
        e: Env,
        protocol_address: Address,
    ) -> Result<(), CombinedProxyError>;

    fn pause(
        e: Env,
    ) -> Result<(), CombinedProxyError>;

    fn unpause(
        e: Env,
    ) -> Result<(), CombinedProxyError>;
    
    fn swap(
        e: Env,
        to: Address,
        path: Vec<Address>,
        amount_in: i128,
        amount_out_min_or_max: i128,
        deadline: u64,
        isExactIn: bool,
    ) -> Result<Vec<i128>, CombinedProxyError>;

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedProxyError>;
    fn get_protocol_address(e: &Env) -> Result<Address, CombinedProxyError>;
    fn is_paused(e: &Env) -> bool;

}

#[contract]
struct SoroswapAggregatorProxyForPhoenix;

#[contractimpl]
impl AggregatorProxyTrait for SoroswapAggregatorProxyForPhoenix {
    /// Initializes the contract and sets the phoenix multihop address
    fn initialize(
        e: Env,
        admin: Address,
        protocol_address: Address,
    ) -> Result<(), CombinedProxyError> {
        if is_initialized(&e) {
            return Err(CombinedProxyError::ProxyInitializeAlreadyInitialized.into());
        }
    
        set_admin(&e, admin);
        put_protocol_address(&e, protocol_address.clone());
    
        // Mark the contract as initialized
        set_initialized(&e);
        event::initialized(&e, true, protocol_address);
        extend_instance_ttl(&e);
        Ok(())
    }
    
    fn update_protocol(
        e: Env,
        protocol_address: Address,
    ) -> Result<(), CombinedProxyError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        // Check if the sender is the admin
        
        put_protocol_address(&e, protocol_address.clone());
    
        event::protocol_updated(&e, protocol_address);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn pause(
        e: Env,
    ) -> Result<(), CombinedProxyError> {
        check_initialized(&e)?;
        check_is_paused(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();

        set_paused(&e, true);
        event::protocol_paused(&e, true);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn unpause(
        e: Env,
    ) -> Result<(), CombinedProxyError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();

        set_paused(&e, false);
        event::protocol_paused(&e, false);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn swap(
        e: Env,
        to: Address,
        path: Vec<Address>,
        amount_in: i128,
        amount_out_min_or_max: i128,
        deadline: u64,
        isExactIn: bool,
    ) -> Result<Vec<i128>, CombinedProxyError> {
        check_initialized(&e)?;
        check_is_paused(&e)?;
        check_nonnegative_amount(amount_in)?;
        check_nonnegative_amount(amount_out_min_or_max)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        let swap_result = protocol_swap(
            &e, 
            &amount_in, 
            &amount_out_min_or_max, 
            path.clone(), 
            to.clone(), 
            deadline, 
            isExactIn
        )?;


        event::swap(&e, amount_in, path, to);
        Ok(swap_result)
    }

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedProxyError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_protocol_address(e: &Env) -> Result<Address, CombinedProxyError> {
        check_initialized(&e)?;
        if !has_protocol_address(e) {
            return Err(CombinedProxyError::ProxyProtocolAddressNotFound);
        }
        
        let address = get_protocol_address(e);
        Ok(address)
    }    

    fn is_paused(e: &Env) -> bool {
        is_paused(&e)
    }

}
