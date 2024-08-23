use crate::{error::AggregatorError, models::Adapter};
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    ProtocolList,
    Adapter(String),
    Initialized,
    Admin,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/* INITIALIZE */
pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

/* ADMIN */
pub fn set_admin(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Admin, &address)
}

pub fn get_admin(e: &Env) -> Result<Address, AggregatorError> {
    match e.storage().instance().get(&DataKey::Admin) {
        Some(admin) => Ok(admin),
        None => Err(AggregatorError::NotInitialized),
    }
}

pub fn put_adapter(e: &Env, adapter: Adapter) {
    e.storage()
        .instance()
        .set(&DataKey::Adapter(adapter.protocol_id.clone()), &adapter);
    add_protocol_id(e, adapter.protocol_id);
}

pub fn has_adapter(e: &Env, protocol_id: String) -> bool {
    e.storage().instance().has(&DataKey::Adapter(protocol_id))
}

pub fn get_adapter(e: &Env, protocol_id: String) -> Result<Adapter, AggregatorError> {
    match e.storage().instance().get(&DataKey::Adapter(protocol_id)) {
        Some(adapter) => Ok(adapter),
        None => Err(AggregatorError::ProtocolNotFound),
    }
}

// TODO, THIS SHOULD FAIL IF PROXY DOES NOT EXIST
pub fn remove_adapter(e: &Env, protocol_id: String) {
    if has_adapter(e, protocol_id.clone()) {
        e.storage()
            .instance()
            .remove(&DataKey::Adapter(protocol_id.clone()));
        remove_protocol_id(e, protocol_id);
    }
}

pub fn add_protocol_id(e: &Env, protocol_id: String) {
    let mut protocols = get_protocol_ids(e);
    if !protocols.contains(&protocol_id) {
        protocols.push_back(protocol_id);
        e.storage()
            .instance()
            .set(&DataKey::ProtocolList, &protocols);
    }
}

pub fn get_protocol_ids(e: &Env) -> Vec<String> {
    match e.storage().instance().get(&DataKey::ProtocolList) {
        Some(protocol_ids) => protocol_ids,
        None => Vec::new(e),
    }
}

pub fn remove_protocol_id(e: &Env, protocol_id: String) {
    let protocols = get_protocol_ids(e);
    let mut new_protocols = Vec::new(e);

    for existing_id in protocols.iter() {
        if existing_id != protocol_id {
            new_protocols.push_back(existing_id.clone());
        }
    }

    e.storage()
        .instance()
        .set(&DataKey::ProtocolList, &new_protocols);
}

pub fn set_pause_protocol(
    e: &Env,
    protocol_id: String,
    paused: bool,
) -> Result<(), AggregatorError> {
    let mut protocol = get_adapter(&e, protocol_id)?;
    protocol.paused = paused;
    put_adapter(&e, protocol);
    Ok(())
}
