// based on https://github.com/Phoenix-Protocol-Group/phoenix_contracts/tree/v1.0.0

use soroban_sdk::{Env, Address, Vec, vec};
use crate::storage::{get_protocol_address};
use soroswap_aggregator_adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./phoenix_contracts/phoenix_multihop.wasm"
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
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>, 
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AdapterError> {

    let phoenix_multihop_address = get_protocol_address(&e)?;
    let phoenix_multihop_client = PhoenixMultihopClient::new(&e, &phoenix_multihop_address);
    let operations = convert_to_swaps(e, path);

    // By using max_spread_bps = None, the Phoenix LP will use the maximum allowed slippage
    // amount_in is the amount being sold of the first token in the operations.
    phoenix_multihop_client.swap(
        &to, // recipient: Address, 
        &operations, // operations: Vec<Swap>,
        &None, // max_spread_bps: Option<i64>.
        &amount_in); //amout: i128. Amount being sold. Input from the user,

    // Returning empty array (should check phoenix response if it return amounts, apparently it doesnt)
    // We dont know the amount of the output token, unless we do an extra cross contract call to the token contract
    // In order to avoid extra calls, we are returning an empty array

    //To be more exact, this adapter should do the cross contract call and check for amount_out_min...
    // But in order to calculate the exact amount_out we should do 2 cross_contract calls to the token contract, 
    // one before and one after....

    // Because our Aggregator contract checks for everything, we wont add this here.
    // Can we do Benchmark studies to see the amount of extra operations/fees that this incurrs?
    Ok(vec![&e])
}

pub fn protocol_swap_tokens_for_exact_tokens(
    e: &Env,
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
    deadline: &u64,
) -> Result<Vec<i128>, AdapterError> {

    let phoenix_multihop_address = get_protocol_address(&e)?;
    let phoenix_multihop_client = PhoenixMultihopClient::new(&e, &phoenix_multihop_address);
    let operations = convert_to_swaps(e, path);

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

    // This is the same of operations.rev()

    let reverse_simulated_swap = phoenix_multihop_client.simulate_reverse_swap(
        operations.rev(), //operations: Vec<Swap>,
        amount_out); //amount: i128,
    
    if reverse_simulated_swap.offer_amount > amount_in_max {
        // TODO: Here we should have a new Error object
    }

    phoenix_multihop_client.swap(
        &to, // recipient: Address, 
        &operations, // operations: Vec<Swap>,
        &None, // max_spread_bps: Option<i64>.
        &reverse_simulated_swap.offer_amount); //amout: i128. Amount being sold. Input from the user,

    // Here we are not 100% sure if the  amount_in will be exactely reverse_simulated_swap.offer_amount
    // and if the amount_out will be indeed amount_out
    Ok(vec![&e])
}