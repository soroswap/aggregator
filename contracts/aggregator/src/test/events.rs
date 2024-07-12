use soroban_sdk::{testutils::{Events},
    vec,
    Vec,
    Address,
    IntoVal, symbol_short, String}; 
use crate::DexDistribution;

use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, new_update_adapters_addresses};

use crate::event::{
    InitializedEvent,
    UpdateProtocolsEvent,
    RemovedProtocolEvent,
    PausedProtocolEvent,
    SwapEvent,
};


#[test]
fn initialized_event() {
    let test = SoroswapAggregatorTest::setup();
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        admin: test.admin.clone(),
        adapter_addresses: initialize_aggregator_addresses.clone(),
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
        adapter_addresses: initialize_aggregator_addresses.clone(),
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
fn update_adapters_event() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_adapters_addresses(&test);
    test.aggregator_contract.update_adapters(&update_aggregator_addresses);

    let updated_event = test.env.events().all().last().unwrap();

    let expected_updated_event: UpdateProtocolsEvent = UpdateProtocolsEvent {
        adapter_addresses: update_aggregator_addresses.clone(),
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
        adapter_addresses: initialize_aggregator_addresses.clone(),
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
fn remove_adapter_event() {
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    
    // Remove protocol
    let protocol_id = String::from_str(&test.env, "soroswap");
    test.aggregator_contract.remove_adapter(&protocol_id);
    
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



#[test]
fn swap_exact_tokens_for_tokens_event() {
    // create test
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 

    let mut distribution_vec = Vec::new(&test.env);
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

   
    test.aggregator_contract
    .swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out),
        &distribution_vec.clone(),
        &test.user.clone(),
        &deadline
    );
    // check the event
    let swap_event = test.env.events().all().last().unwrap();
    let expected_swap_event: SwapEvent = SwapEvent {
        token_in: test.token_0.address.clone(),
        token_out: test.token_1.address.clone(),
        amount_in,
        amount_out: expected_amount_out,
        distribution: distribution_vec.clone(),
        to: test.user.clone(),
    };
    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("swap")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );
}

#[test]
fn swap_tokens_for_exact_tokens_event() {
    // create test
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 

    let mut distribution_vec = Vec::new(&test.env);
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .router_contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();

   
    test.aggregator_contract
    .swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &(amount_in_should),
        &distribution_vec.clone(),
        &test.user.clone(),
        &deadline
    );
    // check the event
    let swap_event = test.env.events().all().last().unwrap();
    let expected_swap_event: SwapEvent = SwapEvent {
        token_in: test.token_0.address.clone(),
        token_out: test.token_1.address.clone(),
        amount_in: amount_in_should,
        amount_out: expected_amount_out,
        distribution: distribution_vec.clone(),
        to: test.user.clone(),
    };
    // test one by one swap_event.clone().0
    // assert_eq!(
    //     swap_event.clone().0,
    //     test.aggregator_contract.address.clone()
    // );
    // assert_eq!(
    //     swap_event.clone().1,
    //     ("SoroswapAggregator", symbol_short!("swap")).into_val(&test.env)
    // );
    // assert_eq!(
    //     swap_event.clone().1,
        
    //     expected_swap_event.clone()
            
    // );

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.aggregator_contract.address.clone(),
                ("SoroswapAggregator", symbol_short!("swap")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );
}