//! Definition of the Events used in the contract
use crate::models::{DexDistribution, Adapter};
use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Vec};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub admin: Address,
    pub adapter_addresses: Vec<Adapter>,
}

pub(crate) fn initialized(e: &Env, admin: Address, adapter_addresses: Vec<Adapter>) {
    let event: InitializedEvent = InitializedEvent {
        admin: admin,
        adapter_addresses,
    };
    e.events()
        .publish(("SoroswapAggregator", symbol_short!("init")), event);
}


// UPDATE PROTOCOL EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProtocolsEvent {
    pub adapter_addresses: Vec<Adapter>,
}

/// Publishes an `UpdateProtocolsEvent` to the event stream.
pub(crate) fn protocols_updated(e: &Env, adapter_addresses: Vec<Adapter>) {
    let event = UpdateProtocolsEvent { adapter_addresses };

    e.events()
        .publish(("SoroswapAggregator", symbol_short!("update")), event);
}

// REMOVE PROTOCOL EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemovedProtocolEvent {
    pub protocol_id: String,
}

pub(crate) fn protocol_removed(e: &Env, protocol_id: String) {
    let event = RemovedProtocolEvent { protocol_id };

    e.events()
        .publish(("SoroswapAggregator", symbol_short!("removed")), event);
}

// PAUSE/UNPAUSE PROTOCOL EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PausedProtocolEvent {
    pub protocol_id: String,
    pub paused: bool,
}

pub(crate) fn protocol_paused(e: &Env, protocol_id: String, paused: bool) {
    let event = PausedProtocolEvent { protocol_id, paused};
    e.events()
        .publish(("SoroswapAggregator", symbol_short!("paused")), event);
}






#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewAdminEvent {
    pub old: Address,
    pub new: Address,
}

pub(crate) fn new_admin(e: &Env, old: Address, new: Address) {
    let event: NewAdminEvent = NewAdminEvent { old: old, new: new };
    e.events()
        .publish(("SoroswapAggregator", symbol_short!("new_admin")), event);
}



// SWAP EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub amount_in: i128,
    pub distribution: Vec<DexDistribution>,
    pub to: Address,
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
pub(crate) fn swap(e: &Env, amount_in: i128, distribution: Vec<DexDistribution>, to: Address) {
    let event = SwapEvent {
        amount_in,
        distribution,
        to,
    };

    e.events()
        .publish(("SoroswapAggregator", symbol_short!("swap")), event);
}