extern crate std;
use crate::error::AggregatorError;
use crate::test::{
    create_protocols_addresses, create_soroswap_phoenix_comet_addresses_for_deployer, create_soroswap_router, new_update_adapters_addresses, new_update_adapters_addresses_deployer, SoroswapAggregatorTest
};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};
use super::soroswap_aggregator_contract::Protocol;

use soroban_sdk::{vec, String, Vec};
use super::soroswap_aggregator_contract::Adapter;

// Create new soroswap router to overwrite the porevious
pub fn update_overwrite_soroswap_protocols_addresses(
    test: &SoroswapAggregatorTest,
) -> Vec<Adapter> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        Adapter {
            protocol_id: Protocol::Soroswap,
            router: new_router.address,
            paused: false,
        },
    ]
}

/* update_adapters  */
#[test]
fn test_update_adapters_add_new() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_soroswap_phoenix_comet_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone(), test.comet_adapter_contract.address.clone());
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_adapters_addresses_deployer(&test);
    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);

    // test that now we have 2 protocols
    let updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(
        updated_protocols.get(0),
        initialize_aggregator_addresses.get(0)
    );
    assert_eq!(updated_protocols.get(3), update_aggregator_addresses.get(0));
    // test both are not paused
    for protocol_address in updated_protocols {
        let is_protocol_paused = test
            .aggregator_contract
            .get_paused(&protocol_address.protocol_id.clone());
        assert_eq!(is_protocol_paused, false);
    }
}

// test that soroswaop protocol is indeed overwriten with new router addresws
#[test]
fn test_update_adapters_overwrite() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_soroswap_phoenix_comet_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone(), test.comet_adapter_contract.address.clone());
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_adapters();
    assert_eq!(protocols, initialize_aggregator_addresses);

    // if we add a new protocol, it wont be overwitten
    let new_aggregator_addresses = new_update_adapters_addresses_deployer(&test);
    test.aggregator_contract
        .update_adapters(&new_aggregator_addresses);

    // generate new router address and protocol addresses
    let update_aggregator_addresses = update_overwrite_soroswap_protocols_addresses(&test);
    // check that router address are different
    assert_ne!(
        update_aggregator_addresses.get(0),
        initialize_aggregator_addresses.get(0)
    );

    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);

    // check that protocol values are updated
    // but the other protocol is still the same
    let updated_protocols = test.aggregator_contract.get_adapters();
    assert_eq!(updated_protocols.get(0), update_aggregator_addresses.get(0));
    assert_eq!(updated_protocols.get(3), new_aggregator_addresses.get(0));

    // test both are not paused
    for protocol_address in updated_protocols {
        let is_protocol_paused = test
            .aggregator_contract
            .get_paused(&protocol_address.protocol_id.clone());
        assert_eq!(is_protocol_paused, false);
    }
}

#[test]
fn test_update_adapters_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test
        .aggregator_contract_not_initialized
        .try_update_adapters(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

// update protocols can only be called by admin

#[test]
fn test_update_adapters_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract_not_initialized
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract_not_initialized.get_adapters();
    assert_eq!(protocols, initialize_aggregator_addresses);

    // if we add a new protocol, it wont be overwitten
    let new_aggregator_addresses = new_update_adapters_addresses(&test);
    // test.aggregator_contract_not_initialized.update_adapters(&new_aggregator_addresses);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract_not_initialized
        .mock_auths(&[MockAuth {
            router: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.aggregator_contract_not_initialized.address.clone(),
                fn_name: "update_adapters",
                args: (new_aggregator_addresses.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .update_adapters(&new_aggregator_addresses.clone());

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.aggregator_contract_not_initialized.address.clone(),
                    Symbol::new(&test.env, "update_adapters"),
                    (new_aggregator_addresses.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}
