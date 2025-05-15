#[cfg(test)]
use soroban_sdk::{Address, vec, Vec};
use crate::test::{PhoenixAggregatorAdapterTest};
use test_utils::phoenix_setup::deploy_and_initialize_lp;
use adapter_interface::AdapterError;
use super::phoenix_adapter_contract::AdapterError as AdapterErrorDeployer;

#[test]
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client_not_initialized.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
        &None
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
fn swap_exact_tokens_for_tokens_amount_in_negative() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &-1,           // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
        &None
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_amount_out_min_negative() {
    let test = PhoenixAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &-1,           // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
        &None
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_expired() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
        &None
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::DeadlineExpired))
    );
}



#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_invalid_path() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    test.adapter_client.swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
        &None
    );
}

#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_insufficient_input_amount() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.env.cost_estimate().budget().reset_unlimited();
    test.adapter_client.swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
        &None
    );
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientInputAmount)));
}



#[test]
// #[should_panic] // TODO: Change to an error object (If we dont delete this check)
// TODO: Check why it fails with the adapter_client?
// #[should_panic(expected = "Amount of token out received is less than the minimum amount expected")]
#[should_panic(expected = "HostError: Error(WasmVm, InvalidAction)")] //TODO: Why it changed using the deployer?
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = PhoenixAggregatorAdapterTest::setup();
    // test.adapter_client_not_initialized.initialize(
    //     &String::from_str(&test.env, "phoenix"),
    //     &test.multihop_client.address);

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    path.push_back(test.token_3.address.clone());

    let amount_in = 50i128;
    // The next taken from phoenix contract tests
    let expected_amount_out = 50i128;

    test.env.cost_estimate().budget().reset_unlimited();
    test.adapter_client.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out + 1),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
        &None
    );
}



#[test]
fn swap_exact_tokens_for_tokens_enough_output_amount() {
    let test = PhoenixAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);

    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    path.push_back(test.token_3.address.clone());

    let amount_in = 500i128;
    // The next taken from phoenix contract tests
    // TODO: Check with future versions of phoenix
    let expected_amount_out = 500i128;

    let initial_user_balance_0 = test.token_0.balance(&test.user);
    let initial_user_balance_1 = test.token_1.balance(&test.user);
    let initial_user_balance_2 = test.token_2.balance(&test.user);
    let initial_user_balance_3 = test.token_3.balance(&test.user);

    let token_out_address = path.get(path.len() - 1).expect("Failed to get token out address");

    assert_eq!(token_out_address, test.token_3.address);
    
    test.env.cost_estimate().budget().reset_unlimited();
    let executed_amounts = test.adapter_client.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
        &None
    );


    assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0 - amount_in);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance_1);
    assert_eq!(test.token_2.balance(&test.user), initial_user_balance_2);
    assert_eq!(test.token_3.balance(&test.user), initial_user_balance_3 + expected_amount_out);

    // WE NEED TO RETURN THE VALUES
    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
}



#[test]
fn swap_exact_tokens_for_tokens_enough_output_amount_with_fees() {
    let test = PhoenixAggregatorAdapterTest::setup();

    // we will make a pool betwern token 0 and token 2 with fees
    deploy_and_initialize_lp(
        &test.env,
        &test.factory_client,
        test.admin.clone(),
        test.token_0.address.clone(),
        1_000_000,
        test.token_2.address.clone(),
        1_000_000,
        Some(2000),
    );


    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);

    path.push_back(test.token_0.address.clone());
    // path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    // path.push_back(test.token_3.address.clone());

    let amount_in = 300i128;
    // The next taken from phoenix contract tests
    // TODO: Check with future versions of phoenix
    // 1000 tokens initially
    // swap 300 from token0 to token1 with 2000 bps (20%)
    // tokens1 will be 240
    let expected_amount_out = 240i128;

    let initial_user_balance_0 = test.token_0.balance(&test.user);
    // let initial_user_balance_1 = test.token_1.balance(&test.user);
    let initial_user_balance_2 = test.token_2.balance(&test.user);
    // let initial_user_balance_3 = test.token_3.balance(&test.user);
    
    test.env.cost_estimate().budget().reset_unlimited();
    let executed_amounts = test.adapter_client.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
        &None
    );


    assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0 - amount_in);
    assert_eq!(test.token_2.balance(&test.user), initial_user_balance_2 + expected_amount_out);

    // WE NEED TO RETURN THE VALUES
    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
}


// use crate::factory_contract::PoolType;
// // FIXM: Disable Referral struct
// // use crate::lp_contract::Referral;
// use crate::storage::Swap;
// use crate::tests::setup::{
//     deploy_and_initialize_factory, deploy_and_initialize_pool, deploy_and_mint_tokens,
//     deploy_multihop_contract, deploy_token_contract,
// };

// use soroban_sdk::contracterror;
// use soroban_sdk::{testutils::Address as _, vec, Address, Env};

// #[contracterror]
// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// #[repr(u32)]
// pub enum ContractError {
//     SpreadExceedsLimit = 1,
// }

