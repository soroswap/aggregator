#![cfg(test)]
extern crate std;
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

// SoroswapAggregatorProxy Contract
fn create_soroswap_aggregator_proxy<'a>(e: &Env) -> SoroswapAggregatorProxyClient<'a> {
    SoroswapAggregatorProxyClient::new(e, &e.register_contract(None, SoroswapAggregatorProxy {}))
}

pub struct SoroswapAggregatorProxyTest<'a> {
    env: Env,
    proxy_contract: SoroswapAggregatorProxyClient<'a>,
}

impl<'a> SoroswapAggregatorProxyTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        
        let proxy_contract = create_soroswap_aggregator_proxy(&env);

        env.budget().reset_unlimited();

        SoroswapAggregatorProxyTest {
            env,
            proxy_contract,
        }
    }
}

// pub mod initialize;
// pub mod swap;
// pub mod update_protocols;
// pub mod get_protocols;