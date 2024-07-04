#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

mod error;
mod event;
mod models;
mod proxy;
mod storage;
mod test;

use error::AggregatorError;
use models::{DexDistribution, Proxy, MAX_DISTRIBUTION_LENGTH};
use proxy::SoroswapAggregatorProxyClient;
use storage::{
    extend_instance_ttl, get_admin, get_protocol_ids, get_proxy, has_proxy,
    is_initialized, put_proxy, remove_proxy, set_admin,
    set_initialized, set_pause_protocol,
};

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
    fn initialize(
        e: Env,
        admin: Address,
        proxy_vec: Vec<Proxy>,
    ) -> Result<(), AggregatorError>;

    /// Updates the protocol addresses for the aggregator
    fn update_proxies(
        e: Env,
        proxy_vec: Vec<Proxy>,
    ) -> Result<(), AggregatorError>;

    /// Removes the protocol from the aggregator
    fn remove_proxy(e: Env, protocol_id: String) -> Result<(), AggregatorError>;

    /// Sets the `admin` address.
    ///
    /// # Argumentsnts
    /// * `e` - The runtime environment.t.
    /// * `protocol_id` - The ID of the protocol to set the paused state for.
    /// * `paused` - The boolean value indicating whether the protocol should be paused or not.
    ///
    /// # Returns
    /// Returns `Ok(())` if the operation is successful, otherwise returns an `AggregatorError`.
    fn set_pause(e: Env, protocol_id: String, paused: bool) -> Result<(), AggregatorError>;

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
    fn get_proxies(e: &Env) -> Result<Vec<Proxy>, AggregatorError>;
    fn get_paused(e: &Env, protocol_id: String) -> Result<bool, AggregatorError>;
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
        proxy_vec: Vec<Proxy>,
    ) -> Result<(), AggregatorError> {
        if is_initialized(&e) {
            return Err(AggregatorError::AlreadyInitialized);
        }

        for proxy in proxy_vec.iter() {
            put_proxy(&e, proxy);
        }

        set_admin(&e, admin.clone());

        // Mark the contract as initialized
        set_initialized(&e);
        event::initialized(&e, admin, proxy_vec);
        extend_instance_ttl(&e);
        Ok(())
    }

    /// this overwriotes the previous protocol address pair if existed
    fn update_proxies(
        e: Env,
        proxy_vec: Vec<Proxy>,
    ) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        for proxy in proxy_vec.iter() {
            put_proxy(&e, proxy);
        }

        event::protocols_updated(&e, proxy_vec);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn remove_proxy(e: Env, protocol_id: String) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        remove_proxy(&e, protocol_id.clone());

        event::protocol_removed(&e, protocol_id);
        extend_instance_ttl(&e);
        Ok(())
    }

    /// Sets the paused state of the protocol in the aggregator.
    ///
    /// # Argumentsnts
    /// * `e` - The runtime environment.t.
    /// * `protocol_id` - The ID of the protocol to set the paused state for.
    /// * `paused` - The boolean value indicating whether the protocol should be paused or not.
    ///
    /// # Returns
    /// Returns `Ok(())` if the operation is successful, otherwise returns an `AggregatorError`.
    fn set_pause(e: Env, protocol_id: String, paused: bool) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        set_pause_protocol(&e, protocol_id.clone(), paused)?;

        event::protocol_paused(&e, protocol_id, paused);
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
                amount
                    .checked_sub(total_swapped)
                    .ok_or(AggregatorError::ArithmeticError)?
            } else {
                // Calculate part of the total amount based on distribution parts
                amount
                    .checked_mul(dist.parts)
                    .and_then(|prod| prod.checked_div(total_parts))
                    .ok_or(AggregatorError::ArithmeticError)?
            };

            let proxy = get_proxy(&e, dist.protocol_id.clone())?;
            // if paused return error
            if proxy.paused {
                return Err(AggregatorError::ProtocolPaused );
            }
            let proxy_client = SoroswapAggregatorProxyClient::new(&e, &proxy.address);
            let response = proxy_client.swap(
                &to,
                &dist.path,
                &swap_amount,
                &amount_out_min,
                &deadline,
                &dist.is_exact_in,
            );

            // Store the response from the swap
            for item in response.iter() {
                swap_responses.push_back(item);
            }
            total_swapped = total_swapped
                .checked_add(swap_amount)
                .ok_or(AggregatorError::ArithmeticError)?;
        }

        event::swap(&e, amount, distribution, to);
        Ok(swap_responses)
    }

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, AggregatorError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_proxies(e: &Env) -> Result<Vec<Proxy>, AggregatorError> {
        check_initialized(&e)?;

        let protocol_ids = get_protocol_ids(e);
        let mut proxy_vec = Vec::new(e);

        // Iterate over each protocol ID and collect their proxy object
        for protocol_id in protocol_ids.iter() {
            if has_proxy(e, protocol_id.clone()) {
                let proxy = get_proxy(e, protocol_id.clone())?;
                let address = 
                proxy_vec.push_back(proxy);
            }
        }

        Ok(proxy_vec)
    }

    fn get_paused(e: &Env, protocol_id: String) -> Result<bool, AggregatorError> {
        let proxy = get_proxy(e, protocol_id)?;
        Ok(proxy.paused)
    }

    /// this is the firs version of the contract   
    fn get_version() -> u32 {
        1
    }
}
