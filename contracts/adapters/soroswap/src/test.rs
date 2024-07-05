#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    vec,
    Vec,
    BytesN, 
    Address, 
    String,
    testutils::{
        Address as _,
        Ledger,
    },
};
use crate::{SoroswapAggregatorAdapter, SoroswapAggregatorAdapterClient};
use soroswap_setup::{SoroswapTest, router};
use router::SoroswapRouterClient;

// SoroswapAggregatorAdapter Contract
fn create_soroswap_aggregator_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterClient<'a> {
    SoroswapAggregatorAdapterClient::new(e, &e.register_contract(None, SoroswapAggregatorAdapter {}))
}

pub struct SoroswapAggregatorAdapterTest<'a> {
    env: Env,
    adapter_contract: SoroswapAggregatorAdapterClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    admin: Address,
}

impl<'a> SoroswapAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = SoroswapTest::soroswap_setup();
        
        let adapter_contract = create_soroswap_aggregator_adapter(&test.env);
        let router_contract = test.router_contract;

        SoroswapAggregatorAdapterTest {
            env: test.env,
            adapter_contract,
            router_contract,
            admin: test.admin,
        }
    }
}

pub mod initialize;
// pub mod swap;
// pub mod update_adapters;
// pub mod get_adapters;