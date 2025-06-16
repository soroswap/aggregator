extern crate std;
use crate::error::AggregatorError as AggregatorErrorFromCrate;
use crate::test::SoroswapAggregatorTest;
// use crate::DexDistribution;
use soroban_sdk::{Address, Vec, vec, BytesN, Symbol};
use soroban_sdk::token::TokenClient;
use soroban_sdk::U256;
use soroban_sdk::testutils::{
    Address as _,
};
use soroban_sdk::FromVal;



use super::soroswap_aggregator_contract::{AggregatorError, DexDistribution};
use super::soroswap_aggregator_contract::Protocol;

// use soroban_sdk::{
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
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract_not_initialized.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_negative_amount_in_soroswap() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &-1,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    assert_eq!(result, Err(Ok(AggregatorError::NegibleAmount)));

}

#[test]
fn swap_exact_tokens_for_tokens_negative_amount_in_aqua() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    // call the function
        
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &-1,
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    assert_eq!(result, Err(Ok(AggregatorError::NegibleAmount)));
}


#[test]
fn swap_exact_tokens_for_tokens_negible_amount() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
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
    Amount in for route 0 will be
    1000 * (parts) / total parts
    1000 * 1 / 1001 = 0
    */

    // This should fail with NegibleAmountError

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &1000, //amount_in
        &100,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );

    assert_eq!(result, Err(Ok(AggregatorError::NegibleAmount)));

}


// We will allow `amount_out_min` to be negative in `swap_exact_tokens_for_tokens_negative_amount_out_min`.
// Calling `swap_exact_tokens_for_tokens_negative_amount_out_min` with `amount_out_min` negative
// is is similar of calling it with `amount_out_min=0`.
// This is because then, the Aggregator checks

// if final_amount_out < amount_out_min {
//     return Err(AggregatorError::InsufficientOutputAmount);
// }

// #[test]
// #[should_panic(expected = "HostError: Error(Contract, #502)")] //Negative not allowed
// fn swap_exact_tokens_for_tokens_negative_amount_out_min() {
//     // creat the test
//     let test = SoroswapAggregatorTest::setup();
//     let mut distribution_vec = Vec::new(&test.env);
    
//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     let distribution_0 = DexDistribution {
//         protocol_id: Protocol::Soroswap,
//         path,
//         parts: 1,
//         bytes: None
//     };
//     distribution_vec.push_back(distribution_0);
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;

//     test.aggregator_contract.swap_exact_tokens_for_tokens(
//         &test.token_0.address.clone(), // token_in
//         &test.token_1.address.clone(), // token_out
//         &100, // amount_in
//         &-1, // amount_out_min
//         &distribution_vec,
//         &test.user.clone(),
//         &deadline,
//     );
// }

#[test]
#[should_panic(expected = "HostError: Error(Contract, #503)")]
fn swap_exact_tokens_for_tokens_deadline_expired() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    
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

    test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &100,
        &0,
        &distribution_vec,
        &test.user.clone(),
        &0,
    );
}

#[test]
fn swap_exact_tokens_for_tokens_distribution_over_max() {
    // creat the test
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_zero_parts() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
// fn swap_exact_tokens_for_tokens_protocol_not_found() {
//     let test = SoroswapAggregatorTest::setup();
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;
//     // Initialize aggregator
//     // let initialize_aggregator_addresses = create_protocols_addresses(&test);
//     // test.aggregator_contract_not_initialized
//     //     .initialize(&test.admin, &initialize_aggregator_addresses);
//     // call the function
//     let mut distribution_vec = Vec::new(&test.env);
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

//     let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_paused_protocol() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    // pause the protocol
    test.aggregator_contract
        .set_pause(&Protocol::Soroswap, &true);

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn swap_exact_tokens_for_tokens_malformed_path_wrong_start() {
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
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out),
        &distribution_vec,                      
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InvalidPath)));
}


#[test]
fn swap_exact_tokens_for_tokens_malformed_path_wrong_end() {
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
        path,
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out),
        &distribution_vec,                      
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InvalidPath)));
}

#[test]
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &amount_in,
        &(expected_amount_out + 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InsufficientOutputAmount)));
}

