// TODO: Test set_admin function and events
// TODO: Test upgreade wasm
use soroban_sdk::{
    testutils::{Address as _}, Address};
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};

extern crate std;
use crate::error::AggregatorError;
use crate::test::{create_protocols_addresses, SoroswapAggregatorTest};

#[test]
fn set_admin() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);

    test.env.budget().reset_default();
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);
    

    // get admin
    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    // set admin
    let new_admin = Address::generate(&test.env);
    test.aggregator_contract.set_admin(&new_admin);
    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, new_admin);
}   


// test non initialized
#[test]
fn test_set_admin_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let new_admin = Address::generate(&test.env);

    let result = test
        .aggregator_contract
        .try_set_admin(&new_admin);

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}



// set_admin can only be called by admin

#[test]
fn test_set_admin_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    let new_admin = Address::generate(&test.env);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract
        .mock_auths(&[MockAuth {
            address: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.aggregator_contract.address.clone(),
                fn_name: "set_admin",
                args: (new_admin.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_admin(&new_admin.clone());

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.aggregator_contract.address.clone(),
                    Symbol::new(&test.env, "set_admin"),
                    (new_admin.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}


