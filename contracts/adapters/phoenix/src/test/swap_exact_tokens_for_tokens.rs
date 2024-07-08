use soroban_sdk::{Address, vec, Vec, String};
use crate::test::{PhoenixAggregatorAdapterTest};
use soroswap_aggregator_adapter_interface::{AdapterError};


#[test]
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
fn swap_exact_tokens_for_tokens_amount_in_negative() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &-1,           // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::NegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_amount_out_min_negative() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &-1,           // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::NegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_expired() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::DeadlineExpired))
    );
}



#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_invalid_path() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    test.adapter_client.swap_exact_tokens_for_tokens(
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
    let test = PhoenixAggregatorAdapterTest::setup();

    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.env.budget().reset_unlimited();
    test.adapter_client.swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientInputAmount)));
}



#[test]
#[should_panic] // TODO: Test the imported error
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.adapter_client.initialize(
        &String::from_str(&test.env, "phoenix"),
        &test.multihop_client.address);

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    path.push_back(test.token_3.address.clone());

    let amount_in = 50i128;

    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9

    let expected_amount_out = 50;

    test.env.budget().reset_unlimited();
    test.adapter_client.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out + 1),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
    );

    // assert_eq!(
    //     result,
    //     Err(Ok(CombinedRouterError::RouterInsufficientOutputAmount))
    // );
}



// #[test]
// fn swap_exact_tokens_for_tokens_enough_output_amount() {
//     let test = PhoenixAggregatorAdapterTest::setup();
//     test.adapter_client.initialize(
//         &String::from_str(&test.env, "phoenix"),
//         &test.multihop_client.address);

//     let deadline: u64 = test.env.ledger().timestamp() + 1000;  

//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     let amount_in = 1_000_000;

//     //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9

//     let expected_amount_out = 3987999;

//     test.env.budget().reset_unlimited();
//     let executed_amounts = test.adapter_client.swap_exact_tokens_for_tokens(
//         &amount_in, //amount_in
//         &(expected_amount_out),  // amount_out_min
//         &path, // path
//         &test.user, // to
//         &deadline); // deadline

//     assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
//     assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
    
// }


// #[test]
// fn swap_exact_tokens_for_tokens_2_hops() {
//     let test = PhoenixAggregatorAdapterTest::setup();
//     test.env.budget().reset_unlimited();
//     test.adapter_client.initialize(
//         &String::from_str(&test.env, "phoenix"),
//         &test.multihop_client.address);

//     let deadline: u64 = test.env.ledger().timestamp() + 1000;  
//     let initial_user_balance = 20_000_000_000_000_000_000;
//     let amount_0: i128 = 1_000_000_000_000_000_000;
//     let amount_1: i128 = 4_000_000_000_000_000_000;
//     let amount_2: i128 = 8_000_000_000_000_000_000;

    
//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());
//     path.push_back(test.token_2.address.clone());


//     let amount_in = 123_456_789;
//     // fee = 123456789 * 3 /1000 =  370370,367 = 370371 // USE CEILING
//     // amount_in less fee = 123456789- 370371 = 123086418
//     // First out = (123086418*4000000000000000000)/(1000000000000000000 + 123086418) = 492345671.939398935 = 492345671
//     let first_out = 492345671;
//     // fee = 492345671 * 3 /1000 =  1477037.013 = 1477038 // USE CEILING
//     // in less fee = 492345671 - 1477038 = 490868633
//     // Second out = (490868633*8000000000000000000)/(4000000000000000000 + 490868633) = 981737265.879523993 = 981737265
//     let expected_amount_out = 981737265;

//     let executed_amounts = test.adapter_client.swap_exact_tokens_for_tokens(
//         &amount_in, //amount_in
//         &0,  // amount_out_min
//         &path, // path
//         &test.user, // to
//         &deadline); // deadline

//     assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
//     assert_eq!(executed_amounts.get(1).unwrap(), first_out);
//     assert_eq!(executed_amounts.get(2).unwrap(), expected_amount_out);
    
//     assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - amount_in);
//     assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1*2);
//     assert_eq!(test.token_2.balance(&test.user), initial_user_balance -amount_2 + expected_amount_out);
// }

