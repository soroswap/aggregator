#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    Address, 
    BytesN,
    Symbol,
    String,
    Vec,
    Val,
    IntoVal
};
use crate::{SoroswapAggregatorAdapter, SoroswapAggregatorAdapterClient};
use soroswap_setup::{SoroswapTest, router, factory, token::TokenClient};
use factory::SoroswapFactoryClient;
use router::SoroswapRouterClient;

mod deployer_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/soroswap_aggregator_deployer.optimized.wasm");
    pub type DeployerClient<'a> = Client<'a>;
}
use deployer_contract::DeployerClient;

fn create_deployer<'a>(e: &Env) -> DeployerClient<'a> {
    let deployer_address = &e.register(deployer_contract::WASM, ());
    let deployer = DeployerClient::new(e, deployer_address);
    deployer
}

// SoroswapAggregatorAdapter Contract
fn create_soroswap_aggregator_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterClient<'a> {
    SoroswapAggregatorAdapterClient::new(e, &e.register(SoroswapAggregatorAdapter {}, ()))
}

pub mod soroswap_adapter_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/soroswap_adapter.optimized.wasm");
    pub type SoroswapAggregatorAdapterClientFromWasm<'a> = Client<'a>;
}
use soroswap_adapter_contract::SoroswapAggregatorAdapterClientFromWasm;


pub struct SoroswapAggregatorAdapterTest<'a> {
    env: Env,
    adapter_contract: SoroswapAggregatorAdapterClientFromWasm<'a>,
    adapter_contract_not_initialized: SoroswapAggregatorAdapterClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    factory_contract: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    user: Address,
    // admin: Address
}

impl<'a> SoroswapAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = SoroswapTest::soroswap_setup();
        
        let wasm_hash = test.env.deployer().upload_contract_wasm(soroswap_adapter_contract::WASM);
        let deployer_client = create_deployer(&test.env);
        
        let adapter_contract_not_initialized = create_soroswap_aggregator_adapter(&test.env);

        // Deploy contract using deployer, and include an init function to call.
        let salt = BytesN::from_array(&test.env, &[0; 32]);
        let init_fn = Symbol::new(&test.env, &("initialize"));

        let protocol_id = String::from_str(&test.env, "soroswap");
        let protocol_address = test.router_contract.address.clone();

        // Convert the arguments into a Vec<Val>
        let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&test.env);

        test.env.mock_all_auths();
        let (contract_id, _init_result) = deployer_client.deploy(
            &deployer_client.address,
            &wasm_hash,
            &salt,
            &init_fn,
            &init_fn_args,
        );

        let adapter_contract = soroswap_adapter_contract::Client::new(&test.env, &contract_id);


        SoroswapAggregatorAdapterTest {
            env: test.env,
            adapter_contract,
            adapter_contract_not_initialized,
            router_contract: test.router_contract,
            factory_contract: test.factory_contract,
            token_0: test.token_0,
            token_1: test.token_1,
            token_2: test.token_2,
            user: test.user,
            // admin: test.admin
        }
    }
}

pub mod initialize;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;