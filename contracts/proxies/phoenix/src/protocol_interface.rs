use soroban_sdk::{Env, Address, Vec, vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use crate::error::CombinedProxyError;

// pub struct Swap {
//   pub ask_asset: Address,
//   pub offer_asset: Address,
//   pub ask_asset_min_amount: Option<i128>,
// }

soroban_sdk::contractimport!(
    file = "../../../protocols/phoenix/target/wasm32-unknown-unknown/release/phoenix_multihop.wasm"
);
pub type PhoenixMultihopClient<'a> = Client<'a>;

fn convert_to_swaps(e: &Env, addresses: Vec<Address>) -> Vec<Swap> {
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


pub fn protocol_swap(
    e: &Env,
    amount_in: &i128,
    amount_out_min_or_max: &i128,
    path: Vec<Address>,
    to: Address,
    deadline: u64,
    isExactIn: bool,
) -> Result<Vec<i128>, CombinedProxyError> {
    if !has_protocol_address(e) {
        return Err(CombinedProxyError::ProxyProtocolAddressNotFound);
    }

    let phoenix_multihop_address = get_protocol_address(e);
    let phoenix_multihop_client = PhoenixMultihopClient::new(e, &phoenix_multihop_address);

    let operations = convert_to_swaps(e, path);

    phoenix_multihop_client.swap(&to, &operations, &None, &amount_in);

    // Returning empty array (should check phoenix response if it return amounts, apparently it doesnt)
    Ok(vec![&e])
}