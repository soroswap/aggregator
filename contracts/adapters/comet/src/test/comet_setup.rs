#![cfg(test)]
extern crate std;
use soroban_sdk::{
    token::{StellarAssetClient as TokenAdminClient, TokenClient}, Address, BytesN, Env
};


pub mod pair{
    soroban_sdk::contractimport!(file = "./comet_contracts/comet_pool.wasm");
       pub type CometPairClient<'a> = Client<'a>;
}

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(pair::WASM)
}

pub mod factory{
    soroban_sdk::contractimport!(file = "./comet_contracts/comet_factory.wasm");
    pub type CometFactoryClient<'a> = Client<'a>;
}
use factory::CometFactoryClient;

pub fn create_comet_factory<'a>(e: &Env) -> CometFactoryClient{
    let pair_hash = pair_contract_wasm(e);

    let factory_address = e.register(factory::WASM, ());
    let factory_client = CometFactoryClient::new(&e.clone(), &factory_address);

    factory_client.init(&pair_hash);

    factory_client
}

pub fn create_token_contract<'a>(e: &Env, admin: & Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let stellar_asset_contract = e.register_stellar_asset_contract_v2(admin.clone());
    
    (TokenClient::new(&e, &stellar_asset_contract.address()), TokenAdminClient::new(&e, &stellar_asset_contract.address()))
}