#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    Address, 
    
};
use crate::{SoroswapAggregatorAdapter, SoroswapAggregatorAdapterClient};
use soroswap_setup::{SoroswapTest, router, token::TokenClient};
use router::SoroswapRouterClient;

// SoroswapAggregatorAdapter Contract
fn create_soroswap_aggregator_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterClient<'a> {
    SoroswapAggregatorAdapterClient::new(e, &e.register_contract(None, SoroswapAggregatorAdapter {}))
}

pub struct SoroswapAggregatorAdapterTest<'a> {
    env: Env,
    adapter_contract: SoroswapAggregatorAdapterClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
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
            token_0: test.token_0,
            token_1: test.token_1,
            token_2: test.token_2,
            user: test.user,
            admin: test.admin
        }
    }
}

pub mod initialize;
// pub mod swap;
// pub mod update_adapters;
// pub mod get_adapters;