extern crate std;
use crate::error::AggregatorError as AggregatorErrorFromCrate;
use crate::test::SoroswapAggregatorTest;
// use crate::DexDistribution;
use super::soroswap_aggregator_contract::{AggregatorError, DexDistribution};
use super::soroswap_aggregator_contract::Protocol; 

use soroban_sdk::{Address, Vec, vec, BytesN, Symbol};
use soroban_sdk::token::TokenClient;
use soroban_sdk::U256;
use soroban_sdk::testutils::{
    Address as _,
};
use soroban_sdk::FromVal; 


#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract_not_initialized.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &Vec::new(&test.env),
        &test.user.clone(),
        &100,
    );
    assert_eq!(result, Err(Ok(AggregatorErrorFromCrate::NotInitialized)));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #616)")] //Negible Amount
fn swap_tokens_for_exact_tokens_negative_amount_out() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &-1,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
}

#[test]
fn swap_tokens_for_exact_tokens_negible_amount() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1000,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    /*
        Because amount_out =0,
        amount_0 will be 1000*part/ total parts
        1000 * 1 / 1001 =0
        Hence will fail with Negible Amount Error
    */

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &1000,//amount_out
        &i128::MAX,//amount_in_max
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );

    assert_eq!(result, Err(Ok(AggregatorError::NegibleAmount)));
}


/*
Negatives in amount_in_max will be allowed, but will fail with ExcessiveInputAmount
Because the Aggregator checks for 
if final_amount_in > amount_in_max {
        return Err(AggregatorError::ExcessiveInputAmount);
}
*/
#[test]
fn swap_tokens_for_exact_tokens_negative_amount_in_max() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &-1,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );

    assert_eq!(result, Err(Ok(AggregatorError::ExcessiveInputAmount)));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #503)")] //Deadline Expired
fn swap_tokens_for_exact_tokens_deadline_expired() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &0,
    );
}

#[test]
fn swap_tokens_for_exact_tokens_distribution_over_max() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    const MAX_DISTRIBUTION_LENGTH: u32 = 15;
    for _i in 0..MAX_DISTRIBUTION_LENGTH + 1 {
        // this will be 16
        let distribution = DexDistribution {
            protocol_id: Protocol::Comet,
            path: Vec::new(&test.env),
            parts: 1,
            bytes: None
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
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::DistributionLengthExceeded)));
}

#[test]
fn swap_tokens_for_exact_tokens_zero_parts() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Comet,
        path: Vec::new(&test.env),
        parts: 1,
        bytes: None
    };
    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Comet,
        path: Vec::new(&test.env),
        parts: 0,
        bytes: None
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
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::ZeroDistributionPart)));
}

// #[test]
// fn swap_tokens_for_exact_tokens_protocol_not_found() {
//     let test = SoroswapAggregatorTest::setup();
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;
//     // Initialize aggregator
//     // s
//     // call the function
//     let mut distribution_vec = Vec::new(&test.env);
//     // add one with part 1 and other with part 0
//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     let distribution_0 = DexDistribution {
//         protocol_id: Protocol::Comet,
//         path,
//         parts: 1,
//         bytes: None
//     };
//     distribution_vec.push_back(distribution_0);

//     let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
//         &test.token_0.address.clone(),
//         &test.token_1.address.clone(),
//         &100,
//         &100,
//         &distribution_vec,
//         &test.user.clone(),
//         &deadline,
//     );
//     // compare the error
//     assert_eq!(result, Err(Ok(AggregatorError::ProtocolNotFound)));
// }

#[test]
fn swap_tokens_for_exact_tokens_paused_protocol() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // s
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    // pause the protocol
    test.aggregator_contract
        .set_pause(&Protocol::Soroswap, &true);

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::ProtocolPaused)));
}

#[test]
fn swap_tokens_for_exact_tokens_malformed_path_wrong_start() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    
    let mut distribution_vec = Vec::new(&test.env);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &i128::MAX,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InvalidPath)));
}


