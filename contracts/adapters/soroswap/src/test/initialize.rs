use soroban_sdk::{String};
use crate::test::{SoroswapAggregatorAdapterTest};
use soroswap_aggregator_adapter_interface::{AdapterError};

#[test]
fn test_initialize_and_get_values() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract_not_initialized.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);

    let protocol_id = test.adapter_contract_not_initialized.get_protocol_id();
    assert_eq!(protocol_id, String::from_str(&test.env, "soroswap"));

    let protocol_address = test.adapter_contract_not_initialized.get_protocol_address();
    assert_eq!(protocol_address, test.router_contract.address);
}

// test initialize twice
#[test]
fn test_initialize_twice() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract_not_initialized.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);

    let result = test.adapter_contract_not_initialized.try_initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);

    assert_eq!(result,Err(Ok(AdapterError::AlreadyInitialized)));
}

// test get protocol id not initialized
#[test]
fn test_get_protocol_id_not_initialized() {
    let test = SoroswapAggregatorAdapterTest::setup();

    let result = test.adapter_contract_not_initialized.try_get_protocol_id(); 
    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));
}

// test get protocol address not initialized
#[test]
fn test_get_protocol_address_not_initialized() {
    let test = SoroswapAggregatorAdapterTest::setup();

    let result = test.adapter_contract_not_initialized.try_get_protocol_address();
    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));
}
