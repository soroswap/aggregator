#![cfg(test)]
use comet_adapter::CometAdapterClient;
use pair::CometPairClient;
use soroban_sdk::{vec, Address, BytesN, Env, IntoVal, String, Symbol, Val, Vec};
use test_utils::phoenix_setup::{generate_salt, DeployerClient};

pub mod pair {
    soroban_sdk::contractimport!(file = "../aggregator/comet_contracts/comet_pool.wasm");
    pub type CometPairClient<'a> = Client<'a>;
}

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(pair::WASM)
}

pub mod factory {
    soroban_sdk::contractimport!(file = "../aggregator/comet_contracts/comet_factory.wasm");
    pub type CometFactoryClient<'a> = Client<'a>;
}
use factory::CometFactoryClient;

pub mod comet_adapter{
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/comet_adapter.optimized.wasm");
    pub type CometAdapterClient<'a> = Client<'a>;
}

pub fn create_comet_factory<'a>(e: &Env) -> CometFactoryClient {
    let pair_hash = pair_contract_wasm(e);

    let factory_address = e.register(factory::WASM, ());
    let factory_client = CometFactoryClient::new(&e.clone(), &factory_address);

    factory_client.init(&pair_hash);

    factory_client
}

pub fn deploy_and_init_comet_pool<'a>(e: &Env, admin: &Address, tokens: &Vec<Address>, factory_client: CometFactoryClient) -> CometPairClient<'a> {
    let comet_address = factory_client.new_c_pool(
        &BytesN::from_array(&e, &[0; 32]),
        &admin,
        tokens,
        &vec![e, 8000000, 2000000],
        &vec![e, 800000000000, 200000000000], // these balances make the tokens have an equal value
        &30000,
    );

    CometPairClient::new(e, &comet_address)
}

pub fn create_comet_adapter<'a>(e: &Env, deployer_client: &DeployerClient<'a>, pair_address: Address, admin: Address) -> CometAdapterClient<'a> {
    let wasm_hash = e.deployer().upload_contract_wasm(comet_adapter::WASM);

    // Deploy contract using deployer, and include an init function to call
    let salt = BytesN::from_array(&e, &generate_salt(3));
    let init_fn = Symbol::new(&e, &("initialize"));

    let protocol_id = String::from_str(&e, "comet");
    let protocol_address = pair_address.clone();

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(e);

    let (contract_id, _init_result) = deployer_client.deploy(
        &admin,
        &wasm_hash,
        &salt,
        &init_fn,
        &init_fn_args,
    );

    let adapter_contract = CometAdapterClient::new(e, &contract_id);
    adapter_contract
}