#[test]
fn swap_tokens_for_exact_tokens_malformed_path_wrong_end() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    
    let mut distribution_vec = Vec::new(&test.env);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_0.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &i128::MAX,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InvalidPath)));
}


#[test]
fn swap_tokens_for_exact_tokens_excessive_input_amount() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // s
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .soroswap_router_contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &expected_amount_out,
        &(amount_in_should - 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    assert_eq!(result, Err(Ok(AggregatorError::ExcessiveInputAmount)));
}

#[test]
fn swap_tokens_for_exact_tokens_succeed_correctly_one_protocol() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // s
    // call the function
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .soroswap_router_contract
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
        &deadline,
    );

    // TODO test specific mock auth

    // check new user balances
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    //     // compare

    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - amount_in_should
    );
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + expected_amount_out
    );
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
    // s
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path,
        parts: 1,
        bytes: None
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
    let middle_amount_in = 61914138;

    // pair token_0, token_1
    // token_0 is r_in, token_1 is r_out
    // first amount in =
    // (1_000_000_000_000_000_000*61914138)*1000 / ((4_000_000_000_000_000_000 - 61914138)*997) + 1 =
    // 61914138000000000000000000000 / (3999999999938085862 * 997) + 1 =
    // CEIL (61914138000000000000000000000 / 3987999999938271604414) +1 = ceil(15525109.8) +1 = 15525111

    let amount_in_should = 15525111;

    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test.aggregator_contract.swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(), //token_in
        &test.token_2.address.clone(), //token_out
        &expected_amount_out,          //amount_out
        &amount_in_should,             // amount_in_max
        &distribution_vec,             // path
        &test.user,                    // to
        &deadline,
    ); // deadline

    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);

    // compare
    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - amount_in_should
    );
    assert_eq!(user_balance_after_1, user_balance_before_1);
    assert_eq!(
        user_balance_after_2,
        user_balance_before_2 + expected_amount_out
    );

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
    // s
    // call the function
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 3,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_out = 30_000_000;

    // The total expected amount will come from 2 different trades:
    let expected_amount_out_0 = 30_000_000_i128
        .checked_div(4)
        .unwrap()
        .checked_mul(1)
        .unwrap();
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
        &(total_amount_in_should - 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
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
        &deadline,
    );
    // check new balances and compare with result
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);

    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - total_amount_in_should
    );
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + total_expected_amount_out
    );

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
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_soroswap_phoenix_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone());

    // test.aggregator_contract
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Phoenix,
        path: path.clone(),
        parts: 3,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_out = 30_000_000;

    // The total expected amount will come from 2 different trades:
    let expected_amount_out_0 = 7500000;//30_000_000_i128
        // .checked_div(4)
        // .unwrap()
        // .checked_mul(1)
        // .unwrap();
    let expected_amount_out_1 = 22500000;//total_expected_amount_out - expected_amount_out_0;

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

    // PHOENIX RETURNS THE SAME
    let amount_in_should_1 = 22500000;
    let total_amount_in_should = amount_in_should_0 + amount_in_should_1;

    // with just one unit less of total_amount_in_should, this will fail with expected error
    // ExcessiveInputAmount
    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_out,
        &(total_amount_in_should - 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
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
        &deadline,
    );
    // check new balances and compare with result
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);

    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - total_amount_in_should
    );
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + total_expected_amount_out
    );

    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    let mut expected_phoenix_result_vec: Vec<i128> = Vec::new(&test.env);

    // first swap
    expected_soroswap_result_vec.push_back(amount_in_should_0);
    expected_soroswap_result_vec.push_back(expected_amount_out_0);

    // second swap
    expected_phoenix_result_vec.push_back(amount_in_should_1);
    expected_phoenix_result_vec.push_back(expected_amount_out_1);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);
    expected_result.push_back(expected_phoenix_result_vec);

    assert_eq!(success_result, expected_result);
}

