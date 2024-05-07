#![deny(warnings)]
#![no_std]

use soroban_sdk::{contractclient, contractspecfn, Address, Env, Vec, String};
pub struct Spec;

mod error;
pub use error::ProxyError;

/// Interface for SoroswapAggregatorProxy
#[contractspecfn(name = "Spec", export = false)]
#[contractclient(name = "SoroswapAggregatorProxyClient")]

pub trait SoroswapAggregatorProxyTrait {
    fn initialize(e: Env, protocol_id: String, protocol_address: Address) -> Result<(), ProxyError>;

    fn swap(
        env: Env,
        to: Address,
        path: Vec<Address>,
        amount_in: i128,
        amount_out_min_or_max: i128,
        deadline: u64,
        is_exact_in: bool,
    ) -> Result<Vec<i128>, ProxyError>;

    /*  *** Read only functions: *** */
    fn get_protocol_id(e: &Env) -> Result<Address, ProxyError>;
    fn get_protocol_address(e: &Env) -> Result<Address, ProxyError>;
}
