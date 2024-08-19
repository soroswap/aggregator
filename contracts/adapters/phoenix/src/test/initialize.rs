use soroban_sdk::{String};
use crate::test::{PhoenixAggregatorAdapterTest};
use adapter_interface::{AdapterError};

#[test]
fn test_initialize_and_get_values() {
    let test = PhoenixAggregatorAdapterTest::setup();

    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let protocol_id = test.adapter_client.get_protocol_id();
    assert_eq!(protocol_id, String::from_str(&test.env, "phoenix"));

    let protocol_address = test.adapter_client.get_protocol_address();
    assert_eq!(protocol_address, test.multihop_client.address);
}

// test initialize twice
#[test]
fn test_initialize_twice() {
    let test = PhoenixAggregatorAdapterTest::setup();

    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let result = test.adapter_client.try_initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    assert_eq!(result,Err(Ok(AdapterError::AlreadyInitialized)));
}

// test get protocol id not initialized
#[test]
fn test_get_protocol_id_not_initialized() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let result = test.adapter_client.try_get_protocol_id(); 
    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));
}

// test get protocol address not initialized
#[test]
fn test_get_protocol_address_not_initialized() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let result = test.adapter_client.try_get_protocol_address();
    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));
}
