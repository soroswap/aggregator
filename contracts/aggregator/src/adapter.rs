soroban_sdk::contractimport!(
    file =
        "../adapters/soroswap/target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm"
);
pub type SoroswapAggregatorAdapterClient<'a> = Client<'a>;
