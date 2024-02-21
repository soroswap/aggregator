use soroban_sdk::{contracttype, Env, Address};
use crate::models::{ProtocolAddressPair, Protocol};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    ProtocolAddress(i32),
    Initialized,
    Admin,
    Protocol(u32),
    TotalProtocols
}

// TTL
const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// INITIALIZED
pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

// TOTAL PROTOCOLS
pub fn get_total_protocols(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalProtocols).unwrap()
}
pub fn set_total_protocols(e: &Env, new_total_protocols: u32) {
    e.storage().instance().set(&DataKey::TotalProtocols, &new_total_protocols);
}

// PROTOCOLS // TODO: Analize if we can do better in a vector
pub fn set_protocol(e: &Env, protocol: Protocol, protocol_index: u32) {
    e.storage().instance().set(&DataKey::Protocol(protocol_index), &protocol);
}
pub fn get_protocol(e: &Env, protocol_index: u32) -> Protocol {
    e.storage().instance().get(&DataKey::Protocol(protocol_index)).unwrap()
}
pub fn exist_protocol(e: &Env, protocol_index: u32) -> bool {
    e.storage().instance().has(&DataKey::Protocol(protocol_index))
}
pub fn push_protocol(e: &Env, protocol: Protocol) {
    // TODO: Check if protocol with same name? or aggregator address already exist
    let next_index = get_total_protocols(&e);
    set_protocol(&e, protocol, next_index.clone());
    set_total_protocols(&e, next_index.checked_add(1).unwrap());
}
pub fn desactivate_protocol(e: &Env, protocol_index: u32) {
    let mut protocol = get_protocol(&e, protocol_index.clone());
    protocol.active = false;
    set_protocol(&e, protocol, protocol_index);
}


//

pub fn put_protocol_address(e: &Env, pair: ProtocolAddressPair) {
    e.storage().instance().set(&DataKey::ProtocolAddress(pair.protocol_id), &pair.address);
}

pub fn has_protocol_address(e: &Env, protocol_id: i32) -> bool {
    e.storage().instance().has(&DataKey::ProtocolAddress(protocol_id))
}

pub fn get_protocol_address(e: &Env, protocol_id: i32) -> Address {
    e.storage().instance().get(&DataKey::ProtocolAddress(protocol_id)).unwrap()
}

pub fn set_admin(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Admin, &address)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}