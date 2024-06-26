extern crate std;
use crate::error::AggregatorError;
use crate::models::ProxyAddressPair;
use crate::test::{create_protocols_addresses, create_soroswap_router, SoroswapAggregatorTest};
use soroban_sdk::{testutils::Address as _, vec, Address, String, Vec};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};

pub fn new_update_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<ProxyAddressPair> {
    vec![
        &test.env,
        ProxyAddressPair {
            protocol_id: String::from_str(&test.env, "some_protocol"),
            address: test.router_contract.address.clone(),
        },
    ]
}

// Create new soroswap router to overwrite the porevious
pub fn update_overwrite_soroswap_protocols_addresses(
    test: &SoroswapAggregatorTest,
) -> Vec<ProxyAddressPair> {
    let new_router = create_soroswap_router(&test.env);
    vec![
        &test.env,
        ProxyAddressPair {
            protocol_id: String::from_str(&test.env, "soroswap"),
            address: new_router.address,
        },
    ]
}

/* update_protocols  */
#[test]
fn test_update_protocols_add_new() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    //Update aggregator
    let update_aggregator_addresses = new_update_protocols_addresses(&test);
    test.aggregator_contract
        .update_protocols(&update_aggregator_addresses);

    // test that now we have 2 protocols
    let updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(
        updated_protocols.get(0),
        initialize_aggregator_addresses.get(0)
    );
    assert_eq!(updated_protocols.get(1), update_aggregator_addresses.get(0));
}

// test that soroswaop protocol is indeed overwriten with new router addresws
#[test]
fn test_update_protocols_overwrite() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_protocols();
    assert_eq!(protocols, initialize_aggregator_addresses);

    // if we add a new protocol, it wont be overwitten
    let new_aggregator_addresses = new_update_protocols_addresses(&test);
    test.aggregator_contract
        .update_protocols(&new_aggregator_addresses);

    // generate new router address and protocol addresses
    let update_aggregator_addresses = update_overwrite_soroswap_protocols_addresses(&test);
    // check that router address are different
    assert_ne!(
        update_aggregator_addresses.get(0),
        initialize_aggregator_addresses.get(0)
    );

    test.aggregator_contract
        .update_protocols(&update_aggregator_addresses);

    // check that protocol values are updated
    // but the other protocol is still the same
    let updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols.get(0), update_aggregator_addresses.get(0));
    assert_eq!(updated_protocols.get(1), new_aggregator_addresses.get(0));
}

#[test]
fn test_update_protocols_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test
        .aggregator_contract
        .try_update_protocols(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

// update protocols can only be called by admin

#[test]
fn test_update_protocols_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_protocols();
    assert_eq!(protocols, initialize_aggregator_addresses);

    // if we add a new protocol, it wont be overwitten
    let new_aggregator_addresses = new_update_protocols_addresses(&test);
    // test.aggregator_contract.update_protocols(&new_aggregator_addresses);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract
        .mock_auths(&[MockAuth {
            address: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.aggregator_contract.address.clone(),
                fn_name: "update_protocols",
                args: (new_aggregator_addresses.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .update_protocols(&new_aggregator_addresses.clone());

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.aggregator_contract.address.clone(),
                    Symbol::new(&test.env, "update_protocols"),
                    (new_aggregator_addresses.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}
