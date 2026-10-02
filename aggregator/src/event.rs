```rust
use soroban_sdk::contractevent;
use soroban_sdk::Address;
use soroban_sdk::Symbol;
use crate::symbol_short;

/// SoroswapAggregator contract event: update
#[contractevent]
pub struct UpdateEvent {
    pub sender: Address,
    pub pool: Address,
    pub asset_a: Address,
    pub amount_a: u128,
    pub asset_b: Address,
    pub amount_b: u128,
}
