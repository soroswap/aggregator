#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, String};

mod models;
mod error;
mod event;
mod storage;
mod test;

use storage::{put_proxy_address, has_proxy_address, get_proxy_address, remove_proxy_address, get_protocol_ids, extend_instance_ttl, is_initialized, set_initialized, set_admin, get_admin};
use models::{DexDistribution, ProxyAddressPair, MAX_DISTRIBUTION_LENGTH};
pub use error::{SoroswapAggregatorError, CombinedAggregatorError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedAggregatorError> {
    if amount < 0 {
        Err(CombinedAggregatorError::AggregatorNegativeNotAllowed)
    } else {
        Ok(())
    }
}

/// Panics if the specified deadline has passed.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `timestamp` - The deadline timestamp to compare against the current ledger timestamp.
fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), CombinedAggregatorError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(SoroswapAggregatorError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), CombinedAggregatorError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(CombinedAggregatorError::AggregatorNotInitialized)
    }
}

/*
    SOROSWAP AGGREGATOR SMART CONTRACT INTERFACE:
*/

pub trait SoroswapAggregatorTrait {

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, admin: Address, proxy_addresses: Vec<ProxyAddressPair>) -> Result<(), CombinedAggregatorError>;

    /// Updates the protocol addresses for the aggregator
    fn update_protocols(
        e: Env,
        proxy_addresses: Vec<ProxyAddressPair>,
    ) -> Result<(), CombinedAggregatorError>;

    /// Removes the protocol from the aggregator
    fn remove_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), CombinedAggregatorError>;

    /// Pauses the protocol from the aggregator
    fn pause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), CombinedAggregatorError>;

    /// Unpause the protocol from the aggregator
    fn unpause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), CombinedAggregatorError>;
    /// Executes a swap operation distributed across multiple decentralized exchanges (DEXes) as specified
    /// by the `distribution`. Each entry in the distribution details which DEX to use, the path of tokens
    /// for swap (if applicable), and the portion of the total `amount` to swap through that DEX. This 
    /// function aims to optimize the swap by leveraging different DEX protocols based on the distribution
    /// strategy to minimize slippage and maximize output.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `from_token` - The address of the input token to swap.
    /// * `dest_token` - The address of the destination token to receive.
    /// * `amount` - The total amount of `from_token` to be swapped.
    /// * `amount_out_min` - The minimum amount of `dest_token` expected to receive, ensuring the swap 
    ///   does not proceed under unfavorable conditions.
    /// * `distribution` - A vector of `DexDistribution` specifying how the total swap amount is distributed 
    ///   across different DEX protocols, including the swap path for each (if required by the DEX).
    /// * `to` - The recipient address for the `dest_token`.
    /// * `deadline` - A Unix timestamp marking the deadline by which the swap must be completed.
    ///
    /// # Returns
    /// The total amount of `dest_token` received from the swap if successful, encapsulated in a `Result`.
    /// On failure, returns a `CombinedAggregatorError` detailing the cause of the error.
    fn swap(
        e: Env,
        from_token: Address,
        dest_token: Address,
        amount: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<i128, CombinedAggregatorError>;

    /// Returns the expected return amount for a given input amount and distribution
    // fn getExpectedReturn(
    //     e: Env,
    //     from_token: Address,
    //     dest_token: Address,
    //     amount: i128,
    //     parts: i128,
    // ) -> Result<i128, CombinedAggregatorError>;

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedAggregatorError>;
    fn get_protocols(e: &Env) -> Result<Vec<ProxyAddressPair>, CombinedAggregatorError>;
    fn is_protocol_paused(
        e: &Env,
        protocol_id: String,
    ) -> Result<bool, CombinedAggregatorError>;

}

#[contract]
struct SoroswapAggregator;

#[contractimpl]
impl SoroswapAggregatorTrait for SoroswapAggregator {
    /// Initializes the contract and sets the soroswap_router address
    fn initialize(
        e: Env,
        admin: Address,
        proxy_addresses: Vec<ProxyAddressPair>,
    ) -> Result<(), CombinedAggregatorError> {
        if is_initialized(&e) {
            return Err(CombinedAggregatorError::AggregatorInitializeAlreadyInitialized.into());
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
    ) -> Result<(), CombinedAggregatorError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        // Check if the sender is the admin
        
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
    ) -> Result<(), CombinedAggregatorError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        
        remove_proxy_address(&e, protocol_id.clone());
    
        event::protocol_removed(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn pause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), CombinedAggregatorError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        
        //Should pause proxy contract
    
        event::protocol_paused(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn unpause_protocol(
        e: Env,
        protocol_id: String,
    ) -> Result<(), CombinedAggregatorError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        
        //Should unpause proxy contract
    
        event::protocol_unpaused(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn swap(
        e: Env,
        _from_token: Address,
        _dest_token: Address,
        amount: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<i128, CombinedAggregatorError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        check_nonnegative_amount(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        if distribution.len() > MAX_DISTRIBUTION_LENGTH {
            return Err(CombinedAggregatorError::AggregatorDistributionLengthExceeded);
        }
    
        let total_parts: i128 = distribution.iter().map(|dist| dist.parts).sum();    

        if total_parts == 0 {
            return Err(CombinedAggregatorError::AggregatorInvalidTotalParts);
        }

        let total_swapped: i128 = 0;
       
        // for dist in distribution.iter() {
        //     let swap_amount = amount.checked_mul(dist.parts)
        //         .and_then(|prod| prod.checked_div(total_parts))
        //         .ok_or(CombinedAggregatorError::AggregatorArithmeticError)?;
            
        //     match dist.index {
        //         dex_constants::SOROSWAP => {
        //             // Call function to handle swap via Soroswap
        //             let swap_result = soroswap_interface::swap_with_soroswap(&e, &swap_amount, dist.path.clone(), to.clone(), deadline.clone())?;
        //             total_swapped += swap_result;
        //         },
        //         dex_constants::PHOENIX => {
        //             // Call function to handle swap via Phoenix
        //             let swap_result = phoenix_interface::swap_with_phoenix(&e, &swap_amount, dist.path.clone(), to.clone(), deadline.clone())?;
        //             total_swapped += swap_result;
        //         },
        //         _ => return Err(CombinedAggregatorError::AggregatorUnsupportedProtocol),
        //     }
        // }

        event::swap(&e, amount, distribution, to);
        Ok(total_swapped)
    }

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedAggregatorError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_protocols(e: &Env) -> Result<Vec<ProxyAddressPair>, CombinedAggregatorError> {
        check_initialized(&e)?;

        let protocol_ids = get_protocol_ids(e);
        let mut addresses = Vec::new(e);
    
        // Iterate over each protocol ID and collect their proxy addresses
        for protocol_id in protocol_ids.iter() {
            if has_proxy_address(e, protocol_id.clone()) {
                let address = get_proxy_address(e, protocol_id.clone());
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
    ) -> Result<bool, CombinedAggregatorError> {
        //should get proxy status
        Ok(true)
    }

}
