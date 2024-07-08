#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    Address, 
    
};
use crate::{SoroswapAggregatorAdapter, SoroswapAggregatorAdapterClient};
use soroswap_setup::{SoroswapTest, router, factory, token::TokenClient};
use factory::SoroswapFactoryClient;
use router::SoroswapRouterClient;

// SoroswapAggregatorAdapter Contract
fn create_soroswap_aggregator_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterClient<'a> {
    SoroswapAggregatorAdapterClient::new(e, &e.register_contract(None, SoroswapAggregatorAdapter {}))
}

pub struct SoroswapAggregatorAdapterTest<'a> {
    env: Env,
    adapter_contract: SoroswapAggregatorAdapterClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    factory_contract: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    user: Address,
    admin: Address
}

impl<'a> SoroswapAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = SoroswapTest::soroswap_setup();
        
        let adapter_contract = create_soroswap_aggregator_adapter(&test.env);

        SoroswapAggregatorAdapterTest {
            env: test.env,
            adapter_contract,
            router_contract: test.router_contract,
            factory_contract: test.factory_contract,
            token_0: test.token_0,
            token_1: test.token_1,
            token_2: test.token_2,
            user: test.user,
            admin: test.admin
        }
    }
}

pub mod initialize;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;