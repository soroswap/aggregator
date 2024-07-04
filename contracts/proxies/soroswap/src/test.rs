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
use crate::{SoroswapAggregatorProxy, SoroswapAggregatorProxyClient};
use soroswap_setup::{SoroswapTest, router};
use router::SoroswapRouterClient;

// SoroswapAggregatorProxy Contract
fn create_soroswap_aggregator_proxy<'a>(e: &Env) -> SoroswapAggregatorProxyClient<'a> {
    SoroswapAggregatorProxyClient::new(e, &e.register_contract(None, SoroswapAggregatorProxy {}))
}

pub struct SoroswapAggregatorProxyTest<'a> {
    env: Env,
    proxy_contract: SoroswapAggregatorProxyClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    admin: Address,
}

impl<'a> SoroswapAggregatorProxyTest<'a> {
    fn setup() -> Self {
        let test = SoroswapTest::soroswap_setup();
        
        let proxy_contract = create_soroswap_aggregator_proxy(&test.env);
        let router_contract = test.router_contract;

        SoroswapAggregatorProxyTest {
            env: test.env,
            proxy_contract,
            router_contract,
            admin: test.admin,
        }
    }
}

pub mod initialize;
// pub mod swap;
// pub mod update_proxies;
// pub mod get_proxies;