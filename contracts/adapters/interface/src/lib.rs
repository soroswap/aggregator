#![deny(warnings)]
#![no_std]

use soroban_sdk::{contractclient, contractspecfn, Address, Env, Vec, String};
pub struct Spec;

mod error;
pub use error::AdapterError;

/// Interface for SoroswapAggregatorAdapter
#[contractspecfn(name = "Spec", export = false)]
#[contractclient(name = "SoroswapAggregatorAdapterClient")]

pub trait SoroswapAggregatorAdapterTrait {
    fn initialize(e: Env, protocol_id: String, protocol_address: Address) -> Result<(), AdapterError>;

    fn swap(
        env: Env,
        to: Address,
        path: Vec<Address>,
        amount_in: i128,
        amount_out_min_or_max: i128,
        deadline: u64,
        is_exact_in: bool,
    ) -> Result<Vec<i128>, AdapterError>;

    /*  *** Read only functions: *** */
    fn get_protocol_id(e: &Env) -> Result<Address, AdapterError>;
    fn get_protocol_address(e: &Env) -> Result<Address, AdapterError>;
}
