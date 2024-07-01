use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short}; 
use soroban_sdk::{Address, testutils::Address as _};
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
use crate::test::protocols_actions::new_update_protocols_addresses;

use crate::event::{
    InitializedEvent,
    UpdateProtocolsEvent};


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

// test protocol_updated
// // UPDATE PROTOCOL EVENT
// #[contracttype] 
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct UpdateProtocolsEvent {
//     pub proxy_addresses: Vec<ProxyAddressPair>
// }

// /// Publishes an `UpdateProtocolsEvent` to the event stream.
// pub(crate) fn protocols_updated(
//     e: &Env,
//     proxy_addresses: Vec<ProxyAddressPair>
// ) {
//     let event = UpdateProtocolsEvent {
    //         proxy_addresses,
    //     };
    
    //     e.events().publish(("SoroswapAggregator", symbol_short!("update")), event);
    // }

    // the function that will generate this event  is 

/* update_protocols  */
// #[test]
// fn test_update_protocols_add_new() {
//     let test = SoroswapAggregatorTest::setup();

//     //Initialize aggregator
//     let initialize_aggregator_addresses = create_protocols_addresses(&test);
//     test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

//     let admin = test.aggregator_contract.get_admin();
//     assert_eq!(admin, test.admin);

//     //Update aggregator
//     let update_aggregator_addresses = new_update_protocols_addresses(&test);
//     test.aggregator_contract.update_protocols(&update_aggregator_addresses);
    
//     // test that now we have 2 protocols
//     let updated_protocols = test.aggregator_contract.get_protocols();
//     assert_eq!(updated_protocols.get(0), initialize_aggregator_addresses.get(0));
//     assert_eq!(updated_protocols.get(1), update_aggregator_addresses.get(0));
// }

// create the test
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