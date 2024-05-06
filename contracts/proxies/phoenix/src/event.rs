//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub state: bool,
    pub protocol_address: Address
}

pub(crate) fn initialized(e: &Env, state: bool, protocol_address: Address) {
    
    let event: InitializedEvent = InitializedEvent {
        state: state,
        protocol_address,
    };
    e.events().publish(("SoroswapAggregatorProxyForPhoenix", symbol_short!("init")), event);
}

// SWAP EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub amount_in: i128,
    pub path: Vec<Address>,
    pub to: Address
}

/// Publishes an `SwapEvent` to the event stream.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `path` - A vector representing the trading route, where the first element is the input token 
///            and the last is the output token. Intermediate elements represent pairs to trade through.
/// * `amounts` - A vector containing the amounts of tokens traded at each step of the trading route.
/// * `to` - The address where the output tokens will be sent to.
pub(crate) fn swap(
    e: &Env,
    amount_in: i128,
    path: Vec<Address>,
    to: Address
) {
    let event = SwapEvent {
        amount_in,
        path,
        to,
    };

    e.events().publish(("SoroswapAggregatorProxyForPhoenix", symbol_short!("swap")), event);
}
// UPDATE PROTOCOL EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProtocolEvent {
    pub protocol_address: Address
}

/// Publishes an `UpdateProtocolEvent` to the event stream.
pub(crate) fn protocol_updated(
    e: &Env,
    protocol_address: Address
) {
    let event = UpdateProtocolEvent {
        protocol_address,
    };

    e.events().publish(("SoroswapAggregatorProxyForPhoenix", symbol_short!("update")), event);
}

pub(crate) fn protocol_paused(
    e: &Env,
    status: bool
) {
    e.events().publish(("SoroswapAggregatorProxyForPhoenix", symbol_short!("paused")), status);
}