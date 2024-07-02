soroban_sdk::contractimport!(
    file =
        "../proxies/soroswap/target/wasm32-unknown-unknown/release/soroswap_proxy.optimized.wasm"
);
pub type SoroswapAggregatorProxyClient<'a> = Client<'a>;
