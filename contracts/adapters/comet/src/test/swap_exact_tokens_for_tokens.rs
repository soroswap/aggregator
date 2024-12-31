extern crate std;

use soroban_sdk::{Address, vec, Vec};
use crate::test::CometAggregatorAdapterTest;
use adapter_interface::AdapterError;

#[test]
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract_not_initialized.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_exact_tokens_for_tokens_amount_in_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &-1,           // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &1,            // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_exact_tokens_for_tokens_amount_out_min_negative() {
    let test: CometAggregatorAdapterTest<'_> = CometAggregatorAdapterTest::setup();
    test.env.budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &-1,           // amount_out_min
        &path,         // path
        &test.user,    // to
        &1,            // deadline
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #405)")]
fn swap_exact_tokens_for_tokens_expired() {
    let test = CometAggregatorAdapterTest::setup();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &10000,            // amount_in
        &1,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );
}



#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_exact_tokens_for_tokens_invalid_path() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    test.adapter_contract.swap_exact_tokens_for_tokens(
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
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.env.budget().reset_unlimited();


    let amount = 100_000;

    test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount,        // amount_in
        &amount,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
}



#[test]
#[should_panic(expected = "HostError: Error(Contract, #20)")]
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_in = 1_000_000;

    let expected_amount_out = 996996;

    
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

    test.env.budget().reset_unlimited();
    test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out + 1),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
    );
}



#[test]
fn swap_exact_tokens_for_tokens_enough_output_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_in = 1_000_000;

    let expected_amount_out = 996996;

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

    test.env.budget().reset_unlimited();
    let executed_amounts = test.adapter_contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &(expected_amount_out),  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
    
}