#[test]
fn swap_exact_tokens_for_tokens_succeed_correctly_one_protocol() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
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

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    // check initial user balance of both tokens
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);

    //  MOCK THE SPECIFIC AUTHORIZATION
    // TODO: solve the sub invokes to to the mock auth corectly
    let result = test
        .aggregator_contract
        // .mock_auths(&[
        //     MockAuth {
        //         router: &test.user.clone(),
        //         invoke:
        //             &MockAuthInvoke {
        //                 contract: &test.aggregator_contract_not_initialized.address.clone(),
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
            &deadline,
        );

    //     // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    //     assert_eq!(
    //         test.env.auths(),
    //         std::vec![(
    //             test.user.clone(),
    //             AuthorizedInvocation {
    //                function: AuthorizedFunction::Contract((
    //                    test.aggregator_contract_not_initialized.address.clone(),
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

    // let result = test.aggregator_contract_not_initialized.swap_exact_tokens_for_tokens(
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
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + expected_amount_out
    );

    // check the result vec
    // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in);
    expected_soroswap_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);

    assert_eq!(result, expected_result);
}

#[test]
fn swap_exact_tokens_for_tokens_succeed_correctly_one_protocol_two_hops() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
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

    let amount_in = 123_456_789;
    // fee = 123456789 * 3 /1000 =  370370,367 = 370371 // USE CEILING
    // amount_in less fee = 123456789- 370371 = 123086418
    // First out = (123086418*4000000000000000000)/(1000000000000000000 + 123086418) = 492345671.939398935 = 492345671
    let first_out = 492345671;
    // fee = 492345671 * 3 /1000 =  1477037.013 = 1477038 // USE CEILING
    // in less fee = 492345671 - 1477038 = 490868633
    // Second out = (490868633*8000000000000000000)/(4000000000000000000 + 490868633) = 981737265.879523993 = 981737265
    let expected_amount_out = 981737265;

    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_2.address.clone(),
        &amount_in,
        &(0),
        &distribution_vec.clone(),
        &test.user.clone(),
        &deadline,
    );

    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);

    // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in);
    assert_eq!(user_balance_after_1, user_balance_before_1);
    assert_eq!(
        user_balance_after_2,
        user_balance_before_2 + expected_amount_out
    );

    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in);
    expected_soroswap_result_vec.push_back(first_out);
    expected_soroswap_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);

    assert_eq!(result, expected_result);
}