#[test]
fn swap_tokens_for_exact_tokens_succeed_comet() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_2.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Comet,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let amount_out = 1_000_000;
    // bone = 10**18
    // fee_ratio = (10**7 - 30000) * 10**11 => 997000000000000000
    // scaled_reserve_(out|in) = token_(out|in)_reserve * 10**7
    // adjusted_out = amount_out * 10**7
    // base = (scaled_reserve_out * BONE) / (scaled_reserve_out) - adjusted_out) 
    // weight_ratio = out_token_weight * 10**18 / out_token_weight
    // power = ((base / BONE) ** (weight_ratio / BONE)) * BONE // The code treats the numbers as 18 digit fixed point values. So code does it differently, but this is equivelant
    // balance_ratio = power - BONE
    // amount_in = scaled_reserve_in * balance_ratio / BONE
    // adjusted_in = amount_in * BONE / fee_ratio
    // <= adjusted_in / 10**7

    // scaled_reserve_in = 800000000000 * 10**7 => 8000000000000000000
    // scaled_reserve_out = 200000000000 * 10**7 => 2000000000000000000
    // adjusted_out = 1_000_000 * 10**7 => 10000000000000
    // base = (2000000000000000000 * BONE) / (2000000000000000000 - 10000000000000) => 1000005000025000125
    // weight_ratio = 2000000 * BONE / 8000000 => 250000000000000000
    // power = ((1000005000025000125 / BONE) ** (250000000000000000 / BONE)) * 10**18 => 1250006250031
    // balance_ratio = 1000001250006250031 - BONE => 1250006250031
    // amount_in = 8000000000000000000 * 1250006250031 / BONE = 10000050000248
    // adjusted_in = 10000050000248 * BONE / fee_ratio => 10030140421512
    // 10030140421512 / 10**7 => 1003015
    let expected_amount_in = 1_003_015;

    // check initial user balance of both tokens
    // let user_balance_before_0 = test.token_0.balance(&test.user);
    // let user_balance_before_2 = test.token_2.balance(&test.user);

    let _result = test
        .aggregator_contract
        .swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_out,
            &expected_amount_in,
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

    // // check new user balances
    // let user_balance_after_0 = test.token_0.balance(&test.user);
    // let user_balance_after_2 = test.token_2.balance(&test.user);
    // // compare
    // assert_eq!(user_balance_after_0, user_balance_before_0 - expected_amount_in);
    // assert_eq!(
    //     user_balance_after_2,
    //     user_balance_before_2 + amount_out
    // );

    // // check the result vec
    // // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    // let mut expected_comet_result_vec: Vec<i128> = Vec::new(&test.env);
    // expected_comet_result_vec.push_back(expected_amount_in);
    // expected_comet_result_vec.push_back(amount_out);

    // let mut expected_result = Vec::new(&test.env);
    // expected_result.push_back(expected_comet_result_vec);

    // assert_eq!(result, expected_result);
}

