use soroban_sdk::{Env, Address, Vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use soroswap_aggregator_adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./contracts/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

pub fn protocol_swap(
    e: &Env,
    amount_in: &i128,
    amount_out_min_or_max: &i128,
    path: Vec<Address>,
    to: Address,
    deadline: u64,
    is_exact_in: bool,
) -> Result<Vec<i128>, AdapterError> {
    if !has_protocol_address(e) {
        return Err(AdapterError::ProtocolAddressNotFound);
    }
    
    let soroswap_router_address = get_protocol_address(e);
    let soroswap_router_client = SoroswapRouterClient::new(e, &soroswap_router_address);

    let result = if is_exact_in {
        /*
            Swaps an exact amount of input tokens for as many output tokens as possible
            along the specified trading route. The route is determined by the `path` vector,
            where the first element is the input token, the last is the output token, 
            and any intermediate elements represent pairs to trade through if a direct pair does not exist.
            # Arguments
            * `amount_in` - The exact amount of input tokens to be swapped.
            * `amount_out_min` - The minimum required amount of output tokens to receive.
            * `path` - A vector representing the trading route, where the first element is the input token 
                    and the last is the output token. Intermediate elements represent pairs to trade through.
            * `to` - The address where the output tokens will be sent to.
            * `deadline` - The deadline for executing the operation.
            # Returns
            A vector containing the amounts of tokens received at each step of the trading route.
            fn swap_exact_tokens_for_tokens(
                e: Env,
                amount_in: i128,
                amount_out_min: i128,
                path: Vec<Address>,
                to: Address,
                deadline: u64,
            ) -> Result<Vec<i128>, CombinedRouterError>;

        */
        soroswap_router_client.swap_exact_tokens_for_tokens(
            amount_in,
            amount_out_min_or_max,
            &path,
            &to,
            &deadline
        )
    } else {


        /*
            Swaps tokens for an exact amount of output token, following the specified trading route.
            The route is determined by the `path` vector, where the first element is the input token,
            the last is the output token, and any intermediate elements represent pairs to trade through.
            
            # Arguments
            * `amount_out` - The exact amount of output token to be received.
            * `amount_in_max` - The maximum allowed amount of input tokens to be swapped.
            * `path` - A vector representing the trading route, where the first element is the input token 
                    and the last is the output token. Intermediate elements represent pairs to trade through.
            * `to` - The address where the output tokens will be sent to.
            * `deadline` - The deadline for executing the operation.
            
            # Returns
            A vector containing the amounts of tokens used at each step of the trading route.
            fn swap_tokens_for_exact_tokens(
                e: Env,
                amount_out: i128,
                amount_in_max: i128,
                path: Vec<Address>,
                to: Address,
                deadline: u64,
            ) -> Result<Vec<i128>, CombinedRouterError> {
        */
        
        soroswap_router_client.swap_tokens_for_exact_tokens(
            amount_out_min_or_max,
            amount_in,
            &path,
            &to.clone(),
            &deadline
        )
    };

    Ok(result)
}