// #[test]
// fn swap_three_equal_pools_no_fees() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         1_000_000,
//         token3.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         1_000_000,
//         token4.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &50i128);
//     assert_eq!(token1.balance(&recipient), 50i128);
//     assert_eq!(token4.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1, swap2, swap3];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &operations, &None, &None, &50i128);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &50i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // 5. check if it goes according to plan
//     assert_eq!(token1.balance(&recipient), 0i128);
//     assert_eq!(token4.balance(&recipient), 50i128);
// }

// // FIXM: Disable Referral struct
// #[ignore]
// #[test]
// fn swap_three_equal_pools_no_fees_referral_fee() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         1_000_000,
//         token3.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         1_000_000,
//         token4.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &50i128);
//     assert_eq!(token1.balance(&recipient), 50i128);
//     assert_eq!(token4.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1, swap2, swap3];
//     let referral_addr = Address::generate(&env);
//     // FIXM: Disable Referral struct
//     // let referral = Referral {
//     //     address: referral_addr.clone(),
//     //     fee: 1_000,
//     // };

//     // multihop.swap(
//     //     &recipient,
//     //     &Some(referral),
//     //     &operations,
//     //     &None,
//     //     &None,
//     //     &50i128,
//     // );
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &50i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // 5. check if it goes according to plan
//     assert_eq!(token1.balance(&recipient), 0i128);
//     assert_eq!(token4.balance(&recipient), 37i128);
//     // referral fee from first swap should be 5 (10% out of 50)
//     assert_eq!(token2.balance(&referral_addr), 5i128);
//     // referral fee from 2nd swap should be 4 (10% out of 45) rounded down
//     assert_eq!(token3.balance(&referral_addr), 4i128);
//     // referral fee from the last swap should also be 4 (10% out of 41) rounded down
//     assert_eq!(token4.balance(&referral_addr), 4i128);
// }

// #[test]
// fn swap_single_pool_no_fees() {
//     let env = Env::default();
//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &5_000i128); // mints 50 token0 to recipient
//     assert_eq!(token1.balance(&recipient), 5_000i128);
//     assert_eq!(token2.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, /*&None,*/ &operations, &None, &None, &50i128);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &1_000,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // 5. check if it goes according to plan
//     assert_eq!(token1.balance(&recipient), 4_000i128); // -1_000 token0
//     assert_eq!(token2.balance(&recipient), 1_000i128); // +1_000 token1
// }

// #[test]
// /// Asserting HostError, because of panic messages are not propagated and IIUC are normally compiled out
// #[should_panic(expected = "HostError: Error(Contract, #1)")]
// fn swap_should_fail_when_spread_exceeds_the_limit() {
//     let env = Env::default();
//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 3_001_000i128);

//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         5_000,
//         token2.address.clone(),
//         2_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &5_000i128); // mints 50 token0 to recipient

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &operations, &None, &Some(50), &50);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &Some(50),
//         &50,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );
// }

// #[test]
// fn swap_single_pool_with_fees() {
//     let env = Env::default();
//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         Some(2000),
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &1000i128);
//     assert_eq!(token1.balance(&recipient), 1000i128);
//     assert_eq!(token2.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &operations, &None, &None, &300i128);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &300i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // 5. check if it goes according to plan
//     // 1000 tokens initially
//     // swap 300 from token0 to token1 with 2000 bps (20%)
//     // tokens1 will be 240
//     assert_eq!(token1.balance(&recipient), 700i128);
//     assert_eq!(token2.balance(&recipient), 240i128);
// }

// #[test]
// fn swap_three_different_pools_no_fees() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         2_000_000,
//         token3.address.clone(),
//         2_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         3_000_000,
//         token4.address.clone(),
//         3_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &5_000i128);

//     assert_eq!(token1.balance(&recipient), 5_000i128);
//     assert_eq!(token4.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1, swap2, swap3];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &operations, &None, &None, &5_000i128);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &5_000i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // 5. check if it goes according to plan
//     assert_eq!(token1.balance(&recipient), 0i128);
//     assert_eq!(
//         token4.balance(&recipient),
//         4_956i128,
//         "token4 not as expected"
//     );
// }

// #[test]
// fn swap_three_different_pools_with_fees() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         Some(1_000),
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         2_000_000,
//         token3.address.clone(),
//         2_000_000,
//         Some(1_000),
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         3_000_000,
//         token4.address.clone(),
//         3_000_000,
//         Some(1_000),
//         PoolType::Xyk,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &10_000i128);
//     assert_eq!(token1.balance(&recipient), 10_000i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1, swap2, swap3];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &operations, &None, &None, &10_000i128);
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &10_000i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     // we start swapping 10_000 tokens

//     // token1 => token2
//     // (10_000 * 1_000_000) / (10_000 + 1_000_000)
//     // 10_000_000_000 / 1_010_000
//     // 9900.99009901
//     // 9901 - 10% =  8911

//     // token2 => token3
//     // (8911 * 2_000_000) / (8911 + 2_000_000)
//     // 17_822_000_000 / 2_008_911
//     // 8871.47315137
//     // 8872 - 10% = 7985

