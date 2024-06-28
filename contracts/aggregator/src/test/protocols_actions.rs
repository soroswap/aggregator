use soroban_sdk::{Address, Vec, vec, String, testutils::Address as _};

use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
use crate::models::{ProxyAddressPair};

pub fn new_update_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<ProxyAddressPair> {
    vec![&test.env,
        ProxyAddressPair {
            protocol_id: String::from_str(&test.env, "some_protocol"),
            address: test.router_contract.address.clone(),
        },
    ]
}

#[test]
fn test_update_protocols() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_protocols_addresses(&test);
    let result = test.aggregator_contract.update_protocols(&update_aggregator_addresses);

    assert_eq!(result, ());
}

#[test]
fn test_update_protocols_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test.aggregator_contract.try_update_protocols(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

#[test]
fn test_get_protocols() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.get_protocols();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_protocols_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test.aggregator_contract.try_update_protocols(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

#[test]
fn test_remove_protocol() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.remove_protocol(&String::from_str(&test.env, "soroswap"));

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
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.is_protocol_paused(&String::from_str(&test.env, "soroswap"));

    assert!(!result);
}