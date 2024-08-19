use soroban_sdk::{Address, vec, Vec, String};
use soroban_sdk::testutils::Ledger;
use crate::test::{SoroswapAggregatorAdapterTest};
use adapter_interface::{AdapterError};


#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract.try_swap_tokens_for_exact_tokens(
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
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract.try_swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::NegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::NegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_expired() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(AdapterError::DeadlineExpired))
    );
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_invalid_path() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];

    test.adapter_contract.swap_tokens_for_exact_tokens( // add try_ to test the error
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
fn swap_tokens_for_exact_tokens_pair_does_not_exist() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &0, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_insufficient_output_amount() {
    let test = SoroswapAggregatorAdapterTest::setup();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.budget().reset_unlimited();
    test.adapter_contract.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientOutputAmount)));
}

#[test]
#[should_panic] // TODO: Test the imported error
fn swap_tokens_for_exact_tokens_amount_in_max_not_enough() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let expected_amount_out = 5_000_000;

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, // amount_out
        &0,                   // amount_in_max
        &path,                // path
        &test.user,           // to
        &deadline,            // deadline
    );

    // assert_eq!(
    //     result,
    //     Err(Ok(CombinedRouterError::RouterExcessiveInputAmount))
    // );
}

#[test]
#[should_panic] // TODO: Test the imported error
fn swap_tokens_for_exact_tokens_amount_in_max_not_enough_amount_in_should_minus_1() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .router_contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, // amount_out
        &(amount_in_should - 1), // amount_in_max
        &path,                // path
        &test.user,           // to
        &deadline,            // deadline
    );

    // assert_eq!(
    //     result,
    //     Err(Ok(CombinedRouterError::RouterExcessiveInputAmount))
    // );
}


#[test]
fn swap_tokens_for_exact_tokens_amount_in_should() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    let expected_amount_out = 5_000_000;
    let amount_in_should = test.router_contract.router_get_amounts_in(&expected_amount_out, &path).get(0).unwrap();

    let amounts = test.adapter_contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &(amount_in_should),  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(amounts.get(0).unwrap(), amount_in_should);
    assert_eq!(amounts.get(1).unwrap(), expected_amount_out);

    let initial_user_balance = 20_000_000_000_000_000_000;

    // pub fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> Result<i128, SoroswapLibraryError> {
    //     if amount_out <= 0 {
    //         return Err(SoroswapLibraryError::InsufficientOutputAmount);
    //     }
    //     if reserve_in <= 0 || reserve_out <= 0 {
    //         return Err(SoroswapLibraryError::InsufficientLiquidity);
    //     }
    //     let numerator = reserve_in.checked_mul(amount_out).unwrap().checked_mul(1000).unwrap();
    //     let denominator = reserve_out.checked_sub(amount_out).unwrap().checked_mul(997).unwrap();
    //     Ok(numerator.checked_ceiling_div(denominator).unwrap().checked_add(1).unwrap())
    // }

    // numerator = 1_000_000_000_000_000_000 * 5_000_000 * 1_000 = 5_000_000_000_000_000_000_000_000_000
    // denominator = (4000000000000000000 - 5000000) * 997 = 3999999999995000000 * 997 = 3987999999995015000000

    // num/den +1 = 5000000000000000000000000000 / 3987999999995015000000 +1 =  ceil(1253761.283853122) +1
    // = 1253762 + 1 = 1253763
    
    let expected_amount_0_in = 1253763;
    assert_eq!(amounts.get(0).unwrap(), expected_amount_0_in);


    assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - expected_amount_0_in);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1*2 + expected_amount_out);

    let pair_address = test.factory_contract.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(test.token_0.balance(&pair_address), amount_0 + expected_amount_0_in);
    assert_eq!(test.token_1.balance(&pair_address), amount_1 - expected_amount_out);

}


#[test]
fn swap_tokens_for_exact_tokens_2_hops() {
    let test = SoroswapAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    test.adapter_contract.initialize(
        &String::from_str(&test.env, "soroswap"),
        &test.router_contract.address);


    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let initial_user_balance = 20_000_000_000_000_000_000;

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;
    let amount_2: i128 = 8_000_000_000_000_000_000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());

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

    let amounts = test.adapter_contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &amount_in_should,  // amount_in_max
        &path, // path
        &test.user, // to
        &desired_deadline); // deadline


    assert_eq!(amounts.get(0).unwrap(), amount_in_should); 
    assert_eq!(amounts.get(1).unwrap(), middle_amount_in); 
    assert_eq!(amounts.get(2).unwrap(), expected_amount_out);

    assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - amount_in_should);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1*2);
    assert_eq!(test.token_2.balance(&test.user), initial_user_balance - amount_2 + expected_amount_out);

    let pair_address_0_1 = test.factory_contract.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(test.token_0.balance(&pair_address_0_1), amount_0 + amount_in_should);
    assert_eq!(test.token_1.balance(&pair_address_0_1), amount_1 - middle_amount_in);

    let pair_address_1_2 = test.factory_contract.get_pair(&test.token_1.address, &test.token_2.address);
    assert_eq!(test.token_1.balance(&pair_address_1_2), amount_1 + middle_amount_in);
    assert_eq!(test.token_2.balance(&pair_address_1_2), amount_2 - expected_amount_out);
}
