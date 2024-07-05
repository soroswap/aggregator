extern crate std;
use crate::error::AggregatorError;
use crate::models::Proxy;
use crate::test::{create_protocols_addresses, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{testutils::Address as _, vec, Address, String, Vec};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};

pub fn new_protocol_vec(
    test: &SoroswapAggregatorTest,
    protocol_id: &String,
) -> Vec<Proxy> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        Proxy {
            protocol_id: protocol_id.clone(),
            address: new_router.address,
            paused: false,
        },
    ]
}

#[test]
fn test_set_pause_true_false() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check that protocol is not paused
    let mut is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, false);

    //  PAUSE
    test.aggregator_contract
        .set_pause(&String::from_str(&test.env, "soroswap"), &true);

    let mut updated_protocols = test.aggregator_contract.get_protocols();

    // we should have the vec but with paused protocol
    let expected_protocols_vec = vec![
        &test.env,
        Proxy {
            protocol_id: String::from_str(&test.env, "soroswap"),
            address: test.soroswap_proxy_contract.address.clone(),
            paused: true,
        },
    ];
    assert_eq!(updated_protocols, expected_protocols_vec);

    // check is_paused

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, true);


    //add new protocol
    let new_protocol_0 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_0"));
    test.aggregator_contract.update_protocols(&new_protocol_0);

    let mut expected_new_protocols = vec![
        &test.env,
        Proxy {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            address: initialize_aggregator_addresses.get(0).unwrap().address,
            paused: true,
        },
        new_protocol_0.get(0).unwrap()
    ];

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, expected_new_protocols);

    // add new protoco 1
    let new_protocol_1 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_1"));
    test.aggregator_contract.update_protocols(&new_protocol_1);

    expected_new_protocols = vec![
        &test.env,
        Proxy {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            address: initialize_aggregator_addresses.get(0).unwrap().address,
            paused: true,
        },
        new_protocol_0.get(0).unwrap(),
        new_protocol_1.get(0).unwrap()
    ];

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, expected_new_protocols);

    // PAUSE PROTOCOL 1
    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "new_protocol_1"), &true);

    expected_new_protocols = vec![
        &test.env,
        Proxy {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            address: initialize_aggregator_addresses.get(0).unwrap().address,
            paused: true,
        },
        new_protocol_0.get(0).unwrap(),
        Proxy {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            address: new_protocol_1.get(0).unwrap().address,
            paused: true,
        },
    ];

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, true);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_0"));
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_1"));
    assert_eq!(is_protocol_paused, true);


    // UNPAUSE new_protocol_1

    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "new_protocol_1"), &false);

    expected_new_protocols = vec![
        &test.env,
        Proxy {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            address: initialize_aggregator_addresses.get(0).unwrap().address,
            paused: true,
        },
        new_protocol_0.get(0).unwrap(),
        Proxy {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            address: new_protocol_1.get(0).unwrap().address,
            paused: false,
        },
    ];

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, true);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_0"));
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_1"));
    assert_eq!(is_protocol_paused, false);


    // UNPAUSE soroswap

    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "soroswap"), &false);

    expected_new_protocols = vec![
        &test.env,
        Proxy {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            address: initialize_aggregator_addresses.get(0).unwrap().address,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
        Proxy {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            address: new_protocol_1.get(0).unwrap().address,
            paused: false,
        },
    ];

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_0"));
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "new_protocol_1"));
    assert_eq!(is_protocol_paused, false);

}

// test non initialized
#[test]
fn test_set_pause_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test
        .aggregator_contract
        .try_set_pause(&String::from_str(&test.env, "soroswap"), &true);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

// test non initialized
#[test]
fn test_set_pause_non_existent() {
    let test = SoroswapAggregatorTest::setup();

    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test
        .aggregator_contract
        .try_set_pause(&String::from_str(&test.env, "nonsoroswap"), &true);

    assert_eq!(result, Err(Ok(AggregatorError::ProtocolNotFound)));
}

