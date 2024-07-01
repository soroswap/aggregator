#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, String, BytesN};

mod models;
mod error;
mod event;
mod storage;
mod proxy;
mod test;

use storage::{
    put_proxy_address, 
    has_proxy_address, 
    get_proxy_address, 
    remove_proxy_address, 
    get_protocol_ids, 
    extend_instance_ttl, 
    is_initialized, 
    set_initialized, 
    set_admin, 
    get_admin,
    set_pause_protocol,
    is_protocol_paused,
};
use models::{DexDistribution, ProxyAddressPair, MAX_DISTRIBUTION_LENGTH};
use error::{AggregatorError};
use proxy::SoroswapAggregatorProxyClient;

pub fn check_nonnegative_amount(amount: i128) -> Result<(), AggregatorError> {
    if amount < 0 {
        Err(AggregatorError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

/// Panics if the specified deadline has passed.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `timestamp` - The deadline timestamp to compare against the current ledger timestamp.
fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), AggregatorError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(AggregatorError::DeadlineExpired)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), AggregatorError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(AggregatorError::NotInitialized)
    }
}

fn check_admin(e: &Env) {
    let admin: Address = get_admin(&e);
    admin.require_auth();
}

/*
    SOROSWAP AGGREGATOR SMART CONTRACT INTERFACE:
*/

pub trait SoroswapAggregatorTrait {

    /* ADMIN FUNCTIONS */

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, admin: Address, proxy_addresses: Vec<ProxyAddressPair>) -> Result<(), AggregatorError>;

    /// Updates the protocol addresses for the aggregator
    fn update_protocols(
        e: Env,
        proxy_addresses: Vec<ProxyAddressPair>,
    ) -> Result<(), AggregatorError>;

    /// Removes the protocol from the aggregator
    fn remove_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError>;

    /// Pauses the protocol from the aggregator
    fn pause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError>;

    /// Unpause the protocol from the aggregator
    fn unpause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), AggregatorError>;

    /// Sets the `admin` address.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `new_admin` - The address to set as the new `admin`.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the Aggregator is not yet initialized or if the caller is not the existing `admin`.
    fn set_admin(e: Env, new_admin: Address) -> Result<(), AggregatorError>;

    /* SWAP FUNCTION */

    /// Executes a swap operation distributed across multiple decentralized exchanges (DEXes) as specified
    /// by the `distribution`. Each entry in the distribution details which DEX to use, the path of tokens
    /// for swap (if applicable), and the portion of the total `amount_in` to swap through that DEX. This 
    /// function aims to optimize the swap by leveraging different DEX protocols based on the distribution
    /// strategy to minimize slippage and maximize output.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `input_token` - The address of the input token to swap.
    /// * `output_token` - The address of the destination token to receive.
    /// * `amount_in` - The total amount of `input_token` to be swapped.
    /// * `amount_out_min` - The minimum amount of `output_token` expected to receive, ensuring the swap 
    ///   does not proceed under unfavorable conditions.
    /// * `distribution` - A vector of `DexDistribution` specifying how the total swap amount is distributed 
    ///   across different DEX protocols, including the swap path for each (if required by the DEX).
    /// * `to` - The recipient address for the `output_token`.
    /// * `deadline` - A Unix timestamp marking the deadline by which the swap must be completed.
    ///
    /// # Returns
    /// The total amount of `output_token` received from the swap if successful, encapsulated in a `Result`.
    /// On failure, returns a `AggregatorError` detailing the cause of the error.
    fn swap(
        e: Env,
        input_token: Address,
        output_token: Address,
        amount_in: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError>;

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, AggregatorError>;
    fn get_protocols(e: &Env) -> Result<Vec<ProxyAddressPair>, AggregatorError>;
    fn is_protocol_paused(e: &Env, protocol_id: String) -> bool;
    fn get_version() -> u32;

}

#[contract]
struct SoroswapAggregator;

#[contractimpl]
impl SoroswapAggregatorTrait for SoroswapAggregator {

