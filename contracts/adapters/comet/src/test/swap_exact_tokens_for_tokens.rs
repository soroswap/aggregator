extern crate std;

use soroban_sdk::{Address, vec, Vec};
use crate::test::CometAggregatorAdapterTest;
use adapter_interface::AdapterError;

#[test]
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract_not_initialized.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_exact_tokens_for_tokens_amount_in_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &-1,           // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &1,            // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_exact_tokens_for_tokens_amount_out_min_negative() {
    let test: CometAggregatorAdapterTest<'_> = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &-1,           // amount_out_min
        &path,         // path
        &test.user,    // to
        &1,            // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #405)")]
fn swap_exact_tokens_for_tokens_expired() {
    let test = CometAggregatorAdapterTest::setup();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &10000,            // amount_in
        &1,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );
}



#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_invalid_path() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    test.adapter_contract.swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_insufficient_input_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.env.budget().reset_unlimited();


    let amount = 100_000;

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount,        // amount_in
        &amount,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
}



#[test]
#[should_panic(expected = "HostError: Error(Contract, #20)")]
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_in = 1_000_000;

    let expected_amount_out = 1276;

    test.env.budget().reset_unlimited();
    test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out + 1),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
    );
}



#[test]
fn swap_exact_tokens_for_tokens_enough_output_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_in = 1_000_000;

    let expected_amount_out = 1276;

    test.env.budget().reset_unlimited();
    let executed_amounts = test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &(expected_amount_out),  // amount_out_minf
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
    
}