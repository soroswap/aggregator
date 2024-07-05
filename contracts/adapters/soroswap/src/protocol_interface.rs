use soroban_sdk::{Env, Address, Vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use soroswap_aggregator_adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./contracts/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>,
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AdapterError> {

    if !has_protocol_address(&e) {
        return Err(AdapterError::ProtocolAddressNotFound);
    }
    
    let soroswap_router_address = get_protocol_address(&e);
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
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AdapterError> {
    if !has_protocol_address(&e) {
        return Err(AdapterError::ProtocolAddressNotFound);
    }
    
    let soroswap_router_address = get_protocol_address(&e);
    let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

    Ok(soroswap_router_client.swap_tokens_for_exact_tokens(
        &amount_out,
        &amount_in_max,
        &path,
        &to,
        &deadline
    ))
}
