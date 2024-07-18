use soroban_sdk::{
   Env, BytesN, Address,
};

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../adapters/soroswap/soroswap_contracts/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

pub fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroswap_router.optimized.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}
pub use router::SoroswapRouterClient;

// SoroswapRouter Contract
pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    let router_address = &e.register_contract_wasm(None, router::WASM);
    let router = SoroswapRouterClient::new(e, router_address);
    router
}


// SoroswapAggregatorAdapter Contract
// For Soroswap
mod soroswap_adapter {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm");
    pub type SoroswapAggregatorAdapterForSoroswapClient<'a> = Client<'a>;
}
pub use soroswap_adapter::SoroswapAggregatorAdapterForSoroswapClient;

// Adapter for Soroswap
pub fn create_soroswap_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterForSoroswapClient<'a> {
    let adapter_address = &e.register_contract_wasm(None, soroswap_adapter::WASM);
    let adapter = SoroswapAggregatorAdapterForSoroswapClient::new(e, adapter_address);
    adapter
}
