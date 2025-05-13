use soroban_sdk::{Env, Address, Vec};

use crate::error::AggregatorError;

soroban_sdk::contractimport!(
    file = "./soroswap_contracts/soroswap_router.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    soroswap_router_address: &Address,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>,
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AggregatorError> {

    let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

    Ok(soroswap_router_client.swap_exact_tokens_for_tokens(
        &amount_in,
        &amount_out_min,
        &path,
        &to,
        &deadline
    ))
}

pub fn protocol_swap_tokens_for_exact_tokens(
    e: &Env,
    soroswap_router_address: &Address,
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AggregatorError> {

    let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

    Ok(soroswap_router_client.swap_tokens_for_exact_tokens(
        &amount_out,
        &amount_in_max,
        &path,
        &to,
        &deadline
    ))
}
