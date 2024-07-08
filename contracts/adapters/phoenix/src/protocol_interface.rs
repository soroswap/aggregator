use soroban_sdk::{Env, Address, Vec, vec};
use crate::storage::{get_protocol_address};
use soroswap_aggregator_adapter_interface::{AdapterError};

soroban_sdk::contractimport!(
    file = "./contracts/phoenix_multihop.wasm"
);
pub type PhoenixMultihopClient<'a> = Client<'a>;

fn convert_to_swaps(e: &Env, addresses: &Vec<Address>) -> Vec<Swap> {
    let mut swaps = Vec::new(e);

    // Iterate through the addresses, creating a Swap for each pair
    // Skip the last address since it cannot be an offer_asset without a corresponding ask_asset
    for i in 0..(addresses.len() - 1) {
        let offer_asset = addresses.get(i).expect("Failed to get offer asset");
        let ask_asset = addresses.get(i + 1).expect("Failed to get ask asset");

        swaps.push_back(Swap {
          ask_asset: ask_asset.clone(),
          offer_asset: offer_asset.clone(),
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

    // TODO: CHECK AND TEST
    phoenix_multihop_client.swap(&to, &operations, &None, &amount_in);

    // Returning empty array (should check phoenix response if it return amounts, apparently it doesnt)
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

    // TODO: CHECK AND TEST
    phoenix_multihop_client.swap(&to, &operations, &None, &amount_in_max);

    // Returning empty array (should check phoenix response if it return amounts, apparently it doesnt)
    Ok(vec![&e])
}