use soroban_sdk::{contracttype, Env, Address, String};
use adapter_interface::{AdapterError};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Initialized,
    ProtocolId,
    ProtocolAddress,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/* INITIALIZED */
pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}


/* PROTOCOL ID - STRING */
pub fn set_protocol_id(e: &Env, protocol_id: String) {
    e.storage().instance().set(&DataKey::ProtocolId, &protocol_id);
}

pub fn get_protocol_id(e: &Env) -> Result<String, AdapterError> {
    e.storage().instance().get(&DataKey::ProtocolId).ok_or(AdapterError::NotInitialized)
}


/* PROTOCOL ADDRESS */
pub fn set_protocol_address(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::ProtocolAddress, &address);
}

pub fn get_protocol_address(e: &Env) -> Result<Address, AdapterError> {
    e.storage().instance().get(&DataKey::ProtocolAddress).ok_or(AdapterError::NotInitialized)
}

