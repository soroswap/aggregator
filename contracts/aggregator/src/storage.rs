use crate::{error::AggregatorError, models::Proxy};
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Proxy(String),
    ProtocolPaused(String),
    Initialized,
    Admin,
    ProtocolList,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn set_admin(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Admin, &address)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn put_proxy_address(e: &Env, proxy: Proxy) {
    e.storage().instance().set(
        &DataKey::Proxy(proxy.protocol_id.clone()),
        &proxy,
    );
    add_protocol_id(e, proxy.protocol_id);
}

pub fn has_proxy_address(e: &Env, protocol_id: String) -> bool {
    e.storage()
        .instance()
        .has(&DataKey::Proxy(protocol_id))
}

pub fn get_proxy(e: &Env, protocol_id: String) -> Result<Proxy, AggregatorError> {
    match e
        .storage()
        .instance()
        .get(&DataKey::Proxy(protocol_id))
    {
        Some(proxy) => Ok(proxy),
        None => Err(AggregatorError::InvalidProtocolId),
    }
}

pub fn remove_proxy_address(e: &Env, protocol_id: String) {
    if has_proxy_address(e, protocol_id.clone()) {
        e.storage()
            .instance()
            .remove(&DataKey::Proxy(protocol_id.clone()));
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

pub fn set_pause_protocol(e: &Env, protocol_id: String, paused: bool) {
    e.storage()
        .instance()
        .set(&DataKey::ProtocolPaused(protocol_id), &paused);
}

pub fn is_protocol_paused(e: &Env, protocol_id: String) -> bool {
    e.storage()
        .instance()
        .get(&DataKey::ProtocolPaused(protocol_id))
        .unwrap_or(false)
}
