#![cfg(test)]
extern crate std;
pub mod phoenix_setup;

use soroban_sdk::{
    Env, 
    Address, 
    
};
use crate::{SoroswapAggregatorPhoenixAdapter, SoroswapAggregatorPhoenixAdapterClient};
use phoenix_setup::{PhoenixTest};
// use factory::SoroswapFactoryClient;
// use router::SoroswapRouterClient;

// PhoenixAggregatorAdapter Contract
fn create_soroswap_aggregator_adapter<'a>(e: &Env) -> SoroswapAggregatorPhoenixAdapterClient<'a> {
    SoroswapAggregatorPhoenixAdapterClient::new(e, &e.register_contract(None, SoroswapAggregatorPhoenixAdapter {}))
}

pub struct PhoenixAggregatorAdapterTest<'a> {
    env: Env,
    adapter_contract: SoroswapAggregatorPhoenixAdapterClient<'a>,
    // router_contract: SoroswapRouterClient<'a>,
    // factory_contract: SoroswapFactoryClient<'a>,
    // token_0: TokenClient<'a>,
    // token_1: TokenClient<'a>,
    // token_2: TokenClient<'a>,
    user: Address,
    admin: Address
}

impl<'a> PhoenixAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = PhoenixTest::phoenix_setup();
        
        let adapter_contract = create_soroswap_aggregator_adapter(&test.env);

        PhoenixAggregatorAdapterTest {
            env: test.env,
            adapter_contract,
            // router_contract: test.router_contract,
            // factory_contract: test.factory_contract,
            // token_0: test.token_0,
            // token_1: test.token_1,
            // token_2: test.token_2,
            user: test.user,
            admin: test.admin
        }
    }
}

// pub mod initialize;
// pub mod swap_exact_tokens_for_tokens;
// pub mod swap_tokens_for_exact_tokens;