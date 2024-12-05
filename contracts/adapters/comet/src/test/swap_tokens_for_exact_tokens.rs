extern crate std;

use soroban_sdk::{Address, vec, Vec};
use crate::test::CometAggregatorAdapterTest;
use adapter_interface::AdapterError;

#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract_not_initialized.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_tokens_for_exact_tokens_amount_out_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &1,        // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &1,        // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #405)")]
fn swap_tokens_for_exact_tokens_expired() {
    let test = CometAggregatorAdapterTest::setup();

    let path: Vec<Address> = Vec::new(&test.env);

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_invalid_path() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];

    test.adapter_contract.swap_tokens_for_exact_tokens( // add try_ to test the error
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
   
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInvalidPath)));
}


#[test]
#[should_panic(expected = "HostError: Error(Contract, #19)")]
fn try_swap_tokens_for_exact_tokens_insufficient_input_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.budget().reset_unlimited();

    let amount_out = 1276;
    let expected_amount_in = 999876;

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &amount_out,        // amount_out
        &(expected_amount_in - 1),        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
}

#[test]
fn try_swap_tokens_for_exact_tokens_sufficient_input_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.budget().reset_unlimited();

    let amount_out = 1276;
    let expected_amount_in = 999876;

    let executed_amounts = test.adapter_contract.swap_tokens_for_exact_tokens(
        &amount_out,        // amount_out
        &expected_amount_in,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );

    assert_eq!(executed_amounts.get(0).unwrap(), expected_amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), amount_out);
}