//     // token3 => token4
//     // (7985 * 3_000_000) / (7985 + 3_000_000)
//     // 23_955_000_000 / 3_007_985
//     // 7963.80301099
//     // 7964 - 10% = 7168
//     assert_eq!(token1.balance(&recipient), 0i128);
//     assert_eq!(token2.balance(&recipient), 0i128);
//     assert_eq!(token3.balance(&recipient), 0i128);
//     assert_eq!(token4.balance(&recipient), 7_168i128);
// }

// #[test]
// #[should_panic(expected = "Multihop: Swap: operations is empty!")]
// fn swap_panics_with_no_operations() {
//     let env = Env::default();
//     env.mock_all_auths();
//     let admin = Address::generate(&env);
//     let factory = Address::generate(&env);
//     let recipient = Address::generate(&env);

//     let token = deploy_token_contract(&env, &admin);
//     token.mint(&recipient, &50i128);

//     let multihop = deploy_multihop_contract(&env, admin, &factory);

//     let swap_vec = vec![&env];

//     // FIXM: Disable Referral struct
//     // multihop.swap(&recipient, &None, &swap_vec, &None, &None, &50i128);
//     multihop.swap(
//         &recipient,
//         &swap_vec,
//         &None,
//         &50i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );
// }

// #[test]
// fn test_v_phx_vul_013_add_belief_price_for_every_swap() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         2_000_000,
//         token3.address.clone(),
//         2_000_000,
//         None,
//         PoolType::Xyk,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         3_000_000,
//         token4.address.clone(),
//         3_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &5_000i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: Some(1_050),
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: Some(2_100),
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: Some(3_150),
//     };

//     let operations = vec![&env, swap1, swap2, swap3];

//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &5_000i128,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     assert_eq!(
//         token1.balance(&recipient),
//         0i128,
//         "token1 balance incorrect"
//     );
//     assert_eq!(
//         token4.balance(&recipient),
//         4_956i128,
//         "token4 balance incorrect"
//     );
// }

// #[test]
// #[should_panic(expected = "Error(Contract, #21)")]
// fn test_swap_with_ask_asset_min_amount() {
//     let env = Env::default();
//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 1_001_000i128);

//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Xyk,
//     );

//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &5_000i128);
//     assert_eq!(token1.balance(&recipient), 5_000i128);
//     assert_eq!(token2.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: Some(1_000),
//     };

//     let operations = vec![&env, swap1];

//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &1_000,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );

//     assert_eq!(token1.balance(&recipient), 4_000i128);
//     assert_eq!(token2.balance(&recipient), 1_000i128);

//     let greedy_swap = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: Some(10_000),
//     };
//     let operations = vec![&env, greedy_swap];
//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &1_000,
//         &PoolType::Xyk,
//         &None::<u64>,
//     );
// }

// #[test]
// #[ignore = "fails with NewtonMethodFailed "]
// fn swap_three_equal_stable_pool() {
//     let env = Env::default();

//     let admin = Address::generate(&env);

//     env.mock_all_auths();
//     env.cost_estimate().budget().reset_unlimited();

//     let token1 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token2 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token3 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);
//     let token4 = deploy_and_mint_tokens(&env, &admin, 10_000_000i128);

//     // 1. deploy factory
//     let factory_client = deploy_and_initialize_factory(&env, admin.clone());

//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token1.address.clone(),
//         1_000_000,
//         token2.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Stable,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token2.address.clone(),
//         1_000_000,
//         token3.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Stable,
//     );
//     deploy_and_initialize_pool(
//         &env,
//         &factory_client,
//         admin.clone(),
//         token3.address.clone(),
//         1_000_000,
//         token4.address.clone(),
//         1_000_000,
//         None,
//         PoolType::Stable,
//     );

//     // 4. swap with multihop
//     let multihop = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
//     let recipient = Address::generate(&env);
//     token1.mint(&recipient, &50i128);
//     assert_eq!(token1.balance(&recipient), 50i128);
//     assert_eq!(token4.balance(&recipient), 0i128);

//     let swap1 = Swap {
//         offer_asset: token1.address.clone(),
//         ask_asset: token2.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap2 = Swap {
//         offer_asset: token2.address.clone(),
//         ask_asset: token3.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };
//     let swap3 = Swap {
//         offer_asset: token3.address.clone(),
//         ask_asset: token4.address.clone(),
//         ask_asset_min_amount: None::<i128>,
//     };

//     let operations = vec![&env, swap1, swap2, swap3];

//     multihop.swap(
//         &recipient,
//         &operations,
//         &None,
//         &50i128,
//         &PoolType::Stable,
//         &None::<u64>,
//     );

//     assert_eq!(token1.balance(&recipient), 0i128);
//     assert_eq!(token4.balance(&recipient), 50i128);
// }

// #[test]
// fn swap_exact_tokens_for_tokens_2_hops() {
//     let test = PhoenixAggregatorAdapterTest::setup();
//     test.env.cost_estimate().budget().reset_unlimited();
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

