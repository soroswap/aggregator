// based on https://github.com/Aqua-Protocol-Group/aqua_contracts/tree/v1.0.0

use soroban_sdk::{Env, Address, Vec, token::Client as TokenClient};
use crate::storage::{get_protocol_address};
use adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./aqua_contracts/soroban_liquidity_pool_swap_router_contract.wasm"
);
pub type AquaRouterClient<'a> = Client<'a>;

/*
        This first version of the AquaAdapter, is written just for pools with 2 tokens, so we will build from 
        path = (TokenA, TokenB, TokenC, TokenD)
        bytes = (pool_hash_0, pool_hash_1, pool_hash_2)
        where pool_hash_0 = hash of the pool with tokenA and tokenB
        where pool_hash_1 = hash of the pool with tokenB and tokenC
        where pool_hash_2 = hash of the pool with tokenC and tokenD
        where token_out = tokenD
        where token_in = tokenA
        where in_amount = amount_in
        where out_min = amount_out_min
    */


fn convert_to_swaps_chain(
    e: &Env, 
    path: &Vec<Address>,
    bytes: &Option<Vec<BytesN<32>>>,
) -> Vec<(Vec<Address>, BytesN<32>, Address)> {

    // We check that bytes is not None
    let pool_hashes_vec = match bytes {
        Some(v) => v,
        None => {
            panic!("Bytes is None. Aqua needs the pool hashes to swap");
        }
    };

    // We check that the length of bytes is equal to the length of path - 1
    if pool_hashes_vec.len() != path.len() - 1 {
        panic!("Bytes length is not equal to path length - 1");
    }

    let mut swaps_chain = Vec::new(e);
    for i in 0..(path.len() - 1) {
        let token_in = path.get(i).expect("Failed to get offer asset");
        let token_out = path.get(i + 1).expect("Failed to get ask asset");
        let pool_hash = pool_hashes_vec.get(i).expect("Failed to get pool hash");

        swaps_chain.push_back((vec![token_in.clone(), token_out.clone()], pool_hash.clone(), token_out.clone()));
    }

    swaps_chain
}

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>, // (TokenA, TokenB, TokenC, TokenD), being TokenC the token to get
    to: &Address,
    _deadline: &u64,
    bytes: &Option<Vec<BytesN<32>>>, // (pool_hash_0, pool_hash_1, pool_hash_2)
) -> Result<Vec<i128>, AdapterError> {

    let aqua_router_address = get_protocol_address(&e)?;
    let aqua_router_client = AquaRouterClient::new(&e, &aqua_router_address);
    
    let swaps_chain = convert_to_swaps_chain(e, path, bytes);


    // fn swap_chained(
    //     e: Env,
    //     user: Address, // to
    //     swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
    //     token_in: Address,
    //     in_amount: u128,
    //     out_min: u128,
    // ) -> u128 {
   
    let token_in = path.get(0).expect("Failed to get token in address");
    let token_out_address = path.get(path.len() - 1).expect("Failed to get token out address");
    let initial_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
    
    let final_amount_out = aqua_router_client.swap_chained(
        &to, // recipient: Address, 
        &swaps_chain, // swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        &token_in, // token_in: Address,
        &amount_in, // in_amount: i128,
        &amount_out_min, // out_min: i128
    );

    
    // check if the amount of token_out received is greater than the minimum amount expected
    // TODO: Remove this checks if we want to reduce the number of total instructions
    // TODO: Do benchmarking
    let final_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
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
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    _deadline: &u64,
) -> Result<Vec<i128>, AdapterError> {

    let aqua_router_address = get_protocol_address(&e)?;
    let aqua_router_client = AquaRouterClient::new(&e, &aqua_router_address);
    let operations = convert_to_swaps(e, path);

    // We first need to get the "reverse_amount from aqua.simulate_reverse_swap"
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
    let reverse_simulated_swap = aqua_router_client.simulate_reverse_swap(
        &operations_reversed, //operations: Vec<Swap>,
        amount_out); //amount: i128,
    
    // TODO: Eliminate this check. The overall in max is checked by the Aggregator
    // Removing this check will reduce the amount of instructions/
    // TODO: Do Benchmarking
    if reverse_simulated_swap.offer_amount > *amount_in_max {
        panic!("Amount of token in required is greater than the maximum amount expected");
    }

    aqua_router_client.swap(
        &to, // recipient: Address, 
        &operations, // operations: Vec<Swap>,
        &None, // max_spread_bps: Option<i64>.
        &reverse_simulated_swap.offer_amount); //amout: i128. Amount being sold. Input from the user,

    // Here we trust in the amounts returned by Aqua contracts
    let mut swap_amounts: Vec<i128> = Vec::new(e);
    swap_amounts.push_back(reverse_simulated_swap.offer_amount);
    swap_amounts.push_back(*amount_out);

    Ok(swap_amounts)
}