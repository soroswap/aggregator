
extern crate std;
use crate::error::AggregatorError as AggregatorErrorFromCrate;
// use crate::models::Adapter;
use crate::test::{create_soroswap_phoenix_comet_addresses_for_deployer, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{vec, String, Vec};
use super::soroswap_aggregator_contract::Protocol;


use super::soroswap_aggregator_contract::{Adapter, AggregatorError};

pub fn new_protocol_vec(test: &SoroswapAggregatorTest, protocol_id: &Protocol) -> Vec<Adapter> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        Adapter {
            protocol_id: protocol_id.clone(),
            router: new_router.address,
            paused: false,
        },
    ]
}

#[test]
fn test_set_pause_true_false() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    let initialize_aggregator_addresses = create_soroswap_phoenix_comet_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone(), test.comet_adapter_contract.address.clone());
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    // check that protocol is not paused
    let mut is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Soroswap);
    assert_eq!(is_protocol_paused, false);

    //  PAUSE
    test.aggregator_contract
        .set_pause(&Protocol::Soroswap, &true);

    let mut updated_protocols = test.aggregator_contract.get_adapters();

    // we should have the vec but with paused protocol
    let expected_protocols_vec = vec![
        &test.env,
        Adapter {
            protocol_id: Protocol::Soroswap,
            router: test.soroswap_adapter_contract.address.clone(),

            paused: true,
        },
        Adapter {
            protocol_id: Protocol::Phoenix,
            router: test.phoenix_adapter_contract.address.clone(),
            paused: false,
        },
        Adapter {
            protocol_id: Protocol::Comet,
            router: test.comet_adapter_contract.address.clone(),
            paused: false,
        },
    ];
    assert_eq!(updated_protocols, expected_protocols_vec);

    // check is_paused

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Soroswap);
    assert_eq!(is_protocol_paused, true);

    //add new protocol
    let new_protocol_0 = new_protocol_vec(&test, &Protocol::Comet);
    test.aggregator_contract.update_adapters(&new_protocol_0);

    let mut expected_new_protocols = vec![
        &test.env,
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(0).unwrap().router,
            paused: true,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(1).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(1).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(2).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(2).unwrap().router,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
    ];

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, expected_new_protocols);

    // add new protoco 1
    let new_protocol_1 = new_protocol_vec(&test, &Protocol::Comet);
    test.aggregator_contract.update_adapters(&new_protocol_1);

    expected_new_protocols = vec![
        &test.env,
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(0).unwrap().router,
            paused: true,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(1).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(1).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(2).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(2).unwrap().router,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
        new_protocol_1.get(0).unwrap(),
    ];

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, expected_new_protocols);

    // PAUSE PROTOCOL 1
    test.aggregator_contract
        .set_pause(&Protocol::Comet, &true);

    expected_new_protocols = vec![
        &test.env,
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(0).unwrap().router,
            paused: true,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(1).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(1).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(2).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(2).unwrap().router,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
        Adapter {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            router: new_protocol_1.get(0).unwrap().router,
            paused: true,
        },
    ];

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Soroswap);
    assert_eq!(is_protocol_paused, true);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, true);

    // UNPAUSE new_protocol_1

    test.aggregator_contract
        .set_pause(&Protocol::Comet, &false);

    expected_new_protocols = vec![
        &test.env,
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(0).unwrap().router,
            paused: true,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(1).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(1).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(2).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(2).unwrap().router,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
        Adapter {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            router: new_protocol_1.get(0).unwrap().router,
            paused: false,
        },
    ];

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Soroswap);
    assert_eq!(is_protocol_paused, true);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, false);

    // UNPAUSE soroswap

    test.aggregator_contract
        .set_pause(&Protocol::Soroswap, &false);

    expected_new_protocols = vec![
        &test.env,
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(0).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(0).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(1).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(1).unwrap().router,
            paused: false,
        },
        Adapter {
            protocol_id: initialize_aggregator_addresses.get(2).unwrap().protocol_id,
            router: initialize_aggregator_addresses.get(2).unwrap().router,
            paused: false,
        },
        new_protocol_0.get(0).unwrap(),
        Adapter {
            protocol_id: new_protocol_1.get(0).unwrap().protocol_id,
            router: new_protocol_1.get(0).unwrap().router,
            paused: false,
        },
    ];

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, expected_new_protocols);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Soroswap);
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, false);

    is_protocol_paused = test
        .aggregator_contract
        .get_paused(&Protocol::Comet);
    assert_eq!(is_protocol_paused, false);
}

// test non initialized
#[test]
fn test_set_pause_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test
        .aggregator_contract_not_initialized
        .try_set_pause(&Protocol::Soroswap, &true);

    assert_eq!(result, Err(Ok(AggregatorErrorFromCrate::NotInitialized)));
}

// test non initialized
#[test]
fn test_set_pause_non_existent() {
    let test = SoroswapAggregatorTest::setup();

    // let initialize_aggregator_addresses = create_soroswap_phoenix_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone());
    
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test
        .aggregator_contract
        .try_set_pause(&Protocol::Comet, &true);

    assert_eq!(result, Err(Ok(AggregatorError::ProtocolNotFound)));
}

// test non initialized
#[test]
fn test_get_paused_non_existent() {
    let test = SoroswapAggregatorTest::setup();

    let result = test
        .aggregator_contract
        .try_get_paused(&Protocol::Comet);

    assert_eq!(result, Err(Ok(AggregatorError::ProtocolNotFound)));
}
