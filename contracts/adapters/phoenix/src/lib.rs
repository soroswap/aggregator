#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, String};

mod event;
mod storage;
mod protocol_interface;
// mod test;

use storage::{
    extend_instance_ttl, 
    set_initialized, 
    is_initialized, 
    set_protocol_id,
    get_protocol_id,
    set_protocol_address, 
    has_protocol_address,
    get_protocol_address, 
};
use soroswap_aggregator_adapter_interface::{SoroswapAggregatorAdapterTrait, AdapterError};
use protocol_interface::{protocol_swap};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), AdapterError> {
    if amount < 0 {
        Err(AdapterError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), AdapterError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(AdapterError::DeadlineExpired)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), AdapterError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(AdapterError::NotInitialized)
    }
}

#[contract]
struct SoroswapAggregatorPhoenixAdapter;

#[contractimpl]
impl SoroswapAggregatorAdapterTrait for SoroswapAggregatorPhoenixAdapter {
    /// Initializes the contract and sets the phoenix multihop address
    fn initialize(
        e: Env,
        protocol_id: String,
        protocol_address: Address,
    ) -> Result<(), AdapterError> {
        if is_initialized(&e) {
            return Err(AdapterError::AlreadyInitialized);
        }
    
        set_protocol_id(&e, protocol_id.clone());
        set_protocol_address(&e, protocol_address.clone());
    
        set_initialized(&e);
        event::initialized(&e, true, protocol_id, protocol_address);
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
        is_exact_in: bool,
    ) -> Result<Vec<i128>, AdapterError> {
        check_initialized(&e)?;
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
            is_exact_in
        )?;


        event::swap(&e, amount_in, path, to);
        Ok(swap_result)
    }

    /*  *** Read only functions: *** */
    fn get_protocol_id(e: &Env) -> Result<Address, AdapterError> {
        check_initialized(&e)?;
        
        let address = get_protocol_address(e);
        Ok(address)
    }    
    
    fn get_protocol_address(e: &Env) -> Result<Address, AdapterError> {
        check_initialized(&e)?;
        
        if !has_protocol_address(e) {
            return Err(AdapterError::ProtocolAddressNotFound);
        }

        let address = get_protocol_id(e);
        Ok(address)
    }    
}
