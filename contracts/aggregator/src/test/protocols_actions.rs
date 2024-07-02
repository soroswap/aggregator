extern crate std;
use crate::error::AggregatorError;
use crate::models::ProxyAddressPair;
use crate::test::{create_protocols_addresses, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{testutils::Address as _, vec, Address, String, Vec};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};

pub fn new_update_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<ProxyAddressPair> {
    vec![
        &test.env,
        ProxyAddressPair {
            protocol_id: String::from_str(&test.env, "some_protocol"),
            address: test.router_contract.address.clone(),
        },
    ]
}

// Create new soroswap router to overwrite the porevious
pub fn update_overwrite_soroswap_protocols_addresses(
    test: &SoroswapAggregatorTest,
) -> Vec<ProxyAddressPair> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        ProxyAddressPair {
            protocol_id: String::from_str(&test.env, "soroswap"),
            address: new_router.address,
        },
    ]
}

#[test]
fn test_get_protocols() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.get_protocols();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_protocols_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test
        .aggregator_contract
        .try_update_protocols(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

#[test]
fn test_remove_protocol() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test
        .aggregator_contract
        .remove_protocol(&String::from_str(&test.env, "soroswap"));

    assert_eq!(result, ());
}

// #[test]
// fn test_pause_protocol() {
//     let test = SoroswapAggregatorTest::setup();

//     //Initialize aggregator
//     let initialize_aggregator_addresses = create_protocols_addresses(&test);
//     test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

//     let result = test.aggregator_contract.pause_protocol(&String::from_str(&test.env, "soroswap"));

//     assert_eq!(result, ());
// }

// #[test]
// fn test_unpause_protocol() {
//     let test = SoroswapAggregatorTest::setup();

//     //Initialize aggregator
//     let initialize_aggregator_addresses = create_protocols_addresses(&test);
//     test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

//     let result = test.aggregator_contract.unpause_protocol(&String::from_str(&test.env, "soroswap"));

//     assert_eq!(result, ());
// }

#[test]
fn test_is_protocol_paused() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test
        .aggregator_contract
        .is_protocol_paused(&String::from_str(&test.env, "soroswap"));

    assert!(!result);
}
