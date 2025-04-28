use soroban_sdk::testutils::{
    Address as _,
};
use soroban_sdk::{
    Address, 
    vec, Vec, BytesN, Symbol, token::TokenClient, U256, FromVal, IntoVal};
use crate::test::{AquaAggregatorAdapterTest, };
use adapter_interface::AdapterError;
use super::aqua_adapter_contract::AdapterError as AdapterErrorDeployer;
use crate::test::aqua_setup::create_token_contract;


#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client_not_initialized.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
        &None,
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
fn swap_tokens_for_exact_tokens_amount_out_negative() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
        &None,
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = AquaAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
        &None,
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::NegativeNotAllowed))
    );
}


#[test]
fn swap_tokens_for_exact_tokens_missing_hash() {
    let test = AquaAggregatorAdapterTest::setup();

    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &0,            // amount_out
        &0,            // amount_in_max
        &path,         // path
        &test.user,    // to
        &0,            // deadline
        &None,  //     bytes
    );

    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::MissingPoolHashes))
    );
}


#[test]
fn try_swap_tokens_for_exact_tokens_invalid_path_lenght() {
    let test = AquaAggregatorAdapterTest::setup();

    let path: Vec<Address> = vec![&test.env, test.tokens[0].address.clone()];
    // vec with dummy bytes
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, BytesN::from_array(&test.env, &[0; 32])];
    
    let result = test.adapter_client.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0, // deadline
        &Some(bytes_vec),

    );

    //WrongMinimumPathLength
    assert_eq!(
        result,
        Err(Ok(AdapterErrorDeployer::WrongMinimumPathLength))
    );
}


#[test]
// panic with error PoolNotFound = 404,
#[should_panic(expected = "Error(Contract, #404)")]
fn try_swap_tokens_for_exact_tokens_pool_not_found() {
    let test = AquaAggregatorAdapterTest::setup();

    let path: Vec<Address> = vec![&test.env,
        test.tokens[0].address.clone(),
        test.tokens[1].address.clone()];

    // vec with dummy bytes
    let bytes_vec: Vec<BytesN<32>> = vec![&test.env, BytesN::from_array(&test.env, &[0; 32])];
    
    test.env.cost_estimate().budget().reset_unlimited();
    test.adapter_client.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0, // deadline,
        &Some(bytes_vec),
    );
}


#[test]
fn swap_tokens_for_exact_tokens_constant_product_pool_1_hop() {
    let test = AquaAggregatorAdapterTest::setup();
    let deadline: u64 = 0;  

    let router = test.router;
    let [token1, token2, _, _] = test.tokens;

    let tokens = Vec::from_array(&test.env, [token1.address.clone(), token2.address.clone()]);
    let user1 = Address::generate(&test.env);
    test.reward_token.mint(&user1, &10_0000000);

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

    let executed_amounts = test.adapter_client.swap_tokens_for_exact_tokens(
        &48,        // amount_out
        &98,        // amount_in_max
        &path,     // path
        &user1, // to
        &0, // deadline,
        &Some(bytes_vec),
    );

    // error OutMinNotSatisfied = 2006,
    
    assert_eq!(token1.balance(&user1), 803);
    assert_eq!(token1.balance(&pool_address), 197);
    assert_eq!(token2.balance(&user1), 948);
    assert_eq!(token2.balance(&pool_address), 52);
    assert_eq!(
        router.get_reserves(&tokens, &pool_hash),
        Vec::from_array(&test.env, [197, 52])
    );


    assert_eq!(executed_amounts.get(0).unwrap(), 97);
    assert_eq!(executed_amounts.get(1).unwrap(), 48);
}



#[test]
fn swap_tokens_for_exact_tokens_constant_product_pool_2_hops() {
    let test = AquaAggregatorAdapterTest::setup();
    let deadline: u64 = 0;  

    let router = test.router;
    let admin = test.admin;
    let [token1, token2, token3, _] = test.tokens;
    let reward_token = test.reward_token;

    let user1 = Address::generate(&test.env);
    reward_token.mint(&user1, &10_0000000);
    test.env.mock_auths(&[]);

    let tokens1 = Vec::from_array(&test.env, [token1.address.clone(), token2.address.clone()]);
    let tokens2 = Vec::from_array(&test.env, [token2.address.clone(), token3.address.clone()]);

    let swapper = Address::generate(&test.env);

    router.mock_all_auths().configure_init_pool_payment(
        &admin,
        &create_token_contract(&test.env, &admin).address,
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


    let executed_amounts = test.adapter_client.mock_all_auths().swap_tokens_for_exact_tokens(
        &96,        // amount_out
        &100,        // amount_in_max
        &path,     // path
        &swapper, // to
        &0, // deadline,
        &Some(bytes_vec.clone()),
    );

    

    assert_eq!(token1.balance(&swapper), 900);
    assert_eq!(token2.balance(&swapper), 0);
    assert_eq!(token3.balance(&swapper), 96);
    assert_eq!(token1.balance(&router.address), 0);
    assert_eq!(token2.balance(&router.address), 0);
    assert_eq!(token3.balance(&router.address), 0);

    
    assert_eq!(executed_amounts.get(0).unwrap(), 100);
    assert_eq!(executed_amounts.get(1).unwrap(), 96);
}
