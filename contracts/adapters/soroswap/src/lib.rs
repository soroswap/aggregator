#![no_std]

use soroban_sdk::{Env, Address, Vec, BytesN};
use crate::adapter::{Adapter, Asset, Quote};

pub struct SoroswapAdapter;

impl Adapter for SoroswapAdapter {
    pub fn get_quote(
        env: Env,
        asset_in: Asset,
        asset_out: Asset,
        amount_in: i128,
        _bytes: Option<Vec<BytesN<32>>>,
    ) -> Quote {
        // Placeholder implementation - real logic queries Soroswap pools
        Quote {
            amount_out: amount_in,
            path: Vec::new(&env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_get_quote() {
        let env = Env::default();
        let asset_in = Asset::Native;
        let asset_out = Asset::Contract(Address::from_string(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
        let amount_in = 1_000;
        let quote = SoroswapAdapter::get_quote(env, asset_in, asset_out, amount_in, None);
        assert_eq!(quote.amount_out, amount_in);
    }
}
