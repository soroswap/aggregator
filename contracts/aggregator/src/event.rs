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
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: i128,
    pub amount_out: i128,
    pub distribution: Vec<DexDistribution>,
    pub to: Address,
}

pub(crate) fn swap(
    e: &Env,
    token_in: Address,
    token_out: Address,
    amount_in: i128,
    amount_out: i128,
    distribution: Vec<DexDistribution>, 
    to: Address) {
    let event = SwapEvent {
        token_in,
        token_out,
        amount_in,
        amount_out,
        distribution,
        to,
    };

    e.events()
        .publish(("SoroswapAggregator", symbol_short!("swap")), event);
}