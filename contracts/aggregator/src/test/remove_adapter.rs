extern crate std;
use crate::error::AggregatorError as AggregatorErrorFromCrate;
// use crate::models::Adapter;
use crate::test::{create_soroswap_phoenix_comet_addresses_for_deployer, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};
use soroban_sdk::{vec, String, Vec};
use super::soroswap_aggregator_contract::{AggregatorError, Adapter};

pub fn new_protocol_vec(test: &SoroswapAggregatorTest, protocol_id: &String) -> Vec<Adapter> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        Adapter {
            protocol_id: protocol_id.clone(),
            address: new_router.address,
            paused: false,
        },
    ]
}

#[test]
fn test_remove_adapter() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    // check that protocol is not paused
    let is_protocol_paused = test
        .aggregator_contract
        .get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(is_protocol_paused, false);

    test.aggregator_contract
        .remove_adapter(&String::from_str(&test.env, "soroswap"));
    test.aggregator_contract
        .remove_adapter(&String::from_str(&test.env, "phoenix"));
    test.aggregator_contract
    .remove_adapter(&String::from_str(&test.env, "comet"));

    let mut updated_protocols = test.aggregator_contract.get_adapters();
    let expected_empty_vec = vec![&test.env];
    assert_eq!(updated_protocols, expected_empty_vec);

    // when removing protocol, paused return error
    let is_protocol_paused = test
        .aggregator_contract
        .try_get_paused(&String::from_str(&test.env, "soroswap"));
    assert_eq!(
        is_protocol_paused,
        Err(Ok(AggregatorError::ProtocolNotFound))
    );

    //add new protocol
    let new_protocol_0 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_0"));
    test.aggregator_contract.update_adapters(&new_protocol_0);

    updated_protocols = test.aggregator_contract.get_adapters();
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
    test.aggregator_contract.update_adapters(&new_protocol_1);

    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols.get(0), new_protocol_0.get(0));
    assert_eq!(updated_protocols.get(1), new_protocol_1.get(0));

    // remove new protocol 0
    test.aggregator_contract
        .remove_adapter(&String::from_str(&test.env, "new_protocol_0"));
    updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols, new_protocol_1);
}

// test non initialized
#[test]
fn test_remove_adapter_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test
        .aggregator_contract_not_initialized
        .try_remove_adapter(&String::from_str(&test.env, "soroswap"));

    assert_eq!(result, Err(Ok(AggregatorErrorFromCrate::NotInitialized)));
}

// update protocols can only be called by admin

#[test]
fn test_update_adapters_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_soroswap_phoenix_comet_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone(), test.comet_adapter_contract.address.clone());
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_adapters();
    assert_eq!(protocols, initialize_aggregator_addresses);

    let protocol_id_to_remove = String::from_str(&test.env, "soroswap");

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract
        .mock_auths(&[MockAuth {
            address: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.aggregator_contract.address.clone(),
                fn_name: "remove_adapter",
                args: (protocol_id_to_remove.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .remove_adapter(&protocol_id_to_remove.clone());

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.aggregator_contract.address.clone(),
                    Symbol::new(&test.env, "remove_adapter"),
                    (protocol_id_to_remove.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}
