#![cfg(test)]
extern crate std;
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
use crate::{SoroswapAggregatorPhoenixAdapter, SoroswapAggregatorPhoenixAdapterClient};
use test_utils::phoenix_setup::{PhoenixTest, MultihopClient, TokenClient, PhoenixFactory};
// use factory::SoroswapFactoryClient;
// use router::SoroswapRouterClient;

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

// PhoenixAggregatorAdapter Contract
fn create_soroswap_aggregator_phoenix_adapter<'a>(e: &Env) -> SoroswapAggregatorPhoenixAdapterClient<'a> {
    SoroswapAggregatorPhoenixAdapterClient::new(e, &e.register(SoroswapAggregatorPhoenixAdapter {}, ()))
}

pub mod phoenix_adapter_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/phoenix_adapter.optimized.wasm");
    pub type SoroswapAggregatorPhoenixAdapterClientFromWasm<'a> = Client<'a>;
}
use phoenix_adapter_contract::SoroswapAggregatorPhoenixAdapterClientFromWasm;

pub struct PhoenixAggregatorAdapterTest<'a> {
    env: Env,
    adapter_client: SoroswapAggregatorPhoenixAdapterClientFromWasm<'a>,
    adapter_client_not_initialized: SoroswapAggregatorPhoenixAdapterClient<'a>,
    factory_client: PhoenixFactory<'a>,
    multihop_client: MultihopClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    token_3: TokenClient<'a>,
    user: Address,
    admin: Address
}

impl<'a> PhoenixAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = PhoenixTest::phoenix_setup();
        
        let wasm_hash = test.env.deployer().upload_contract_wasm(phoenix_adapter_contract::WASM);
        let deployer_client = create_deployer(&test.env);

        let adapter_client_not_initialized = create_soroswap_aggregator_phoenix_adapter(&test.env);
        // Deploy contract using deployer, and include an init function to call.
        let salt = BytesN::from_array(&test.env, &[0; 32]);
        let init_fn = Symbol::new(&test.env, &("initialize"));

        let protocol_id = String::from_str(&test.env, "phoenix");
        let protocol_address = test.multihop_client.address.clone();

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

        let adapter_client = phoenix_adapter_contract::Client::new(&test.env, &contract_id);

        PhoenixAggregatorAdapterTest {
            env: test.env,
            adapter_client,
            adapter_client_not_initialized,
            factory_client: test.factory_client,
            multihop_client: test.multihop_client,
            token_0: test.token_0,
            token_1: test.token_1,
            token_2: test.token_2,
            token_3: test.token_3,
            user: test.user,
            admin: test.admin
        }
    }
}

pub mod initialize;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;