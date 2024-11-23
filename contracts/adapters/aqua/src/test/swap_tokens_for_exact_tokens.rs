use soroban_sdk::{Address, vec, Vec};
use crate::test::AquaAggregatorAdapterTest;
use adapter_interface::AdapterError;
use soroban_sdk::testutils::Ledger;
use super::aqua_adapter_contract::AdapterError as AdapterErrorDeployer;


#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client_not_initialized.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
fn swap_tokens_for_exact_tokens_amount_out_negative() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_expired() {
    let test = AquaAggregatorAdapterTest::setup();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::DeadlineExpired))
    );
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_invalid_path() {
    let test = AquaAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];

    test.adapter_client.swap_tokens_for_exact_tokens( // add try_ to test the error
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
   
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInvalidPath)));
}


#[test]
// Panics because LP does not exist; here panics with a Error(Storage, MissingValue)
// We should implement a pair_address.exist() without needing to call the Factory
#[should_panic]
fn try_swap_tokens_for_exact_tokens_pair_does_not_exist() {
    let test = AquaAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_3.address.clone());

    test.adapter_client.swap_tokens_for_exact_tokens(
        &1, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_insufficient_output_amount() {
    let test = AquaAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.budget().reset_unlimited();
    test.adapter_client.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientOutputAmount)));
}

#[test]
// #[should_panic(expected = "Amount of token in required is greater than the maximum amount expected")] // TODO: Test the imported error
#[should_panic(expected = "HostError: Error(WasmVm, InvalidAction)")] //TODO: Why it changed using the deployer?
fn try_swap_tokens_for_exact_tokens_amount_in_max_not_enough() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    // test.adapter_client_not_initialized.initialize(
    //     &String::from_str(&test.env, "aqua"),
    //     &test.multihop_client.address);

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let expected_amount_out = 50;
    // From Aqua tests
    let amount_in_should = 50;

    test.adapter_client.swap_tokens_for_exact_tokens(
        &expected_amount_out, // amount_out
        &(amount_in_should-1),                   // amount_in_max
        &path,                // path
        &test.user,           // to
        &deadline,            // deadline
    );


    // TODO: Evaluate if change panic message with error object (check benchmark)
    // assert_eq!(
    //     result,
    //     Err(Ok(AdapterError::NotInitialized))
    // );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_should() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let expected_amount_out = 50;
    // From Aqua tests
    let amount_in_should = 50;

    let initial_user_balance_0 = test.token_0.balance(&test.user);
    let initial_user_balance_1 = test.token_1.balance(&test.user);

    let amounts = test.adapter_client.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &(amount_in_should),  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(amounts.get(0).unwrap(), amount_in_should);
    assert_eq!(amounts.get(1).unwrap(), expected_amount_out);


    assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0 - amount_in_should);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance_1 + expected_amount_out);

}


#[test]
fn swap_tokens_for_exact_tokens_3_hops() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    path.push_back(test.token_3.address.clone());

    let expected_amount_out = 50;
    // From Aqua tests
    let amount_in_should =50;

    let initial_user_balance_0 = test.token_0.balance(&test.user);
    let initial_user_balance_1 = test.token_1.balance(&test.user);
    let initial_user_balance_2 = test.token_2.balance(&test.user);
    let initial_user_balance_3 = test.token_3.balance(&test.user);

    let amounts = test.adapter_client.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &amount_in_should,  // amount_in_max
        &path, // path
        &test.user, // to
        &desired_deadline); // deadline


    assert_eq!(amounts.get(0).unwrap(), amount_in_should); 
    assert_eq!(amounts.get(1).unwrap(), expected_amount_out);

    assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0 - amount_in_should);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance_1);
    assert_eq!(test.token_2.balance(&test.user), initial_user_balance_2);
    assert_eq!(test.token_3.balance(&test.user), initial_user_balance_3 + expected_amount_out);
}
