#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use soroban_sdk::token::Client as TokenClient;


mod error;
mod event;
mod models;
mod storage;
mod test;

use error::AggregatorError;
use models::{DexDistribution, Adapter, MAX_DISTRIBUTION_LENGTH};
use storage::{
    extend_instance_ttl, get_admin, get_protocol_ids, get_adapter, has_adapter,
    is_initialized, put_adapter, remove_adapter, set_admin,
    set_initialized, set_pause_protocol,
};
use soroswap_aggregator_adapter_interface::SoroswapAggregatorAdapterClient;

pub enum SwapType {
    ExactTokensForTokens,
    TokensForExactTokens,
}

//fn swap_exact_tokens_for_tokens(
    // env: Env,
    // amount_in: i128,
    // amount_out_min: i128,
    // path: Vec<Address>,
    // to: Address,
    // deadline: u64,
// create an object with all these fields
pub struct SwapExactTokensForTokens {
    amount_in: i128,
    amount_out_min: i128,
    path : Vec<Address>,
    to: Address,
    deadline: u64}
pub struct SwapTokensForExactTokens {
    amount_out: i128,
    amount_in_max: i128,
    path: Vec<Address>,
    to: Address,
    deadline: u64}
// creata a function swap that receives either a SwapExactTokensForTokens or a SwapTokensForExactTokens
// and returns a Result<Vec<i128>, AggregatorError>
pub fn swap(
    env: Env,
    swap: SwapExactTokensForTokens,
    adapter_client: SoroswapAggregatorAdapterClient,
) -> Result<Vec<i128>, AggregatorError> {
    let mut swap_responses: Vec<i128> = Vec::new(&env);


    // let response = adapter_client.swap(
    //     &swap.to,
    //     &swap.path,
    //     &swap.amount_in,
    //     &swap.amount_out_min,
    //     &swap.deadline,
    //     &true,
    // )?;

    Ok(swap_responses)
}

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

fn check_parameters(
    e: &Env,
    amount_0: i128,
    amount_1: i128,
    to: Address,
    deadline: u64,
    distribution: Vec<DexDistribution>,
    ) -> Result<(), AggregatorError> {

    check_initialized(e)?;
    check_nonnegative_amount(amount_0)?;
    check_nonnegative_amount(amount_1)?;
    to.require_auth();
    ensure_deadline(e, deadline)?;

    if distribution.len() > MAX_DISTRIBUTION_LENGTH {
        return Err(AggregatorError::DistributionLengthExceeded);
    }
    for dist in distribution {
        if dist.parts  == 0 {
                return Err(AggregatorError::ZeroDistributionPart);
        }
    }

    Ok(())
}


fn calculate_distribution_amounts(
    env: &Env,
    total_amount: i128,
    distribution: &Vec<DexDistribution>,
) -> Result<Vec<i128>, AggregatorError> {

    let total_parts: u32 = distribution.iter().map(|dist| dist.parts).sum();
    let total_parts: i128 = total_parts.into();
    let mut total_swapped = 0;
    let mut swap_amounts = soroban_sdk::Vec::new(env);

    for (index, dist) in distribution.iter().enumerate() {
        let swap_amount = if index == (distribution.len() - 1) as usize {
            total_amount
                .checked_sub(total_swapped)
                .ok_or(AggregatorError::ArithmeticError)?
        } else {
            let amount = total_amount
                .checked_mul(dist.parts.into())
                .and_then(|prod| prod.checked_div(total_parts))
                .ok_or(AggregatorError::ArithmeticError)?;
            total_swapped += amount;
            amount
        };

        swap_amounts.push_back(swap_amount);
    }

    Ok(swap_amounts)
}

