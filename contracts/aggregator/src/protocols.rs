use soroban_sdk::{
    contracttype,
    Env,
    Address};

use storage::{
    set_total_protocols,
    get_total_protocols,
    set_protocol,
    get_protocol,
    exist_protocol};

pub fn push_protocol(e: &Env, protocol: Protocol) {
    // TODO: Check if protocol with same name? or aggregator address already exist
    let next_index = get_total_protocols(&e);
    put_protocol(&e, &protocol, &next_index);
    set_total_protocols(&e, next_index.checked_sum(1).unwrap());
}