#[test]
fn swap_tokens_for_exact_tokens_succeed_comet_soroswap_two_hops() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0

    let mut path_soroswap: Vec<Address> = Vec::new(&test.env);
    path_soroswap.push_back(test.token_0.address.clone());
    path_soroswap.push_back(test.token_1.address.clone());
    path_soroswap.push_back(test.token_2.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Soroswap,
        path: path_soroswap,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let mut path_comet: Vec<Address> = Vec::new(&test.env);
    path_comet.push_back(test.token_0.address.clone());
    path_comet.push_back(test.token_2.address.clone());

    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Comet,
        path: path_comet,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_1);

    let amount_out = 2_000_000;


    let amount_out_soroswap = 1_000_000;
    // pair token_1, token_2
    // token_1 is r_in, token_2 is r_out
    // (r_in*amount_out)*1000 / (r_out - amount_out)*997
    // (4_000_000_000_000_000_000*1_000_000)*1000 / ((8_000_000_000_000_000_000 - 1_000_000)*997) + 1 =
    // 4000000000000000000000000000 / (7999999999999000000 * 997) +1 =
    // 4000000000000000000000000000 / 7975999999999003000000 +1 = CEIL(501504.51354068454) +1 = 501505 +1 = 501506
    //
    let middle_amount_in = 501506;

    // pair token_0, token_1
    // token_0 is r_in, token_1 is r_out
    // first amount in =
    // (1_000_000_000_000_000_000*501506)*1000 / ((4_000_000_000_000_000_000 - 501506)*997) + 1 =
    // 501506000000000000000000000 / (3999999999999498494 * 997) + 1 =
    // CEIL (501506000000000000000000000 / 3987999999999499998518) +1 = ceil(125753.76128386732) +1 = 125755

    let expected_amount_in_soroswap = 125755;


    let amount_out_comet = 1_000_000;
    // bone = 10**18
    // fee_ratio = (10**7 - 30000) * 10**11 => 997000000000000000
    // scaled_reserve_(out|in) = token_(out|in)_reserve * 10**7
    // adjusted_out = amount_out * 10**7
    // base = (scaled_reserve_out * BONE) / (scaled_reserve_out) - adjusted_out) 
    // weight_ratio = out_token_weight * 10**18 / out_token_weight
    // power = ((base / BONE) ** (weight_ratio / BONE)) * BONE // The code treats the numbers as 18 digit fixed point values. So code does it differently, but this is equivelant
    // balance_ratio = power - BONE
    // amount_in = scaled_reserve_in * balance_ratio / BONE
    // adjusted_in = amount_in * BONE / fee_ratio
    // <= adjusted_in / 10**7

    // scaled_reserve_in = 800000000000 * 10**7 => 8000000000000000000
    // scaled_reserve_out = 200000000000 * 10**7 => 2000000000000000000
    // adjusted_out = 1_000_000 * 10**7 => 10000000000000
    // base = (2000000000000000000 * BONE) / (2000000000000000000 - 10000000000000) => 1000005000025000125
    // weight_ratio = 2000000 * BONE / 8000000 => 250000000000000000
    // power = ((1000005000025000125 / BONE) ** (250000000000000000 / BONE)) * 10**18 => 1250006250031
    // balance_ratio = 1000001250006250031 - BONE => 1250006250031
    // amount_in = 8000000000000000000 * 1250006250031 / BONE = 10000050000248
    // adjusted_in = 10000050000248 * BONE / fee_ratio => 10030140421512
    // 10030140421512 / 10**7 => 1003015
    let expected_amount_in_comet = 1_003_015;

    let expected_amount_in = expected_amount_in_comet + expected_amount_in_soroswap;

    // check initial user balance of both tokens
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test
        .aggregator_contract
        .swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_out,
            &expected_amount_in,
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

    // check new user balances
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);
    // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - expected_amount_in);
    assert_eq!(
        user_balance_after_2,
        user_balance_before_2 + amount_out
    );

    // check the result vec
    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(expected_amount_in_soroswap);
    expected_soroswap_result_vec.push_back(middle_amount_in);
    expected_soroswap_result_vec.push_back(amount_out_soroswap);

    let mut expected_comet_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_comet_result_vec.push_back(expected_amount_in_comet);
    expected_comet_result_vec.push_back(amount_out_comet);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);
    expected_result.push_back(expected_comet_result_vec);

    assert_eq!(result, expected_result);
}

// Aqua

#[test]
fn swap_tokens_for_exact_tokens_missing_hash_aqua() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut distribution_vec = Vec::new(&test.env);
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_2.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    let expected_amount_out = 996996;

    let result = test.aggregator_contract.try_swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_in,
            &(expected_amount_out),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

    assert_eq!(result, Err(Ok(AggregatorError::MissingPoolHashes)));
}


