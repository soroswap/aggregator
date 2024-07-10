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
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_negative_amount_in() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_negative_amount_out_min() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_deadline_expired() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_distribution_over_max() {
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_zero_parts() {
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_protocol_not_found() {
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_paused_protocol() {
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
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
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out+1),
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InsufficientOutputAmount)));
}

#[test]
fn swap_exact_tokens_for_tokens_succeed_correctly() {
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
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    // check initial user balance of both tokens
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);


    //  MOCK THE SPECIFIC AUTHORIZATION
    // TODO: solve the sub invokes to to the mock auth corectly
    let result = test.aggregator_contract
    // .mock_auths(&[
    //     MockAuth {
    //         address: &test.user.clone(),
    //         invoke: 
    //             &MockAuthInvoke {
    //                 contract: &test.aggregator_contract.address.clone(),
    //                 fn_name: "swap_exact_tokens_for_tokens",
    //                 args: (
    //                     test.token_0.address.clone(),
    //                     test.token_1.address.clone(),
    //                     amount_in,
    //                     (expected_amount_out),
    //                     distribution_vec.clone(),
    //                     test.user.clone(),
    //                     deadline,
    //                 ).into_val(&test.env),
    //                 sub_invokes: &[],
    //             },
    //     }
    // ])
    .swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out),
        &distribution_vec.clone(),
        &test.user.clone(),
        &deadline
    );
    
//     // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
//     assert_eq!(
//         test.env.auths(),
//         std::vec![(
//             test.user.clone(),
//             AuthorizedInvocation {
//                function: AuthorizedFunction::Contract((
//                    test.aggregator_contract.address.clone(),
//                    Symbol::new(&test.env, "swap_exact_tokens_for_tokens"),
//                    (
//                     test.token_0.address.clone(),
//                     test.token_1.address.clone(),
//                     amount_in,
//                     (expected_amount_out),
//                     distribution_vec.clone(),
//                     test.user.clone(),
//                     deadline,
//                 ).into_val(&test.env)
//                )),
//                sub_invocations: std::vec![]
//            }
//         )]
//    );


    // let result = test.aggregator_contract.swap_exact_tokens_for_tokens(
    //     &test.token_0.address.clone(),
    //     &test.token_1.address.clone(),
    //     &amount_in,
    //     &(expected_amount_out),
    //     &distribution_vec,
    //     &test.user.clone(),
    //     &deadline&test.token_0.address.clone(),
    //     &test.token_1.address.clone(),
    //     &amount_in,
    //     &(expected_amount_out),
    //     &distribution_vec,
    //     &test.user.clone(),
    //     &deadline
    // );
    // check new user balances
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in);
    assert_eq!(user_balance_after_1, user_balance_before_1 + expected_amount_out);
    
    // check the result vec
    // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in);
    expected_soroswap_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);

    assert_eq!(result, expected_result);
}