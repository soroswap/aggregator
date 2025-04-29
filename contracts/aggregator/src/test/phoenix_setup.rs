use super::{generate_salt, DeployerClient};

// For Phoenix
mod phoenix_adapter {
    soroban_sdk::contractimport!(
        file =
            "../target/wasm32-unknown-unknown/release/phoenix_adapter.optimized.wasm"
    );
    pub type SoroswapAggregatorAdapterForPhoenixClient<'a> = Client<'a>;
}
pub use phoenix_adapter::SoroswapAggregatorAdapterForPhoenixClient;
use crate::test::install_token_wasm;
// Adapter for phoenix
// pub fn create_phoenix_adapter<'a>(e: &Env) -> SoroswapAggregatorAdapterForPhoenixClient<'a> {
//     let adapter_address = &e.register_contract_wasm(None, phoenix_adapter::WASM);
//     let adapter = SoroswapAggregatorAdapterForPhoenixClient::new(e, adapter_address);
//     adapter
// }

pub fn create_phoenix_adapter<'a>(e: &Env, deployer_client: &DeployerClient<'a>, multihop_contract: Address, admin: Address) -> SoroswapAggregatorAdapterForPhoenixClient<'a> {
    let wasm_hash = e.deployer().upload_contract_wasm(phoenix_adapter::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&e, &generate_salt(1));
    let init_fn = Symbol::new(&e, &("initialize"));

    let protocol_id = String::from_str(&e, "phoenix");
    let protocol_address = multihop_contract.clone();

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(e);

    let (contract_id, _init_result) = deployer_client.deploy(
        &admin,
        &wasm_hash,
        &salt,
        &init_fn,
        &init_fn_args,
    );

    let adapter_contract = SoroswapAggregatorAdapterForPhoenixClient::new(e, &contract_id);
    adapter_contract
}



// #![cfg(test)]
// extern crate std;
use soroban_sdk::{
    testutils::{
        arbitrary::std,
        Address as _,
    }, vec, Address, Bytes, BytesN, Env, String, Symbol, Vec, Val, IntoVal
};

/* *************  PHOENIX FACTORY  *************  */

#[allow(clippy::too_many_arguments)]
pub mod factory {
    soroban_sdk::contractimport!(
        file = "../adapters/phoenix/phoenix_contracts/phoenix_factory.wasm"
    );
}
use factory::{LiquidityPoolInitInfo, StakeInitInfo, TokenInitInfo};
pub use factory::PoolType;

pub fn deploy_factory_contract(e: &Env, admin: & Address) -> Address {
    let factory_wasm = e.deployer().upload_contract_wasm(factory::WASM);
    let salt = Bytes::new(&e.clone());
    let salt = e.crypto().sha256(&salt);

    e.deployer().with_address(admin.clone(), salt).deploy_v2(factory_wasm, ())
}

pub use factory::Client as PhoenixFactory;

/* *************  MULTIHOP  *************  */
#[allow(clippy::too_many_arguments)]
pub mod multihop {
    soroban_sdk::contractimport!(file = "../adapters/phoenix/phoenix_contracts/phoenix_multihop.wasm");
    pub type MultihopClient<'a> = Client<'a>;
}
pub use multihop::MultihopClient; 

pub fn install_multihop_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../adapters/phoenix/phoenix_contracts/phoenix_multihop.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}
pub fn deploy_multihop_contract<'a>(
    env: &Env,
    admin: impl Into<Option<Address>>,
    factory: &Address,
) -> MultihopClient<'a> {
    let admin = admin.into().unwrap_or(Address::generate(env));

    let multihop_address = &env.register(multihop::WASM, ());
    let multihop = MultihopClient::new(env, multihop_address); 

    multihop.initialize(&admin, factory);
    multihop
}


/* *************  STABLE CONTRACT  *************  */

#[allow(clippy::too_many_arguments)]
pub mod stable_contract {
    soroban_sdk::contractimport!(
        file = "../adapters/phoenix/phoenix_contracts/phoenix_pool_stable.wasm"
    );
}