#[test]
fn swap_tokens_for_exact_tokens_constant_product_pool_1_hop() {
    let test = SoroswapAggregatorTest::setup();
    let aqua_setup = test.aqua_setup;
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let router = aqua_setup.router;
    let [token1, token2, _, _] = aqua_setup.tokens;

    let tokens = Vec::from_array(&test.env, [token1.address.clone(), token2.address.clone()]);
    let user1 = Address::generate(&test.env);
    aqua_setup.reward_token.mint(&user1, &10_0000000);

    let (pool_hash, pool_address) = router.init_standard_pool(&user1, &tokens, &30);
    assert_eq!(
        router.pool_type(&tokens, &pool_hash),
        Symbol::new(&test.env, "constant_product")
    );
    let pool_info = router.get_info(&tokens, &pool_hash);
    assert_eq!(
        Symbol::from_val(&test.env, &pool_info.get(Symbol::new(&test.env, "pool_type")).unwrap()),
        Symbol::new(&test.env, "constant_product")
    );

    let token_share = TokenClient::new(&test.env, &router.share_id(&tokens, &pool_hash));

    token1.mint(&user1, &1000);
    assert_eq!(token1.balance(&user1), 1000);

    token2.mint(&user1, &1000);
    assert_eq!(token2.balance(&user1), 1000);

    assert_eq!(token_share.balance(&user1), 0);

    let desired_amounts = Vec::from_array(&test.env, [100, 100]);
    router.deposit(&user1, &tokens, &pool_hash, &desired_amounts, &0);
    assert_eq!(router.get_total_liquidity(&tokens), U256::from_u32(&test.env, 2));

    assert_eq!(token_share.balance(&user1), 100);
    assert_eq!(router.get_total_shares(&tokens, &pool_hash), 100);
    assert_eq!(token_share.balance(&pool_address), 0);
    assert_eq!(token1.balance(&user1), 900);
    assert_eq!(token1.balance(&pool_address), 100);
    assert_eq!(token2.balance(&user1), 900);
    assert_eq!(token2.balance(&pool_address), 100);

    assert_eq!(
        router.get_reserves(&tokens, &pool_hash),
        Vec::from_array(&test.env, [100, 100])
    );

    assert_eq!(
        router.estimate_swap(&tokens, &token1.address, &token2.address, &pool_hash, &97),
        48
    );


    let path: Vec<Address> = vec![&test.env,
        token1.address.clone(),
        token2.address.clone()];

    // vec pool hash
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, pool_hash.clone()];
    // in 97, our  48

    let mut distribution_vec = Vec::new(&test.env);
    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: Some(bytes_vec)
    };
    distribution_vec.push_back(distribution_0);

    // let executed_amounts = test.adapter_client.swap_tokens_for_exact_tokens(
    //     &48,        // amount_out
    //     &98,        // amount_in_max
    //     &path,     // path
    //     &user1, // to
    //     &0, // deadline,
    //     &Some(bytes_vec),
    // );
    let result = test.aggregator_contract.mock_all_auths().swap_tokens_for_exact_tokens(
        &token1.address.clone(),
        &token2.address.clone(),
        &48,        // amount_in
        &98,        // amount_out_min
        &distribution_vec.clone(),
        &user1.clone(),
        &deadline,
    );
    

    // // error OutMinNotSatisfied = 2006,
    
    assert_eq!(token1.balance(&user1), 803);
    assert_eq!(token1.balance(&pool_address), 197);
    assert_eq!(token2.balance(&user1), 948);
    assert_eq!(token2.balance(&pool_address), 52);
    assert_eq!(
        router.get_reserves(&tokens, &pool_hash),
        Vec::from_array(&test.env, [197, 52])
    );

    let mut expected_aqua_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_aqua_result_vec.push_back(97);
    expected_aqua_result_vec.push_back(48);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_aqua_result_vec);

    assert_eq!(result, expected_result);
}



