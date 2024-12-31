use soroban_sdk::{vec, Address, Env, Vec};
use crate::storage::get_protocol_address;
use adapter_interface::AdapterError;

soroban_sdk::contractimport!(
    file = "./comet_contracts/comet_pool.wasm"
);
pub type CometPoolClient<'a> = Client<'a>;

pub fn protocol_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: &Vec<Address>,
    to: &Address,
) -> Result<Vec<i128>, AdapterError> {

    let comet_pool_address = get_protocol_address(&e)?;
    let comet_client = CometPoolClient::new(&e, &comet_pool_address);

    let (amount_out, _) = comet_client.swap_exact_amount_in(
        &path.get(0).unwrap(),
        amount_in,
        &path.get(1).unwrap(),
        amount_out_min,
        &i128::MAX,
        to
    );

    Ok(vec![e, *amount_in, amount_out])
}

pub fn protocol_swap_tokens_for_exact_tokens(
    e: &Env,
    amount_out: &i128,
    amount_in_max: &i128,
    path: &Vec<Address>,
    to: &Address,
) -> Result<Vec<i128>, AdapterError> {

    let comet_pool_address = get_protocol_address(&e)?;
    let comet_client = CometPoolClient::new(&e, &comet_pool_address);

    let (amount_in, _) = comet_client.swap_exact_amount_out(
        &path.get(0).unwrap(),
        amount_in_max,
        &path.get(1).unwrap(),
        amount_out,
        &i128::MAX,
        to
    );

    Ok(vec![e, amount_in, *amount_out])
}
