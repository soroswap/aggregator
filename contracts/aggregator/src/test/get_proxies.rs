extern crate std;
use soroban_sdk::{Address, Vec, vec, String, testutils::Address as _};
use soroban_sdk::{
    IntoVal,
    testutils::{
        MockAuth,
        MockAuthInvoke,
        AuthorizedInvocation,
        AuthorizedFunction
    },
    Symbol
};
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, create_soroswap_router};
use crate::models::{Proxy};

pub fn new_update_proxies_addresses(test: &SoroswapAggregatorTest) -> Vec<Proxy> {
    vec![&test.env,
        Proxy {
            protocol_id: String::from_str(&test.env, "some_protocol"),
            address: test.router_contract.address.clone(),
            paused: false,
        },
    ]
}

// Create new soroswap router to overwrite the porevious
pub fn update_overwrite_soroswap_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<Proxy> {
    let new_router = create_soroswap_router(&test.env);
    vec![&test.env,
        Proxy {
            protocol_id: String::from_str(&test.env, "soroswap"),
            address: new_router.address,
            paused: false,
        },
    ]
}

#[test]
fn test_get_proxies() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.get_proxies();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_proxies_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test.aggregator_contract.try_get_proxies();

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}