#[test]
fn swap_exact_tokens_for_tokens_succeed_correctly_same_protocol_twice() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
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

    let total_expected_amount_in = 123_456_789;

    // The total expected amount will come from 2 different trades:
    let expected_amount_in_0 = 123_456_789_i128
        .checked_div(4)
        .unwrap()
        .checked_mul(1)
        .unwrap();
    let expected_amount_in_1 = total_expected_amount_in - expected_amount_in_0;

    // reserve_0 = 1_000_000_000_000_000_000;
    // reserve_1 = 4_000_000_000_000_000_000;

    // expected_amount_in_0 = 30864197
    // expected_amount_in_1 = 92592592

    // swap 0
    // fee = ceil(30864197 * 3 /1000) =  92592.591 = 92593 // USE CEILING
    // amount_in less fee = 30864197- 92593 = 30771604
    // out = (amount_in_less_fees*reserve_1)/(reserve_0 + amount_in_less_fees) =
    // First out = (30771604*4000000000000000000)/(1000000000000000000 + 30771604) =
    // 123086416000000000000000000 / 1000000000030771604 = 123086415.996212434 = 123086415 // no ceiling div
    let expected_amount_out_0 = 123086415;

    // swap 1 happens with new reserves
    // reserve_0 = 1_000_000_000_000_000_000 + 30864197 =
    // 1000000000000000000 + 30864197 = 1000000000030864197
    // reserve_1 = 4_000_000_000_000_000_000 - 123086415 =
    // 4000000000000000000 - 123086415 = 3999999999876913585

    // fee = ceil(92592592 * 3 /1000) =  277777.776 = 277778 // USE CEILING
    // amount_in less fee = 92592592- 277778 = 92314814
    // out = (amount_in_less_fees*reserve_1)/(reserve_0 + amount_in_less_fees) =
    // Second out = (92314814*3999999999876913585)/(1000000000030864197 + 92314814) =
    // 369259255988637300493348190 / 1000000000123179011 = 369259255.943152311 = 369259255 // no ceiling div
    let expected_amount_out_1 = 369259255;

    let total_expected_amount_out = expected_amount_out_0 + expected_amount_out_1;

    // if we just expect one unit more of the expected amount out, the function should fail with expected error
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &(total_expected_amount_out + 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InsufficientOutputAmount)));

    // check balance before
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);

    // if we expect the exact amount out, the function should succeed
    let success_result = test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &total_expected_amount_out,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );

    // check balance after
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);

    // compare
    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - total_expected_amount_in
    );
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + total_expected_amount_out
    );

    // check the result vec
    // the result vec in this case is a vec of 2 vecs with two elements, the amount 0 and amount 1
    let mut expected_soroswap_result_vec_0: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec_0.push_back(expected_amount_in_0);
    expected_soroswap_result_vec_0.push_back(expected_amount_out_0);

    let mut expected_soroswap_result_vec_1: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec_1.push_back(expected_amount_in_1);
    expected_soroswap_result_vec_1.push_back(expected_amount_out_1);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec_0);
    expected_result.push_back(expected_soroswap_result_vec_1);

    assert_eq!(success_result, expected_result);
}
#[test]
fn swap_exact_tokens_for_tokens_succeed_correctly_two_protocols() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    // Initialize aggregator
    // let initialize_aggregator_addresses = create_soroswap_phoenix_addresses(&test);

    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);

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
        protocol_id: Protocol::Phoenix,
        path: path.clone(),
        parts: 3,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_in = 123_456_789;

    // The total expected amount will come from 2 different trades:
    // 123_456_789_i128
    //     .checked_div(4)
    //     .unwrap()
    //     .checked_mul(1)
    //     .unwrap();
    let expected_amount_in_0 = 30864197;
    let expected_amount_in_1 = 92592592;// total_expected_amount_in - expected_amount_in_0;

    // FOR SOROSWAP:
    // reserve_0 = 1_000_000_000_000_000_000;
    // reserve_1 = 4_000_000_000_000_000_000;

    // expected_amount_in_0 = 30864197
    // expected_amount_in_1 = 92592592

    // swap 0
    // fee = ceil(30864197 * 3 /1000) =  92592.591 = 92593 // USE CEILING
    // amount_in less fee = 30864197- 92593 = 30771604
    // out = (amount_in_less_fees*reserve_1)/(reserve_0 + amount_in_less_fees) =
    // First out = (30771604*4000000000000000000)/(1000000000000000000 + 30771604) =
    // 123086416000000000000000000 / 1000000000030771604 = 123086415.996212434 = 123086415 // no ceiling div
    let expected_amount_out_0 = 123086415;

    // FOR PHOENIX WE EXPECT OUT THE SAME AS IN
    let expected_amount_out_1 = 92592592;

    let total_expected_amount_out = expected_amount_out_0 + expected_amount_out_1;

    // if we just expect one unit more of the expected amount out, the function should fail with expected error
    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &(total_expected_amount_out + 1),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    // compare the error
    assert_eq!(result, Err(Ok(AggregatorError::InsufficientOutputAmount)));

    // check balance before
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);

    // if we expect the exact amount out, the function should succeed
    let success_result = test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &total_expected_amount_out,
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );

    // check balance after
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);

    // compare
    assert_eq!(
        user_balance_after_0,
        user_balance_before_0 - total_expected_amount_in
    );
    assert_eq!(
        user_balance_after_1,
        user_balance_before_1 + total_expected_amount_out
    );

    // check the result vec
    // the result vec in this case is a vec of 2 vecs with two elements, the amount 0 and amount 1
    let mut expected_soroswap_result: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result.push_back(expected_amount_in_0);
    expected_soroswap_result.push_back(expected_amount_out_0);

    let mut expected_phoenix_result: Vec<i128> = Vec::new(&test.env);
    expected_phoenix_result.push_back(expected_amount_in_1);
    expected_phoenix_result.push_back(expected_amount_out_1);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result);
    expected_result.push_back(expected_phoenix_result);

    assert_eq!(success_result, expected_result);
}


