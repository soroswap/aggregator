use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short, String}; 
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, new_update_proxies_addresses};

use crate::event::{
    InitializedEvent,
    UpdateProtocolsEvent,
    RemovedProtocolEvent,
    PausedProtocolEvent};


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
fn update_proxies_event() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_proxies_addresses(&test);
    test.aggregator_contract.update_proxies(&update_aggregator_addresses);

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
fn remove_proxy_event() {
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    
    // Remove protocol
    let protocol_id = String::from_str(&test.env, "soroswap");
    test.aggregator_contract.remove_proxy(&protocol_id);
    
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


#[test]
fn set_pause_event() {
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    
    // Remove protocol
    let protocol_id = String::from_str(&test.env, "soroswap");
    let true_false_vec = vec![&test.env, true, false];

    for my_bool in true_false_vec.iter(){

        test.aggregator_contract.set_pause(&protocol_id, &my_bool);
    
        let set_pause = test.env.events().all().last().unwrap();
        let expected_set_pause: PausedProtocolEvent = PausedProtocolEvent {
            protocol_id: protocol_id.clone(),
            paused: my_bool
        };
        assert_eq!(
            vec![&test.env, set_pause.clone()],
            vec![
                &test.env,
                (
                    test.aggregator_contract.address.clone(),
                    ("SoroswapAggregator", symbol_short!("paused")).into_val(&test.env),
                    (expected_set_pause).into_val(&test.env)
                ),
            ]
        );
        let false_set_pause: PausedProtocolEvent = PausedProtocolEvent {
            protocol_id: String::from_str(&test.env, "uniswap"),
            paused: my_bool
        };
        assert_ne!(
            vec![&test.env, set_pause.clone()],
            vec![
                &test.env,
                (
                    test.aggregator_contract.address.clone(),
                    ("SoroswapAggregator", symbol_short!("paused")).into_val(&test.env),
                    (false_set_pause).into_val(&test.env)
                ),
            ]
        );
        // Wrong symbol_short
        assert_ne!(
            vec![&test.env, set_pause.clone()],
            vec![
                &test.env,
                (
                    test.aggregator_contract.address.clone(),
                    ("SoroswapAggregator", symbol_short!("pp")).into_val(&test.env),
                    (expected_set_pause).into_val(&test.env)
                ),
            ]
        );
        // Wrong string
        assert_ne!(
            vec![&test.env, set_pause.clone()],
            vec![
                &test.env,
                (
                    test.aggregator_contract.address.clone(),
                    ("SoroswapAggregatorr", symbol_short!("paused")).into_val(&test.env),
                    (expected_set_pause).into_val(&test.env)
                ),
            ]
        );

    }
    
}