pub fn install_stable_contract(env: &Env) -> BytesN<32> {
    env.deployer().upload_contract_wasm(stable_contract::WASM)
}


/* *************  LP CONTRACT  *************  */

#[allow(clippy::too_many_arguments)]
pub mod lp_contract {
    soroban_sdk::contractimport!(
        file = "../adapters/phoenix/phoenix_contracts/phoenix_pool.wasm"
    );
}

pub fn install_lp_contract(env: &Env) -> BytesN<32> {
    env.deployer().upload_contract_wasm(lp_contract::WASM)
}


/* *************  STAKE  *************  */

#[allow(clippy::too_many_arguments)]
pub fn install_stake_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../adapters/phoenix/phoenix_contracts/phoenix_stake.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}



pub fn deploy_and_initialize_factory<'a>(env: &Env, admin: Address) -> PhoenixFactory<'a> {
    let factory_addr = deploy_factory_contract(&env, &admin.clone());
    let factory_client = PhoenixFactory::new(env, &factory_addr);
    let multihop_wasm_hash = install_multihop_wasm(env);
    let whitelisted_accounts = vec![env, admin.clone()];

    let lp_wasm_hash = install_lp_contract(env);
    let stable_wasm_hash = install_stable_contract(env);
    let stake_wasm_hash = install_stake_wasm(env);
    let token_wasm_hash = install_token_wasm(env);

    factory_client.initialize( 
        &admin.clone(),
        &multihop_wasm_hash,
        &lp_wasm_hash,
        &stable_wasm_hash,
        &stake_wasm_hash,
        &token_wasm_hash,
        &whitelisted_accounts,
        &10u32,
    );

    factory_client
}

#[allow(clippy::too_many_arguments)]
pub fn deploy_and_initialize_lp(
    env: &Env,
    factory: &PhoenixFactory,
    admin: Address,
    mut token_a: Address,
    mut token_a_amount: i128,
    mut token_b: Address,
    mut token_b_amount: i128,
    fees: Option<i64>,
) {
    // 2. create liquidity pool from factory

    if token_b < token_a {
        std::mem::swap(&mut token_a, &mut token_b);
        std::mem::swap(&mut token_a_amount, &mut token_b_amount);
    }

    let token_init_info = TokenInitInfo {
        token_a: token_a.clone(),
        token_b: token_b.clone(),
    };
    let stake_init_info = StakeInitInfo {
        min_bond: 10i128,
        min_reward: 5i128,
        manager: Address::generate(env),
        max_complexity: 10u32,
    };

    let lp_init_info = LiquidityPoolInitInfo {
        admin: admin.clone(),
        swap_fee_bps: fees.unwrap_or(0i64),
        fee_recipient: admin.clone(),
        max_allowed_slippage_bps: 5000,
        default_slippage_bps: 2_500,
        max_allowed_spread_bps: 500,
        max_referral_bps: 5_000,
        stake_init_info,
        token_init_info,
    };

    let lp = factory.create_liquidity_pool(
        &admin.clone(), //     sender: Address,
        &lp_init_info, //     lp_init_info: LiquidityPoolInitInfo,
        &String::from_str(env, "Pool"),  //     share_token_name: String,
        &String::from_str(env, "PHO/XLM"),//     share_token_symbol: String,
        &PoolType::Xyk, //     pool_type: PoolType,
        &None::<u64>,//     amp: Option<u64>,
        &100i64, //     default_slippage_bps: i64,
        &2_000,//     max_allowed_fee_bps: i64,
    );

    let lp_client = lp_contract::Client::new(env, &lp);
    lp_client.provide_liquidity(
        &admin.clone(), //     depositor: Address,
        &Some(token_a_amount), //  desired_a: Option<i128>,
        &None, //     min_a: Option<i128>,
        &Some(token_b_amount), //     desired_b: Option<i128>,
        &None, //     min_b: Option<i128>,
        &None::<i64>, //     custom_slippage_bps: Option<i64>,
        &None::<u64>, //     deadline: Option<u64>,
    );
}