#[test]
fn swap_exact_tokens_for_tokens_succeed_comet() {
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

    let amount_in = 1_000_000;
    // bone = 10**18
    // fee_ratio = (10**7 - 30000) * 10**11 => 997000000000000000
    // scaled_reserve_(out|in) = token_(out|in)_reserve * 10**7
    // adjusted_in = amount_in * fee_ratio / BONE
    // base = (scaled_reserve_in * BONE) / (scaled_reserve_in) + adjusted_in) 
    // weight_ratio = in_token_weight * 10**18 / out_token_weight
    // power = ((base / BONE) ** (weight_ratio / BONE)) * BONE // The code treats the numbers as 18 digit fixed point values. So code does it differently, but this is equivelant
    // balance_ratio = BONE - power
    // <= scaled_reserve_out * balance_ratio / BONE / 10**7

    // scaled_reserve_in = 800000000000 * 10**7 => 8000000000000000000
    // scaled_reserve_out = 200000000000 * 10**7 => 2000000000000000000
    // adjusted_in = 1_000_000 * 997000000000000000 / BONE => 99700000000000000
    // base = (8000000000000000000 * BONE) / (8000000000000000000 + 99700000000000000) => 999998753751553137
    // weight_ratio = 8000000 * bone / 2000000 => 4000000000000000000
    // power = ((999998753751553137 / BONE) ** (4000000000000000000 / BONE)) * 10**18 => 999995015015531351
    // balance_ratio = BONE - 999995015015531351 => 4984984468649
    // 2000000000000000000 * 4984984468649 / BONE / 10**7 => 996996
    let expected_amount_out = 996996;

    // check initial user balance of both tokens
    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test
        .aggregator_contract
        .swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_in,
            &(expected_amount_out),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

    // check new user balances
    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);
    // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in);
    assert_eq!(
        user_balance_after_2,
        user_balance_before_2 + expected_amount_out
    );

    // check the result vec
    // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    let mut expected_comet_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_comet_result_vec.push_back(amount_in);
    expected_comet_result_vec.push_back(expected_amount_out);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_comet_result_vec);

    assert_eq!(result, expected_result);
}

#[test]
fn swap_exact_tokens_for_tokens_succeed_comet_soroswap_two_hops() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    // call the function
    let mut distribution_vec = Vec::new(&test.env);
    
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

    let mut path_comet: Vec<Address> = Vec::new(&test.env);
    path_comet.push_back(test.token_0.address.clone());
    path_comet.push_back(test.token_2.address.clone());

    let distribution_1 = DexDistribution {
        protocol_id: Protocol::Comet,
        path: path_comet,
        parts: 1,
        bytes: None
    };

    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    // let initial_user_balance: i128 = 20_000_000_000_000_000_000;
    // let amount_0: i128 = 1_000_000_000_000_000_000;
    // let amount_1: i128 = 4_000_000_000_000_000_000;
    // let amount_2: i128 = 8_000_000_000_000_000_000;

    let amount_in = 2_000_000;


    let amount_in_soroswap = 1_000_000;
    // fee = 1_000_000 * 3 /1000 =  3000 // USE CEILING
    // amount_in less fee = 1_000_000 - 3000 = 997000
    // First out = (997000*4000000000000000000)/(1000000000000000000 + 997000) = 3987999.999996024 = 3987999
    let first_out = 3987999;
    // fee = 3987999 * 3 /1000 =  11963.997 = 11964 // USE CEILING
    // in less fee = 3987999 - 11964 = 3976035
    // Second out = (3976035*8000000000000000000)/(4000000000000000000 + 3976035) = 7952069.999992096 = 7952069
    let expected_amount_out_soroswap = 7952069;

    let amount_in_comet = 1_000_000;
    // bone = 10**18
    // fee_ratio = (10**7 - 30000) * 10**11 => 997000000000000000
    // scaled_reserve_(out|in) = token_(out|in)_reserve * 10**7
    // adjusted_in = amount_in * fee_ratio / BONE
    // base = (scaled_reserve_in * BONE) / (scaled_reserve_in) + adjusted_in) 
    // weight_ratio = in_token_weight * 10**18 / out_token_weight
    // power = ((base / BONE) ** (weight_ratio / BONE)) * BONE // The code treats the numbers as 18 digit fixed point values. So code does it differently, but this is equivelant
    // balance_ratio = BONE - power
    // <= scaled_reserve_out * balance_ratio / BONE / 10**7

    // scaled_reserve_in = 800000000000 * 10**7 => 8000000000000000000
    // scaled_reserve_out = 200000000000 * 10**7 => 2000000000000000000
    // adjusted_in = 1_000_000 * 997000000000000000 / BONE => 99700000000000000
    // base = (8000000000000000000 * BONE) / (8000000000000000000 + 99700000000000000) => 999998753751553137
    // weight_ratio = 8000000 * bone / 2000000 => 4000000000000000000
    // power = ((999998753751553137 / BONE) ** (4000000000000000000 / BONE)) * 10**18 => 999995015015531351
    // balance_ratio = BONE - 999995015015531351 => 4984984468649
    // 2000000000000000000 * 4984984468649 / BONE / 10**7 => 996996
    let expected_amount_out_comet = 996996;

    let expected_out = expected_amount_out_comet + expected_amount_out_soroswap;

    let user_balance_before_0 = test.token_0.balance(&test.user);
    let user_balance_before_1 = test.token_1.balance(&test.user);
    let user_balance_before_2 = test.token_2.balance(&test.user);

    let result = test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_2.address.clone(),
        &amount_in,
        &(0),
        &distribution_vec.clone(),
        &test.user.clone(),
        &deadline,
    );

    let user_balance_after_0 = test.token_0.balance(&test.user);
    let user_balance_after_1 = test.token_1.balance(&test.user);
    let user_balance_after_2 = test.token_2.balance(&test.user);

    // compare
    assert_eq!(user_balance_after_0, user_balance_before_0 - amount_in);
    assert_eq!(user_balance_after_1, user_balance_before_1);
    assert_eq!(
        user_balance_after_2,
        user_balance_before_2 + expected_out
    );

    let mut expected_soroswap_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_soroswap_result_vec.push_back(amount_in_soroswap);
    expected_soroswap_result_vec.push_back(first_out);
    expected_soroswap_result_vec.push_back(expected_amount_out_soroswap);

    let mut expected_comet_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_comet_result_vec.push_back(amount_in_comet);
    expected_comet_result_vec.push_back(expected_amount_out_comet);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_soroswap_result_vec);
    expected_result.push_back(expected_comet_result_vec);

    assert_eq!(result, expected_result);
}

