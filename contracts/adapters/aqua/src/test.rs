#![cfg(test)]
extern crate std;
pub mod aqua_setup;

use soroban_sdk::testutils::Address as _;

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
use crate::{SoroswapAggregatorAquaAdapter, SoroswapAggregatorAquaAdapterClient};
use aqua_setup::{AquaTest, TokenClient, AquaRouter};

mod deployer_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/soroswap_aggregator_deployer.optimized.wasm");
    pub type DeployerClient<'a> = Client<'a>;
}
use deployer_contract::DeployerClient;

fn create_deployer<'a>(e: &Env) -> DeployerClient<'a> {
    let deployer_address = &e.register(deployer_contract::WASM, ());
    let deployer = DeployerClient::new(e, deployer_address);
    deployer
}

// AquaAggregatorAdapter Contract
fn create_soroswap_aggregator_aqua_adapter<'a>(e: &Env) -> SoroswapAggregatorAquaAdapterClient<'a> {
    SoroswapAggregatorAquaAdapterClient::new(e, &e.register(SoroswapAggregatorAquaAdapter, {}))
}

pub mod aqua_adapter_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/aqua_adapter.optimized.wasm");
    pub type SoroswapAggregatorAquaAdapterClientFromWasm<'a> = Client<'a>;
}
use aqua_adapter_contract::SoroswapAggregatorAquaAdapterClientFromWasm;

pub struct AquaAggregatorAdapterTest<'a> {
    env: Env,
    adapter_client: SoroswapAggregatorAquaAdapterClientFromWasm<'a>,
    adapter_client_not_initialized: SoroswapAggregatorAquaAdapterClient<'a>,
    router: AquaRouter<'a>,
    tokens: [TokenClient<'a>; 4],
    user: Address,
    admin: Address,
    reward_token: TokenClient<'a>,
}

impl<'a> AquaAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let test = AquaTest::aqua_setup();
        
        let wasm_hash = test.env.deployer().upload_contract_wasm(aqua_adapter_contract::WASM);
        let deployer_client = create_deployer(&test.env);

        let adapter_client_not_initialized = create_soroswap_aggregator_aqua_adapter(&test.env);
        // Deploy contract using deployer, and include an init function to call.
        let salt = BytesN::from_array(&test.env, &[0; 32]);
        let init_fn = Symbol::new(&test.env, &("initialize"));

        let protocol_id = String::from_str(&test.env, "aqua");
        let protocol_address = test.router.address.clone();

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

        let adapter_client = aqua_adapter_contract::Client::new(&test.env, &contract_id);

        let user = Address::generate(&test.env);

        AquaAggregatorAdapterTest {
            env: test.env,
            adapter_client,
            adapter_client_not_initialized,
            router: test.router,
            tokens: test.tokens,
            user,
            admin: test.admin,
            reward_token: test.reward_token,
        }
    }
}

pub mod initialize;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;