extern crate std;

use soroban_sdk::{Address, vec, Vec};
use crate::test::CometAggregatorAdapterTest;
use adapter_interface::AdapterError;

#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.adapter_contract_not_initialized.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
        &None
    );

    assert_eq!(result,Err(Ok(AdapterError::NotInitialized)));

}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_tokens_for_exact_tokens_amount_out_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &1,        // deadline
        &None
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #37)")]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = CometAggregatorAdapterTest::setup();
    test.env.cost_estimate().budget().reset_unlimited();

    let path: Vec<Address> = vec![&test.env, test.token_0.address, test.token_1.address];

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &1,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &1,        // deadline
        &None
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #405)")]
fn swap_tokens_for_exact_tokens_expired() {
    let test = CometAggregatorAdapterTest::setup();

    let path: Vec<Address> = Vec::new(&test.env);

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
        &None
    );
}


#[test]
#[should_panic] // TODO: Test the imported error
fn try_swap_tokens_for_exact_tokens_invalid_path() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];

    test.adapter_contract.swap_tokens_for_exact_tokens( // add try_ to test the error
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
        &None
    );
   
    // assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInvalidPath)));
}


#[test]
#[should_panic(expected = "HostError: Error(Contract, #19)")]
fn try_swap_tokens_for_exact_tokens_insufficient_input_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.cost_estimate().budget().reset_unlimited();

    let amount_out = 1_000_000;
    let expected_amount_in = 1_003_015;
            
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

    test.adapter_contract.swap_tokens_for_exact_tokens(
        &amount_out,        // amount_out
        &(expected_amount_in - 1),        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
        &None
    );
}

#[test]
fn try_swap_tokens_for_exact_tokens_sufficient_input_amount() {
    let test = CometAggregatorAdapterTest::setup();

    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    test.env.cost_estimate().budget().reset_unlimited();

    let amount_out = 1_000_000;
    let expected_amount_in = 1_003_015;

        
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


    let executed_amounts = test.adapter_contract.swap_tokens_for_exact_tokens(
        &amount_out,        // amount_out
        &expected_amount_in,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
        &None
    );

    assert_eq!(executed_amounts.get(0).unwrap(), expected_amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), amount_out);
}