// AQUA

#[test]
fn swap_exact_tokens_for_tokens_missing_hash_aqua() {
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

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
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
fn try_swap_exact_tokens_for_tokens_invalid_bytes_lenght_aqua() {
    let test = SoroswapAggregatorTest::setup();
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut distribution_vec = Vec::new(&test.env);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, BytesN::from_array(&test.env, &[0; 32])];
    
    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: Some(bytes_vec)
    };
    distribution_vec.push_back(distribution_0);

    let amount_in = 1_000_000;
    let expected_amount_out = 996996;

    let result = test.aggregator_contract.try_swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_in,
            &(expected_amount_out),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

    assert_eq!(result, Err(Ok(AggregatorError::WrongPoolHashesLength)));
}

#[test]
// panic with error PoolNotFound = 404,
#[should_panic(expected = "Error(Contract, #404)")]
fn try_swap_tokens_for_exact_tokens_pool_not_found_aqua() {
    let test = SoroswapAggregatorTest::setup();
   
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut distribution_vec = Vec::new(&test.env);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    // vec with dummy bytes
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, BytesN::from_array(&test.env, &[0; 32])];

    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: Some(bytes_vec)
    };
    distribution_vec.push_back(distribution_0);
    
    test.env.cost_estimate().budget().reset_unlimited();
    
    let amount_in = 1_000_000;
    let expected_amount_out = 996996;

    test.aggregator_contract.swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_1.address.clone(),
            &amount_in,
            &(expected_amount_out),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );

}