pub fn get_adapter_client(
    e: &Env,
    protocol_id: String
) -> Result<SoroswapAggregatorAdapterClient, AggregatorError> {
    
    let adapter = get_adapter(&e, protocol_id.clone())?;
    if adapter.paused {
        return Err(AggregatorError::ProtocolPaused);
    }
    Ok(SoroswapAggregatorAdapterClient::new(&e, &adapter.address))
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
        adapter_vec: Vec<Adapter>,
    ) -> Result<(), AggregatorError>;

    /// Updates the protocol addresses for the aggregator
    fn update_adapters(
        e: Env,
        adapter_vec: Vec<Adapter>,
    ) -> Result<(), AggregatorError>;

    /// Removes the protocol from the aggregator
    fn remove_adapter(e: Env, protocol_id: String) -> Result<(), AggregatorError>;

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

    /// Swaps an exact amount of input tokens for as many output tokens as possible
    /// along the specified trading route. The route is determined by the `path` vector,
    /// where the first element is the input token, the last is the output token, 
    /// and any intermediate elements represent pairs to trade through if a direct pair does not exist.
    /// # Arguments
    /// * `amount_in` - The exact amount of input tokens to be swapped.
    /// * `amount_out_min` - The minimum required amount of output tokens to receive.
    /// * `path` - A vector representing the trading route, where the first element is the input token 
    ///         and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    /// # Returns
    /// A vector containing the amounts of tokens received at each step of the trading route.
    fn swap_exact_tokens_for_tokens(
        env: Env,
        token_in: Address,
        token_out: Address,
        amount_in: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError>;


    /// Swaps tokens for an exact amount of output token, following the specified trading route.
    /// The route is determined by the `path` vector, where the first element is the input token,
    /// the last is the output token, and any intermediate elements represent pairs to trade through.
    
    /// # Arguments
    /// * `amount_out` - The exact amount of output token to be received.
    /// * `amount_in_max` - The maximum allowed amount of input tokens to be swapped.
    /// * `path` - A vector representing the trading route, where the first element is the input token 
    ///         and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    
    /// # Returns
    /// A vector containing the amounts of tokens used at each step of the trading route.
    fn swap_tokens_for_exact_tokens(
        e: Env,
        token_in: Address,
        token_out: Address,
        amount_out: i128,
        amount_in_max: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError>;

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, AggregatorError>;
    fn get_adapters(e: &Env) -> Result<Vec<Adapter>, AggregatorError>;
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
        adapter_vec: Vec<Adapter>,
    ) -> Result<(), AggregatorError> {
        if is_initialized(&e) {
            return Err(AggregatorError::AlreadyInitialized);
        }

        for adapter in adapter_vec.iter() {
            put_adapter(&e, adapter);
        }

        set_admin(&e, admin.clone());

        // Mark the contract as initialized
        set_initialized(&e);
        event::initialized(&e, admin, adapter_vec);
        extend_instance_ttl(&e);
        Ok(())
    }

    /// this overwriotes the previous protocol address pair if existed
    fn update_adapters(
        e: Env,
        adapter_vec: Vec<Adapter>,
    ) -> Result<(), AggregatorError> {

        
        check_initialized(&e)?;
        check_admin(&e);

        for adapter in adapter_vec.iter() {
            put_adapter(&e, adapter);
        }

        event::protocols_updated(&e, adapter_vec);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn remove_adapter(e: Env, protocol_id: String) -> Result<(), AggregatorError> {
        check_initialized(&e)?;
        check_admin(&e);

        remove_adapter(&e, protocol_id.clone());

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

    fn swap_exact_tokens_for_tokens(
        e: Env,
        token_in: Address,
        token_out: Address,
        amount_in: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError> {
        
        extend_instance_ttl(&e);        
        check_parameters(&e, amount_in, amount_out_min, to.clone(), deadline, distribution.clone())?;
        
        let swap_amounts = calculate_distribution_amounts(&e, amount_in, &distribution)?;
        let mut swap_responses: Vec<i128> = Vec::new(&e);

        // Check initial out balance
        let initial_token_out_balance = TokenClient::new(&e, &token_out).balance(&to);

        for (index, swap_amount) in swap_amounts.iter().enumerate() {
            let dist = distribution.get(index as u32).unwrap();
            let protocol_id = dist.protocol_id;
            
            let adapter_client = get_adapter_client(&e, protocol_id.clone())?;
            
            // Perform the swap and handle the response (not shown)
            let response = adapter_client.swap_exact_tokens_for_tokens(
                &swap_amount, // amount_in
                &0, // amount_out_min: amount out min per protocol will allways be 0, we will then compare the toal amoiunt out
                &dist.path,
                &to,
                &deadline,
            );

            // TODO: handle response, maybe store?
            //     for item in response.iter() {
            //         swap_responses.push_back(item);
            //     }
        }
        // Check final token out balance
        let final_token_out_balance = TokenClient::new(&e, &token_out).balance(&to);
        if final_token_out_balance.checked_sub(initial_token_out_balance).ok_or(AggregatorError::ArithmeticError)? 
            < amount_out_min {
            return Err(AggregatorError::InsufficientOutputAmount);
        }

        // event::swap(&e, amount, distribution, to); // MAYBE NOT NEEDED IF ADAPTERS RESPOND WITH AMOUNTS

        // // TODO check amount out min
        // event::swap(&e, amount, distribution, to);
        Ok(swap_responses)
    }

    fn swap_tokens_for_exact_tokens(
        e: Env,
        token_in: Address,
        token_out: Address,
        amount_out: i128,
        amount_in_max: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AggregatorError>{
        
        extend_instance_ttl(&e);
        check_parameters(&e, amount_out, amount_in_max, to.clone(), deadline, distribution.clone())?;
        
        let swap_amounts = calculate_distribution_amounts(&e, amount_out, &distribution)?;
        let mut swap_responses: Vec<i128> = Vec::new(&e);

        // TODO: check initial balances
        for (index, swap_amount) in swap_amounts.iter().enumerate() {
            let dist = distribution.get(index as u32).unwrap();
            let protocol_id = dist.protocol_id;
            
            let adapter_client = get_adapter_client(&e, protocol_id.clone())?;
            
            let response = adapter_client.swap_tokens_for_exact_tokens(
                &swap_amount, // amount_out
                &i128::MAX, // amount_in_max
                &dist.path, //path
                &to,//to
                &deadline, //deadline
            );

            // TODO: handle response, maybe store?
            //     for item in response.iter() {
            //         swap_responses.push_back(item);
            //     }

        }
        // TODO check FINAL BALANCES AND CHECK FOR amount_in_max
        // event::swap(&e, amount, distribution, to);
        Ok(swap_responses)
    }

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, AggregatorError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_adapters(e: &Env) -> Result<Vec<Adapter>, AggregatorError> {
        check_initialized(&e)?;

        let protocol_ids = get_protocol_ids(e);
        let mut adapter_vec = Vec::new(e);

        // Iterate over each protocol ID and collect their adapter object
        for protocol_id in protocol_ids.iter() {
            if has_adapter(e, protocol_id.clone()) {
                let adapter = get_adapter(e, protocol_id.clone())?;
                adapter_vec.push_back(adapter);
            }
        }

        Ok(adapter_vec)
    }

    fn get_paused(e: &Env, protocol_id: String) -> Result<bool, AggregatorError> {
        let adapter = get_adapter(e, protocol_id)?;
        Ok(adapter.paused)
    }

    /// this is the firs version of the contract   
    fn get_version() -> u32 {
        1
    }
}
