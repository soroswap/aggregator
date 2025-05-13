use soroban_sdk::{Env, Address, Vec, token::Client as TokenClient, BytesN, vec};
use adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./aqua_contracts/soroban_liquidity_pool_router_contract.wasm"
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

        The interface is based on https://github.com/AquaToken/soroban-amm/
*/


fn convert_to_swaps_chain(
    e: &Env, 
    path: &Vec<Address>,
    bytes: &Option<Vec<BytesN<32>>>,
) -> Result<
    Vec<(Vec<Address>, BytesN<32>, Address)>, // (path, pool_hash, token_out)
    AggregatorError
> {
    
    // We check that bytes is not None
    let pool_hashes_vec = bytes.as_ref().ok_or(AggregatorError::MissingPoolHashes)?;
    
    // path should have at least 2 elements. ifnot error WrongMinimumPathLength
    if path.len() < 2 {
        return Err(AggregatorError::WrongMinimumPathLength);
    }
    // We check that the length of bytes is equal to the length of path - 1
    if pool_hashes_vec.len() != path.len().checked_sub(1).unwrap() { // unwrap safe as we checked the length of path
        return Err(AggregatorError::WrongPoolHashesLength);
    }

    let mut swaps_chain = Vec::new(e);
    for i in 0..(path.len() - 1) {
        let token_in = path.get(i).unwrap(); // This should be safe as we checked the length of path
        let token_out = path.get(i + 1).unwrap(); // This should be safe as we checked the length of path
        let pool_hash = pool_hashes_vec.get(i).unwrap(); // This should be safe as we checked the length of pool_hashes_vec

        let swap_chain_path = if token_in < token_out {
            vec![&e, token_in.clone(), token_out.clone()]
        } else {
            vec![&e, token_out.clone(), token_in.clone()]
        };

        swaps_chain.push_back((swap_chain_path, pool_hash.clone(), token_out.clone()));
    }

    Ok(swaps_chain)
}

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    aqua_router_address: &Address,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>, // (TokenA, TokenB, TokenC, TokenD), being TokenC the token to get
    to: &Address,
    bytes: &Option<Vec<BytesN<32>>>, // (pool_hash_0, pool_hash_1, pool_hash_2)
) -> Result<Vec<i128>, AggregatorError> {

    let aqua_router_client = AquaRouterClient::new(&e, &aqua_router_address);  
    let swaps_chain = convert_to_swaps_chain(e, path, bytes)?;

    let token_in = path.get(0).expect("Failed to get token in address"); // should be safe as we checked the length of path
    let token_out_address = path.get(path.len().checked_sub(1).unwrap()).expect("Failed to get token out address"); // should be safe as we checked the length of path

    // TODO Remove this if we remove the check
    let initial_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
    
    // let final_amount_out = aqua_router_client.swap_chained(
    aqua_router_client.swap_chained(
        &to, // user: Address 
        &swaps_chain, // swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        &token_in, // token_in: Address,
        &(*amount_in as u128), // in_amount: i128,
        &(*amount_out_min as u128), // out_min: i128
    );

    
    // Check if the amount of token_out received is greater than the minimum amount expected
    // TODO: Remove this checks if we want to reduce the number of total instructions
    // TODO: Do benchmarking
    // We could get the final_amount_out from the aqua_router_client.swap_chained function
    let final_token_out_balance = TokenClient::new(&e, &token_out_address).balance(&to);
    let final_amount_out = final_token_out_balance.checked_sub(initial_token_out_balance).unwrap();
    if  final_amount_out < *amount_out_min {
        // panic
        panic!("Amount of token out received is less than the minimum amount expected");
    }
 
    let mut swap_amounts: Vec<i128> = Vec::new(e);
    swap_amounts.push_back(*amount_in);
    swap_amounts.push_back(final_amount_out as i128);

    Ok(swap_amounts)
}

pub fn protocol_swap_tokens_for_exact_tokens(
    e: &Env,
    aqua_router_address: &Address,
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    bytes: &Option<Vec<BytesN<32>>>, // (pool_hash_0, pool_hash_1, pool_hash_2)
) -> Result<Vec<i128>, AggregatorError> {

    let aqua_router_client = AquaRouterClient::new(&e, &aqua_router_address);
    let swaps_chain = convert_to_swaps_chain(e, path, bytes)?;
    /*
        fn swap_chained_strict_receive(
            e: Env,
            user: Address,
            swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
            token_in: Address,
            out_amount: u128, // fixed amount of output token to receive
            max_in: u128,     // maximum input token amount allowed
        ) -> u128 // final_amount_in
    */


    let token_in = path.get(0).expect("Failed to get token in address");
    
    let final_amount_in = aqua_router_client.swap_chained_strict_receive(
        &to, // user: Address 
        &swaps_chain, // swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        &token_in, // token_in: Address,
        &(*amount_out as u128), // out_amount: u128,
        &(*amount_in_max as u128), // max_in: u128,
    );

    let mut swap_amounts: Vec<i128> = Vec::new(e);
    swap_amounts.push_back(final_amount_in as i128);
    swap_amounts.push_back(*amount_out);

    Ok(swap_amounts)
}