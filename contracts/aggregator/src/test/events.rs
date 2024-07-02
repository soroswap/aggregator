use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short, String}; 
use soroban_sdk::{Address, testutils::Address as _};
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
use crate::test::protocols_actions::new_update_protocols_addresses;

use crate::event::{
    InitializedEvent,
    UpdateProtocolsEvent,
    RemovedProtocolEvent};


#[test]
fn initialized_event() {
    let test = SoroswapAggregatorTest::setup();
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        admin: test.admin.clone(),
        proxy_addresses: initialize_aggregator_addresses.clone(),
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        admin: test.user,
        proxy_addresses: initialize_aggregator_addresses.clone(),
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("init")).into_val(&test.env),
                (false_initialized_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address,
                ("SoroswapAggregatorr", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}

#[test]
fn update_protocols_event() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_protocols_addresses(&test);
    test.aggregator_contract.update_protocols(&update_aggregator_addresses);

    let updated_event = test.env.events().all().last().unwrap();

    let expected_updated_event: UpdateProtocolsEvent = UpdateProtocolsEvent {
        proxy_addresses: update_aggregator_addresses.clone(),
    };

    assert_eq!(
        vec![&test.env, updated_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("update")).into_val(&test.env),
                (expected_updated_event).into_val(&test.env)
            ),
        ]
    );

    let false_updated_event: UpdateProtocolsEvent = UpdateProtocolsEvent {
        proxy_addresses: initialize_aggregator_addresses.clone(),
    };

    assert_ne!(
        vec![&test.env, updated_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("update")).into_val(&test.env),
                (false_updated_event).into_val(&test.env)
            ),
        ]
    );

    // Wront symbol_short
    assert_ne!(
        vec![&test.env, updated_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("updat")).into_val(&test.env),
                (expected_updated_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, updated_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address,
                ("SoroswapAggregatorr", symbol_short!("update")).into_val(&test.env),
                (expected_updated_event).into_val(&test.env)
            ),
        ]
    );

}

#[test]
fn remove_protocol_event() {
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    
    // Remove protocol
    let protocol_id = String::from_str(&test.env, "soroswap");
    test.aggregator_contract.remove_protocol(&protocol_id);
    
    let removed_event = test.env.events().all().last().unwrap();
    let expected_removed_event: RemovedProtocolEvent = RemovedProtocolEvent {
        protocol_id: protocol_id.clone(),
    };
    assert_eq!(
        vec![&test.env, removed_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("removed")).into_val(&test.env),
                (expected_removed_event).into_val(&test.env)
            ),
        ]
    );
    let false_removed_event: RemovedProtocolEvent = RemovedProtocolEvent {
        protocol_id: String::from_str(&test.env, "uniswap"),
    };
    assert_ne!(
        vec![&test.env, removed_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("removed")).into_val(&test.env),
                (false_removed_event).into_val(&test.env)
            ),
        ]
    );
    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, removed_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("remove")).into_val(&test.env),
                (expected_removed_event).into_val(&test.env)
            ),
        ]
    );
    // Wrong string
    assert_ne!(
        vec![&test.env, removed_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address,
                ("SoroswapAggregatorr", symbol_short!("removed")).into_val(&test.env),
                (expected_removed_event).into_val(&test.env)
            ),
        ]
    );
}