    /* ADMIN FUNCTIONS */

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(
        e: Env,
        admin: Address,
        proxy_addresses: Vec<ProxyAddressPair>,
    ) -> Result<(), AggregatorError> {
        if is_initialized(&e) {
            return Err(AggregatorError::AlreadyInitialized);
        }
    
        for pair in proxy_addresses.iter() {
            put_proxy_address(&e, pair);
        }

        set_admin(&e, admin);
    
        // Mark the contract as initialized
        set_initialized(&e);
        event::initialized(&e, true, proxy_addresses);
        extend_instance_ttl(&e);
        Ok(())
    }
        
    fn update_protocols(
        e: Env,
        proxy_addresses: Vec<ProxyAddressPair>,
    ) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        for pair in proxy_addresses.iter() {
            put_proxy_address(&e, pair);
        }
    
        event::protocols_updated(&e, proxy_addresses);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn remove_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);
        
        remove_proxy_address(&e, protocol_id.clone());
    
        event::protocol_removed(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn pause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);
        
        set_pause_protocol(&e, protocol_id.clone(), true);
    
        event::protocol_paused(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn unpause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);
        
        set_pause_protocol(&e, protocol_id.clone(), false);
    
        event::protocol_unpaused(&e, protocol_id);
        extend_instance_ttl(&e);
        
        Ok(())
    }

    fn set_admin(e: Env, new_admin: Address) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        let admin: Address = get_admin(&e);
        set_admin(&e, new_admin.clone());

        event::new_admin(&e, admin, new_admin);
        Ok(())
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }

    // ** SWAP FUNCTIONS ** //

    fn swap(
        e: Env,
        _from_token: Address,
        _dest_token: Address,
        amount: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        check_nonnegative_amount(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;
    
        if distribution.len() > MAX_DISTRIBUTION_LENGTH {
            return Err(AggregatorError::DistributionLengthExceeded);
        }
    
        let total_parts: i128 = distribution.iter().map(|dist| dist.parts).sum();    
    
        if total_parts == 0 {
            return Err(AggregatorError::InvalidTotalParts);
        }

        // Vector to store responses from each swap
        let mut swap_responses: Vec<i128> = Vec::new(&e); 
        let mut total_swapped: i128 = 0;
    
        for (index, dist) in distribution.iter().enumerate() {
            let swap_amount = if index == (distribution.len() - 1) as usize {
                // For the last iteration, swap whatever remains
                amount.checked_sub(total_swapped).ok_or(AggregatorError::ArithmeticError)?
            } else {
                // Calculate part of the total amount based on distribution parts
                amount.checked_mul(dist.parts)
                    .and_then(|prod| prod.checked_div(total_parts))
                    .ok_or(AggregatorError::ArithmeticError)?
            };
            
            let proxy_contract_address = get_proxy_address(&e, dist.protocol_id.clone())?;
            let proxy_client = SoroswapAggregatorProxyClient::new(&e, &proxy_contract_address);
            let response = proxy_client.swap(
                &to, 
                &dist.path, 
                &swap_amount, 
                &amount_out_min, 
                &deadline, 
                &dist.is_exact_in
            );
        
            // Store the response from the swap
            for item in response.iter() {
                swap_responses.push_back(item);
            }
            total_swapped = total_swapped.checked_add(swap_amount).ok_or(AggregatorError::ArithmeticError)?;
        }
    
        event::swap(&e, amount, distribution, to);
        Ok(swap_responses)
    }

    
    
    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, AggregatorError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_protocols(e: &Env) -> Result<Vec<ProxyAddressPair>, AggregatorError> { 
        check_initialized(&e)?;

        let protocol_ids = get_protocol_ids(e);
        let mut addresses = Vec::new(e);
    
        // Iterate over each protocol ID and collect their proxy addresses
        for protocol_id in protocol_ids.iter() {
            if has_proxy_address(e, protocol_id.clone()) {
                let address = get_proxy_address(e, protocol_id.clone())?;
                addresses.push_back(ProxyAddressPair {
                    protocol_id: protocol_id,
                    address,
                });
            }
        }
    
        Ok(addresses)
    }   

    fn is_protocol_paused(
        e: &Env,
        protocol_id: String,
    ) -> bool {
        is_protocol_paused(&e, protocol_id)
    }

    /// this is the firs version of the contract   
    fn get_version() -> u32 {
        1
    }

}
