extern crate std;
use crate::error::AggregatorError;
use crate::models::Proxy;
use crate::test::{create_protocols_addresses, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{vec, String, Vec};
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
fn test_remove_proxy() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check that protocol is not paused
    let is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, false);

    test.aggregator_contract
        .remove_proxy(&String::from_str(&test.env, "soroswap"));

    let mut updated_protocols = test.aggregator_contract.get_proxies();
    let expected_empty_vec = vec![&test.env];
    assert_eq!(updated_protocols, expected_empty_vec);

    // when removing protocol, paused return error
    let is_protocol_paused = test
        .aggregator_contract
        .try_get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, Err(Ok(AggregatorError::ProtocolNotFound)));

    //add new protocol
    let new_protocol_0 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_0"));
    test.aggregator_contract.update_proxies(&new_protocol_0);

    updated_protocols = test.aggregator_contract.get_proxies();
    assert_eq!(updated_protocols, new_protocol_0);

    // // test both are not paused
    // for protocol_address in updated_protocols {
    //     let is_protocol_paused = test
    //         .aggregator_contract
    //         .get_paused(&protocol_address.protocol_id.clone());
    //     assert_eq!(is_protocol_paused, false);
    // }

    // add new protoco 1
    let new_protocol_1 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_1"));
    test.aggregator_contract.update_proxies(&new_protocol_1);

    updated_protocols = test.aggregator_contract.get_proxies();
    assert_eq!(updated_protocols.get(0), new_protocol_0.get(0));
    assert_eq!(updated_protocols.get(1), new_protocol_1.get(0));

    // remove new protocol 0
    test.aggregator_contract
        .remove_proxy(&String::from_str(&test.env, "new_protocol_0"));
    updated_protocols = test.aggregator_contract.get_proxies();
    assert_eq!(updated_protocols, new_protocol_1);
}

// test non initialized
#[test]
fn test_remove_proxy_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test
        .aggregator_contract
        .try_remove_proxy(&String::from_str(&test.env, "soroswap"));

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

// update protocols can only be called by admin

#[test]
fn test_update_proxies_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_proxies();
    assert_eq!(protocols, initialize_aggregator_addresses);

    let protocol_id_to_remove = String::from_str(&test.env, "soroswap");

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract
        .mock_auths(&[MockAuth {
            address: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.aggregator_contract.address.clone(),
                fn_name: "remove_proxy",
                args: (protocol_id_to_remove.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .remove_proxy(&protocol_id_to_remove.clone());

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.aggregator_contract.address.clone(),
                    Symbol::new(&test.env, "remove_proxy"),
                    (protocol_id_to_remove.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}
