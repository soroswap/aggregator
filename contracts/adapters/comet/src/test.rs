#![cfg(test)]
extern crate std;
pub mod comet_setup;

use comet_adapter_contract::CometAggregatorAdapterClientFromWasm;
use comet_setup::{create_comet_factory, create_token_contract, pair::CometPairClient};
use soroban_sdk::{
    testutils::Address as _, token::TokenClient, vec, Address, BytesN, Env, IntoVal, String,
    Symbol, Val, Vec,
};

mod deployer_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/soroswap_aggregator_deployer.optimized.wasm");
    pub type DeployerClient<'a> = Client<'a>;
}
use deployer_contract::DeployerClient;

use crate::{CometAggregatorAdapter, CometAggregatorAdapterClient};

fn create_deployer<'a>(e: &Env) -> DeployerClient<'a> {
    let deployer_address = &e.register(deployer_contract::WASM, ());
    let deployer = DeployerClient::new(e, deployer_address);
    deployer
}

fn create_comet_aggregator_adapter<'a>(e: &Env) -> CometAggregatorAdapterClient<'a> {
    CometAggregatorAdapterClient::new(e, &e.register(CometAggregatorAdapter {}, ()))
}

pub mod comet_adapter_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/comet_adapter.optimized.wasm"
    );
    pub type CometAggregatorAdapterClientFromWasm<'a> = Client<'a>;
}

pub struct CometAggregatorAdapterTest<'a> {
    pub env: Env,
    pub adapter_contract: CometAggregatorAdapterClientFromWasm<'a>,
    pub adapter_contract_not_initialized: CometAggregatorAdapterClient<'a>,
    pub comet_contract: CometPairClient<'a>,
    // pub factory_contract: CometFactoryClient<'a>,
    pub token_0: TokenClient<'a>,
    pub token_1: TokenClient<'a>,
    pub user: Address,
    // pub admin: Address,
}

impl<'a> CometAggregatorAdapterTest<'a> {
    fn setup() -> Self {
        let e = Env::default();
        e.mock_all_auths();

        let admin = Address::generate(&e);
        let user = Address::generate(&e);

        let (token_a, admin_token_a) = create_token_contract(&e, &admin);
        let (token_b, admin_token_b) = create_token_contract(&e, &admin);

        admin_token_a.mint(&admin, &1000000_000_000_0);
        admin_token_b.mint(&admin, &1000000_000_000_0);

        admin_token_a.mint(&user, &1000000_000_000_0);
        admin_token_b.mint(&user, &1000000_000_000_0);

        let comet_factory_client = create_comet_factory(&e);
        let comet_address = comet_factory_client.new_c_pool(
            &BytesN::from_array(&e, &[0; 32]),
            &admin,
            &vec![&e, token_a.address.clone(), token_b.address.clone()],
            &vec![&e, 8000000, 2000000],
            &vec![&e, 800000000000, 200000000000], // these balances make the tokens have an equal value
            &30000,
        );
        let comet_client = CometPairClient::new(&e, &comet_address);

        let adapter_wasm_hash = e
            .deployer()
            .upload_contract_wasm(comet_adapter_contract::WASM);
        let deployer_client = create_deployer(&e);

        let adapter_contract_not_initialized = create_comet_aggregator_adapter(&e);

        // Deploy contract using deployer, and include an init function to call.
        let salt = BytesN::from_array(&e, &[0; 32]);
        let init_fn = Symbol::new(&e, &("initialize"));

        let protocol_id = String::from_str(&e, "comet_blend");
        let protocol_address = comet_address.clone();

        // Convert the arguments into a Vec<Val>
        let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&e);

        let (adapter_address, _init_result) = deployer_client.deploy(
            &deployer_client.address,
            &adapter_wasm_hash,
            &salt,
            &init_fn,
            &init_fn_args,
        );

        let adapter_contract = CometAggregatorAdapterClientFromWasm::new(&e, &adapter_address);

        CometAggregatorAdapterTest {
            env: e.clone(),
            adapter_contract: adapter_contract,
            adapter_contract_not_initialized: adapter_contract_not_initialized,
            comet_contract: comet_client,
            // factory_contract: comet_factory_client,
            token_0: token_a,
            token_1: token_b,
            user: user,
        }
    }
}

pub mod initialize;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;
