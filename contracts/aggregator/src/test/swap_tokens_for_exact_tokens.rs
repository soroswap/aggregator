extern crate std;
use soroban_sdk::{Vec, vec, String, Address};
use crate::DexDistribution;
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
use soroban_sdk::{
    IntoVal,
    testutils::{
        MockAuth, 
        MockAuthInvoke,
        AuthorizedInvocation,
        AuthorizedFunction
    },
    Symbol
};

#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &Vec::new(&test.env),
        &test.user.clone(),
        &100
    );
    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
    
}

#[test]
fn swap_tokens_for_exact_tokens_negative_amount_out() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &-1,
        &100,
        &Vec::new(&test.env),
        &test.user.clone(),
        &100
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::NegativeNotAllowed)));
}   

#[test]
fn swap_tokens_for_exact_tokens_negative_amount_in_max() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &-1,
        &Vec::new(&test.env),
        &test.user.clone(),
        &100
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::NegativeNotAllowed)));
}


#[test]
fn swap_tokens_for_exact_tokens_deadline_expired() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &Vec::new(&test.env),
        &test.user.clone(),
        &0
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::DeadlineExpired)));
}

#[test]
fn swap_tokens_for_exact_tokens_distribution_over_max() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    let MAX_DISTRIBUTION_LENGTH: u32 = 15;
    for i in 0..MAX_DISTRIBUTION_LENGTH+1 { // this will be 16
        let distribution = DexDistribution {
            protocol_id: String::from_str(&test.env, "protocol_id"),
            path: Vec::new(&test.env),
            parts: 1,
        };
        distribution_vec.push_back(distribution);
    }

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::DistributionLengthExceeded)));
}

#[test]
fn swap_tokens_for_exact_tokens_zero_parts() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "protocol_id"),
        path: Vec::new(&test.env),
        parts: 1,
    };
    let distribution_1 = DexDistribution {
        protocol_id: String::from_str(&test.env, "protocol_id"),
        path: Vec::new(&test.env),
        parts: 0,
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::ZeroDistributionPart)));

}

#[test]
fn swap_tokens_for_exact_tokens_protocol_not_found() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "protocol_id"),
        path: Vec::new(&test.env),
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::ProtocolNotFound)));
}

#[test]
fn swap_tokens_for_exact_tokens_paused_protocol() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: Vec::new(&test.env),
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    // pause the protocol
    test.aggregator_contract.set_pause(&String::from_str(&test.env, "soroswap"), &true);

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::ProtocolPaused)));
}


#[test]
fn swap_tokens_for_exact_tokens_excessive_input_amount() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .router_contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();
    
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &(amount_in_should-1),
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    assert_eq!(result, Err(Ok(AggregatorError::ExcessiveInputAmount)));
}

#[test]
fn swap_tokens_for_exact_tokens_succeed_correctly() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
    // call the function
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .router_contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();

    // check initial user balance of both tokens
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);

    let result = test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &amount_in_should,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );

// TODO test specific mock auth

    // check new user balances
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
//     // compare

    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in_should);
    assert_eq!(user_balance_after_1, user_balance_before_1 + expected_amount_out);    
    // check the result vec
    // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in_should);
    expected_soroswap_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);

    assert_eq!(result, expected_result);
}


#[test]
fn swap_tokens_for_exact_tokens_succeed_correctly_two_hops() {
//     let test = SoroswapAggregatorTest::setup();
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;
//     // Initialize aggregator
//     let initialize_aggregator_addresses = create_protocols_addresses(&test);
//     test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses); 
//     // call the function
//     let mut distribution_vec = Vec::new(&test.env);
//     // add one with part 1 and other with part 0
//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());
//     path.push_back(test.token_2.address.clone());


//     let distribution_0 = DexDistribution {
//         protocol_id: String::from_str(&test.env, "soroswap"),
//         path,
//         parts: 1,
//     };
//     distribution_vec.push_back(distribution_0);

//     let initial_user_balance: i128 = 20_000_000_000_000_000_000;
//     let amount_0: i128 = 1_000_000_000_000_000_000;
//     let amount_1: i128 = 4_000_000_000_000_000_000;
//     let amount_2: i128 = 8_000_000_000_000_000_000;

//     let amount_out = 123_456_789;
//     // fee = 123456789 * 3 /1000 =  370370,367 = 370371 // USE CEILING
//     // amount_out less fee = 123456789- 370371 = 123086418
//     // First out = (123086418*4000000000000000000)/(1000000000000000000 + 123086418) = 492345671.939398935 = 492345671
//     let first_out = 492345671;
//     // fee = 492345671 * 3 /1000 =  1477037.013 = 1477038 // USE CEILING
//     // in less fee = 492345671 - 1477038 = 490868633
//     // Second out = (490868633*8000000000000000000)/(4000000000000000000 + 490868633) = 981737265.879523993 = 981737265
//     let expected_amount_out = 981737265;

//     let user_balance_before_0 = test.token_0.balance(&test.user);
//     let user_balance_before_1 = test.token_1.balance(&test.user);
//     let user_balance_before_2 = test.token_2.balance(&test.user);

//     let result = test.aggregator_contract
//     .swap_tokens_for_exact_tokens(
//         &test.token_0.address.clone(),
//         &test.token_2.address.clone(),
//         &amount_out,
//         &(0),
//         &distribution_vec.clone(),
//         &test.user.clone(),
//         &deadline
//     );

//     let user_balance_after_0 = test.token_0.balance(&test.user);
//     let user_balance_after_1 = test.token_1.balance(&test.user);
//     let user_balance_after_2 = test.token_2.balance(&test.user);

//    // compare
//     assert_eq!(user_balance_after_0, user_balance_before_0 - amount_out);
//     assert_eq!(user_balance_after_1, user_balance_before_1);
//     assert_eq!(user_balance_after_2, user_balance_before_2 + expected_amount_out);


//     let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
//     expected_soroswap_result_vec.push_back(amount_out);
//     expected_soroswap_result_vec.push_back(first_out);
//     expected_soroswap_result_vec.push_back(expected_amount_out);

//     let mut expected_result = Vec::new(&test.env);
//     expected_result.push_back(expected_soroswap_result_vec);

//     assert_eq!(result, expected_result);
todo!();  
}