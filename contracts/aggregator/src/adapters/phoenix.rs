// based on https://github.com/Phoenix-Protocol-Group/phoenix_contracts/tree/v1.0.0

use soroban_sdk::{Env, Address, Vec, token::Client as TokenClient};

use crate::error::AggregatorError;
// use phoenix_contracts::PoolType;

soroban_sdk::contractimport!(
    file = "../aggregator/phoenix_contracts//phoenix_multihop.wasm"
);
pub type PhoenixMultihopClient<'a> = Client<'a>;

fn convert_to_swaps(e: &Env, path: &Vec<Address>) -> Vec<Swap> {
    let mut swaps = Vec::new(e);

    // Iterate through the addresses in the path, creating a Swap object for each pair
    // If path is [token0, token1, token2, token3], swaps should be
    // swap_0 = Swap{
    //     offer_asset: token0,
    //     ask_asset: token1,
    //     ask_asset_min_amount: None,
    // },
    // swap_1 = Swap{
    //     offer_asset: token1,
    //     ask_asset: token2,
    //     ask_asset_min_amount: None,
    // },
    // swap_2 = Swap{
    //     offer_asset: token2,
    //     ask_asset: token3,
    //     ask_asset_min_amount: None,
    // }

    for i in 0..(path.len() - 1) {
        let offer_asset = path.get(i).expect("Failed to get offer asset");
        let ask_asset = path.get(i + 1).expect("Failed to get ask a    sset");

        swaps.push_back(Swap {
          offer_asset: offer_asset.clone(), // asset being sold (token_in)
          ask_asset: ask_asset.clone(), // asset buying (token_out)
          ask_asset_min_amount: None, 
        });
    }

    swaps
}

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    phoenix_multihop_address: &Address,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>, 
    to: &Address,
    _deadline: &u64,
) -> Result<Vec<i128>, AggregatorError> {

    let phoenix_multihop_client = PhoenixMultihopClient::new(&e, &phoenix_multihop_address);
    let operations = convert_to_swaps(e, path);
    
    // TODO: Remove this checks if we want to reduce the number of total instructions
    // TODO: Do benchmarking
    let token_out_address = path.get(path.len() - 1).expect("Failed to get token out address");
    let initial_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
    
    let pool_type = PoolType::Xyk; // currently we only support XYK pools
    
    // By using max_spread_bps = None, the Phoenix LP will use the maximum allowed slippage
    // amount_in is the amount being sold of the first token in the operations.

    // fn swap(
    //     env: Env,
    //     recipient: Address,
    //     // FIXM: Disable Referral struct
    //     // referral: Option<Referral>,
    //     operations: Vec<Swap>,
    //     max_spread_bps: Option<i64>,
    //     amount: i128,
    //     pool_type: PoolType,
    //     deadline: Option<u64>,
    //     max_allowed_fee_bps: Option<i64>,
    // );
    phoenix_multihop_client.swap(
        &to, // recipient: Address, 
        &operations, // operations: Vec<Swap>,
        &None, // max_spread_bps: Option<i64>.
        &amount_in, //amout: i128. Amount being sold. Input from the user,
        &pool_type, // pool_type: PoolType,
        &None, // deadline: Option<u64>,
        &None, // max_allowed_fee_bps: Option<i64>,
    ); 
        
    let final_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
    
    // check if the amount of token_out received is greater than the minimum amount expected
    // TODO: Remove this checks if we want to reduce the number of total instructions
    // TODO: Do benchmarking
    let final_amount_out = final_token_out_balance.checked_sub(initial_token_out_balance).unwrap();
    if  final_amount_out < *amount_out_min {
        // panic
        panic!("Amount of token out received is less than the minimum amount expected");
    }

    let mut swap_amounts: Vec<i128> = Vec::new(e);
    swap_amounts.push_back(amount_in.clone());
    swap_amounts.push_back(final_amount_out);

    Ok(swap_amounts)
}

pub fn protocol_swap_tokens_for_exact_tokens(
    e: &Env,
    phoenix_multihop_address: &Address,
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    _deadline: &u64,
) -> Result<Vec<i128>, AggregatorError> {

    let phoenix_multihop_client = PhoenixMultihopClient::new(&e, &phoenix_multihop_address);
    let operations = convert_to_swaps(e, path);

    // let pool_type = PoolType::Xyk; // currently we only support XYK pools


    // We first need to get the "reverse_amount from phoenix.simulate_reverse_swap"
    // however here, if the path is [t0, t1, t2, t3, t4], the  operations should be
    // swap_0 = Swap{
    //     offer_asset: t3,
    //     ask_asset: t4,
    //     ask_asset_min_amount: None,
    // },
    // swap_1 = Swap{
    //     offer_asset: t2,
    //     ask_asset: t3,
    //     ask_asset_min_amount: None,
    // },
    // swap_2 = Swap{
    //     offer_asset: t1,
    //     ask_asset: t2,
    //     ask_asset_min_amount: None,
    // },
    // swap_3 = Swap{
    //     offer_asset: t0,
    //     ask_asset: t1,
    //     ask_asset_min_amount: None,
    // }

    let mut operations_reversed = soroban_sdk::Vec::new(&e);
    for op in operations.iter().rev() {
        operations_reversed.push_back(op.clone());
    }

    // fn simulate_reverse_swap(
    //     env: Env,
    //     operations: Vec<Swap>,
    //     amount: i128,
    //     pool_type: PoolType,
    // ) -> SimulateReverseSwapResponse;

    let reverse_simulated_swap = phoenix_multihop_client.simulate_reverse_swap(
        &operations_reversed, //operations: Vec<Swap>,
        amount_out, //amount: i128,
        &PoolType::Xyk //PoolType,
    ); 
    
    // TODO: Eliminate this check. The overall in max is checked by the Aggregator
    // Removing this check will reduce the amount of instructions/
    // TODO: Do Benchmarking
    if reverse_simulated_swap.offer_amount > *amount_in_max {
        panic!("Amount of token in required is greater than the maximum amount expected");
    }

    // fn swap(
    //     env: Env,
    //     recipient: Address,
    //     // FIXM: Disable Referral struct
    //     // referral: Option<Referral>,
    //     operations: Vec<Swap>,
    //     max_spread_bps: Option<i64>,
    //     amount: i128,
    //     pool_type: PoolType,
    //     deadline: Option<u64>,
    //     max_allowed_fee_bps: Option<i64>,
    // );
    phoenix_multihop_client.swap(
        &to, // recipient: Address, 
        &operations, // operations: Vec<Swap>,
        &None, // max_spread_bps: Option<i64>.
        &reverse_simulated_swap.offer_amount, //amout: i128. Amount being sold. Input from the user,
        &PoolType::Xyk, // pool_type: PoolType,
        &None, // deadline: Option<u64>,
        &None, // max_allowed_fee_bps: Option<i64>,
    );

    // Here we trust in the amounts returned by Phoenix contracts
    let mut swap_amounts: Vec<i128> = Vec::new(e);
    swap_amounts.push_back(reverse_simulated_swap.offer_amount);
    swap_amounts.push_back(*amount_out);

    Ok(swap_amounts)
}