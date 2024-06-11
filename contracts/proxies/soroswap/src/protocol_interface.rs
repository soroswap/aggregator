use soroban_sdk::{Env, Address, Vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use soroswap_aggregator_proxy_interface::{ProxyError};

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
) -> Result<Vec<i128>, ProxyError> {
    if !has_protocol_address(e) {
        return Err(ProxyError::ProtocolAddressNotFound);
    }
    
    let soroswap_router_address = get_protocol_address(e);
    let soroswap_router_client = SoroswapRouterClient::new(e, &soroswap_router_address);

    let result = if is_exact_in {
        soroswap_router_client.swap_exact_tokens_for_tokens(
            amount_in,
            amount_out_min_or_max,
            &path,
            &to,
            &deadline
        )
    } else {
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

