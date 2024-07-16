extern crate std;
use soroban_sdk::{Vec, String, Address};
use crate::DexDistribution;
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
// use soroban_sdk::{
//     vec,
//     IntoVal,
//     testutils::{
//         MockAuth, 
//         MockAuthInvoke,
//         AuthorizedInvocation,
//         AuthorizedFunction
//     },
//     Symbol
// };

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
    const MAX_DISTRIBUTION_LENGTH: u32 = 15;
    for _i in 0..MAX_DISTRIBUTION_LENGTH+1 { // this will be 16
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
fn swap_tokens_for_exact_tokens_succeed_correctly_one_protocol() {
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
fn swap_tokens_for_exact_tokens_succeed_correctly_one_protocol_two_hops() {
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
    path.push_back(test.token_2.address.clone());


    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    // let initial_user_balance: i128 = 20_000_000_000_000_000_000;
    // let amount_0: i128 = 1_000_000_000_000_000_000;
    // let amount_1: i128 = 4_000_000_000_000_000_000;
    // let amount_2: i128 = 8_000_000_000_000_000_000;

    let expected_amount_out = 123_456_789;
    // pair token_1, token_2
    // token_1 is r_in, token_2 is r_out
    // (r_in*amount_out)*1000 / (r_out - amount_out)*997
    // (4_000_000_000_000_000_000*123456789)*1000 / ((8_000_000_000_000_000_000 - 123456789)*997) + 1 = 
    // 493827156000000000000000000000 / (7999999999876543211 * 997) +1 = 
    // 493827156000000000000000000000 / 7975999999876913581367 +1 = CEIL(61914136.911687662) +1 = 61914137 +1 = 61914138
    // 
    let middle_amount_in =61914138;

    // pair token_0, token_1
    // token_0 is r_in, token_1 is r_out
    // first amount in = 
    // (1_000_000_000_000_000_000*61914138)*1000 / ((4_000_000_000_000_000_000 - 61914138)*997) + 1 = 
    // 61914138000000000000000000000 / (3999999999938085862 * 997) + 1 =
    // CEIL (61914138000000000000000000000 / 3987999999938271604414) +1 = ceil(15525109.8) +1 = 15525111

    let amount_in_should =15525111;


    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(), //token_in
        &test.token_2.address.clone(), //token_out
        &expected_amount_out, //amount_out
        &amount_in_should,  // amount_in_max
        &distribution_vec, // path
        &test.user, // to
        &deadline); // deadline


    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);

   // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in_should);
    assert_eq!(user_balance_after_1, user_balance_before_1);
    assert_eq!(user_balance_after_2, user_balance_before_2 + expected_amount_out);

    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in_should);
    expected_soroswap_result_vec.push_back(middle_amount_in);
    expected_soroswap_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);

    assert_eq!(result, expected_result);

}

#[test]
fn swap_tokens_for_exact_tokens_succeed_correctly_same_protocol_twice() {
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
    let distribution_1 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 3,
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_out = 30_000_000;

    // The total expected amount will come from 2 different trades:
    let expected_amount_out_0 = 30_000_000_i128.checked_div(4).unwrap().checked_mul(1).unwrap();
    let expected_amount_out_1 = total_expected_amount_out - expected_amount_out_0;

    // 

    // swap 0 occurs with original reserves
    // R0 = 1_000_000_000_000_000_000;
    // R1 = 4_000_000_000_000_000_000;
    // expected_amount_out_0 = 7500000
    // ceil((r_in*amount_out)*1000 / (r_out - amount_out))*997 + 1 
    // (1_000_000_000_000_000_000*7500000)*1000 / ((4_000_000_000_000_000_000 - 7500000)*997) + 1 =
    // (1000000000000000000*7500000)*1000 / ((4000000000000000000 - 7500000)*997) + 1 =
    // CEIL(7500000000000000000000000000 / 3987999999992522500000 ) + 1 =
    // CEIL (1880641.925780858)  + 1 = 1880642 + 1 = 1880643
    let amount_in_should_0 = 1880643;
    

    // swap 1 occurs with new reserves
    // R0 = 1_000_000_000_000_000_000 + 1880643 = 1000000000000000000 + 1880643 = 1000000000001880643
    // R1 = 4_000_000_000_000_000_000 - 7500000 = 4000000000000000000 - 7500000 = 3999999999992500000
    // expected_amount_out_1 = 22500000
    // ceil((r_in*amount_out)*1000 / (r_out - amount_out))*997 + 1 
    // (1000000000001880643*22500000)*1000 / ((3999999999992500000 - 22500000)*997) + 1 =
    // (1000000000001880643*22500000)*1000 / ((3999999999992500000 - 22500000)*997) + 1 =
    // CEIL(22500000000042314467500000000 / 3987999999970090000000) + 1 =
    // CEIL(5641925.777384921) + 1 = 5641926 + 1 = 5641927
    let amount_in_should_1 = 5641927;
    let total_amount_in_should = amount_in_should_0 + amount_in_should_1;
    
    // with just one unit less of total_amount_in_should, this will fail with expected error
    // ExcessiveInputAmount
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_out,
        &(total_amount_in_should-1),
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );

    assert_eq!(result, Err(Ok(AggregatorError::ExcessiveInputAmount)));

    // however with the correct amount it should succeed

    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);

    let success_result = test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_out,
        &total_amount_in_should,
        &distribution_vec,
        &test.user.clone(),
        &deadline
    );
    // check new balances and compare with result
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);

    assert_eq!(user_balance_after_0, user_balance_before_0 - total_amount_in_should);
    assert_eq!(user_balance_after_1, user_balance_before_1 + total_expected_amount_out);

    let mut expected_soroswap_result_vec_0: Vec<i128> = Vec::new(&test.env);
    let mut expected_soroswap_result_vec_1: Vec<i128> = Vec::new(&test.env);

    // first swap
    expected_soroswap_result_vec_0.push_back(amount_in_should_0);
    expected_soroswap_result_vec_0.push_back(expected_amount_out_0);

    // second swap
    expected_soroswap_result_vec_1.push_back(amount_in_should_1);
    expected_soroswap_result_vec_1.push_back(expected_amount_out_1);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec_0);
    expected_result.push_back(expected_soroswap_result_vec_1);

    assert_eq!(success_result, expected_result);
}

#[test]
fn swap_tokens_for_exact_tokens_succeed_correctly_two_protocols() {
    todo!();
}