#[test]
fn swap_exact_tokens_for_tokens_constant_product_pool_1_hop_aqua() {
    let test = SoroswapAggregatorTest::setup();
    let aqua_setup = test.aqua_setup;
    let deadline: u64 = test.env.ledger().timestamp() + 1000;


    let router = aqua_setup.router;

    let tokens = Vec::from_array(&test.env, [test.token_0.address.clone(), test.token_1.address.clone()]);
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

    test.token_0.mint(&user1, &1000);
    assert_eq!(test.token_0.balance(&user1), 1000);

    test.token_1.mint(&user1, &1000);
    assert_eq!(test.token_1.balance(&user1), 1000);

    assert_eq!(token_share.balance(&user1), 0);

    let desired_amounts = Vec::from_array(&test.env, [100, 100]);
    router.deposit(&user1, &tokens, &pool_hash, &desired_amounts, &0);
    assert_eq!(router.get_total_liquidity(&tokens), U256::from_u32(&test.env, 2));

    assert_eq!(token_share.balance(&user1), 100);
    assert_eq!(router.get_total_shares(&tokens, &pool_hash), 100);
    assert_eq!(token_share.balance(&pool_address), 0);
    assert_eq!(test.token_0.balance(&user1), 900);
    assert_eq!(test.token_0.balance(&pool_address), 100);
    assert_eq!(test.token_1.balance(&user1), 900);
    assert_eq!(test.token_1.balance(&pool_address), 100);

    assert_eq!(
        router.get_reserves(&tokens, &pool_hash),
        Vec::from_array(&test.env, [100, 100])
    );

    assert_eq!(
        router.estimate_swap(&tokens, &test.token_0.address, &test.token_1.address, &pool_hash, &97),
        48
    );

    // Here we will swap using the adapter instead of directly using the pool:
    // assert_eq!(
    //     router.swap(
    //         &user1,
    //         &tokens,
    //         &test.token_0.address,
    //         &test.token_1.address,
    //         &pool_hash,
    //         &97_u128, // amount_in
    //         &48_u128, // amount_out_min
    //     ),
    //     48
    // );

    let path: Vec<Address> = vec![&test.env,
        test.token_0.address.clone(),
        test.token_1.address.clone()];
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, pool_hash.clone()];

    let mut distribution_vec = Vec::new(&test.env);
    let distribution_0 = DexDistribution {
        protocol_id: Protocol::Aqua,
        path,
        parts: 1,
        bytes: Some(bytes_vec)
    };
    distribution_vec.push_back(distribution_0);


    let result = test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &97,        // amount_in
        &48,        // amount_out_min
        &distribution_vec.clone(),
        &user1.clone(),
        &deadline,
    );

    
    assert_eq!(test.token_0.balance(&user1), 803);
    assert_eq!(test.token_0.balance(&pool_address), 197);
    assert_eq!(test.token_1.balance(&user1), 948);
    assert_eq!(test.token_1.balance(&pool_address), 52);
    assert_eq!(
        router.get_reserves(&tokens, &pool_hash),
        Vec::from_array(&test.env, [197, 52])
    );


    // check the result vec
    // the result vec in this case is a vec of 1 vec with two elements, the amount 0 and amount 1
    let mut expected_aqua_result_vec: Vec<i128> = Vec::new(&test.env);
    expected_aqua_result_vec.push_back(97);
    expected_aqua_result_vec.push_back(48);

    let mut expected_result = Vec::new(&test.env);
    expected_result.push_back(expected_aqua_result_vec);

    assert_eq!(result, expected_result);
}


#[test]
fn swap_exact_tokens_for_tokens_constant_product_pool_2_hops() {
    let test = SoroswapAggregatorTest::setup();
    let aqua_setup = test.aqua_setup;
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let router = aqua_setup.router;
    let admin = aqua_setup.admin;
    let [token1, token2, token3, _] = aqua_setup.tokens;
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

    // swapping token 1 to 3 through combination of 2 pools as we don't have pool (1, 3)
    // Here we will swap using the adapter instead of directly using the pool:

    // let swap_root_args = vec![
    //     &test.env,
    //     swapper.clone().to_val(),
    //     vec![
    //         &test.env,
    //         (tokens1.clone(), pool_index1.clone(), token2.address.clone()),
    //         (tokens2.clone(), pool_index2.clone(), token3.address.clone()),
    //     ]
    //     .into_val(&test.env),
    //     token1.address.clone().clone().to_val(),
    //     100_u128.into_val(&test.env),
    //     96_u128.into_val(&test.env),
    // ];


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


    let result = test.aggregator_contract.mock_all_auths().swap_exact_tokens_for_tokens(
        &token1.address.clone(),
        &token3.address.clone(),
        &100,        // amount_in
        &96,        // amount_out_min
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