#[test]
fn swap_tokens_for_exact_tokens_constant_product_pool_2_hops() {
    let test = SoroswapAggregatorTest::setup();
    
    let aqua_setup = test.aqua_setup;
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let router = aqua_setup.router;
    let [token1, token2, token3, _] = aqua_setup.tokens;

    let admin = aqua_setup.admin;
    let reward_token = aqua_setup.reward_token;

    let user1 = Address::generate(&test.env);
    reward_token.mint(&user1, &10_0000000);
    test.env.mock_auths(&[]);

    let tokens1 = Vec::from_array(&test.env, [token1.address.clone(), token2.address.clone()]);
    let tokens2 = Vec::from_array(&test.env, [token2.address.clone(), token3.address.clone()]);

    let swapper = Address::generate(&test.env);

    router.mock_all_auths().configure_init_pool_payment(
        &admin,
        &TokenClient::new(
            &test.env,
            &test.env.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    ).address,
        &0,
        &0,
        &router.address,
    );

    let (pool_index1, _pool_address1) = router
        .mock_all_auths()
        .init_standard_pool(&swapper, &tokens1, &30);
    let (pool_index2, _pool_address2) = router
        .mock_all_auths()
        .init_standard_pool(&swapper, &tokens2, &30);
    token1.mock_all_auths().mint(&admin, &10000);
    token2.mock_all_auths().mint(&admin, &20000);
    token3.mock_all_auths().mint(&admin, &10000);
    router.mock_all_auths().deposit(
        &admin,
        &tokens1,
        &pool_index1,
        &Vec::from_array(&test.env, [10000, 10000]),
        &0,
    );
    router.mock_all_auths().deposit(
        &admin,
        &tokens2,
        &pool_index2,
        &Vec::from_array(&test.env, [10000, 10000]),
        &0,
    );

    token1.mock_all_auths().mint(&swapper, &1000);

    

    assert_eq!(token1.balance(&swapper), 1000);
    assert_eq!(token2.balance(&swapper), 0);
    assert_eq!(token3.balance(&swapper), 0);
    assert_eq!(token1.balance(&router.address), 0);
    assert_eq!(token2.balance(&router.address), 0);
    assert_eq!(token3.balance(&router.address), 0);


    let path: Vec<Address> = vec![&test.env,
        token1.address.clone(),
        token2.address.clone(),
        token3.address.clone()];

    // vec pool hash
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env,
        pool_index1.clone(),
        pool_index2.clone()];
    
    let mut distribution_vec = Vec::new(&test.env);
    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: Some(bytes_vec)
    };
    distribution_vec.push_back(distribution_0);

    // let executed_amounts = test.adapter_client.mock_all_auths().swap_tokens_for_exact_tokens(
    //     &96,        // amount_out
    //     &100,        // amount_in_max
    //     &path,     // path
    //     &swapper, // to
    //     &0, // deadline,
    //     &Some(bytes_vec.clone()),
    // );

    let result = test.aggregator_contract.mock_all_auths().swap_tokens_for_exact_tokens(
        &token1.address.clone(),
        &token3.address.clone(),
        &96,        // amount_out
        &100,        // amount_in_max
        &distribution_vec.clone(),
        &swapper.clone(),
        &deadline,
    );
    
    

    assert_eq!(token1.balance(&swapper), 900);
    assert_eq!(token2.balance(&swapper), 0);
    assert_eq!(token3.balance(&swapper), 96);
    assert_eq!(token1.balance(&router.address), 0);
    assert_eq!(token2.balance(&router.address), 0);
    assert_eq!(token3.balance(&router.address), 0);

    
    let mut expected_aqua_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_aqua_result_vec.push_back(100);
    expected_aqua_result_vec.push_back(96);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_aqua_result_vec);

    assert_eq!(result, expected_result);
}

