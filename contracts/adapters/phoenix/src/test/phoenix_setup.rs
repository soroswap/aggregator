#![cfg(test)]
// extern crate std;
use soroban_sdk::{
    vec,
    IntoVal,
    String,
    Env, 
    Bytes,
    BytesN, 
    Address, 
    testutils::{
        arbitrary::std,
        Address as _,
    },
};

// use crate::contract::{Multihop, MultihopClient};


// //Token Contract
// pub mod token {
//     soroban_sdk::contractimport!(file = "../../../protocols/soroswap/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
//     pub type TokenClient<'a> = Client<'a>;
// }
// use token::TokenClient;

// pub fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
//     TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
// }


#[allow(clippy::too_many_arguments)]
pub mod factory {
    soroban_sdk::contractimport!(
        file = "./contracts/phoenix_factory.wasm"
    );
}
use crate::test::phoenix_setup::factory::{LiquidityPoolInitInfo, StakeInitInfo, TokenInitInfo};

/* *************  MULTIHOP  *************  */
#[allow(clippy::too_many_arguments)]
pub mod multihop {
    soroban_sdk::contractimport!(file = "./contracts/phoenix_multihop.wasm");
    pub type MultihopClient<'a> = Client<'a>;
}
use multihop::MultihopClient; 

pub fn deploy_multihop_contract<'a>(
    env: &Env,
    admin: impl Into<Option<Address>>,
    factory: &Address,
) -> MultihopClient<'a> {
    let admin = admin.into().unwrap_or(Address::generate(env));

    let multihop_address = &env.register_contract_wasm(None, multihop::WASM);
    let multihop = MultihopClient::new(env, multihop_address); 

    multihop.initialize(&admin, factory);
    multihop
}

/* *************  TOKEN  *************  */

pub mod token_contract {
    soroban_sdk::contractimport!(
        file = "./contracts/soroban_token_contract.wasm"
    );
}

use token_contract::Client as TokenClient;

pub fn create_token_contract_with_metadata<'a>(
    env: &Env,
    admin: &Address,
    decimals: u32,
    name: String,
    symbol: String,
    amount: i128,
) -> TokenClient<'a> {
    let token =
        TokenClient::new(env, &env.register_contract_wasm(None, token_contract::WASM));
    token.initialize(admin, &decimals, &name.into_val(env), &symbol.into_val(env));
    token.mint(admin, &amount);
    token
}



#[allow(clippy::too_many_arguments)]
pub mod lp_contract {
    soroban_sdk::contractimport!(
        file = "./contracts/phoenix_pool.wasm"
    );
}

pub fn install_lp_contract(env: &Env) -> BytesN<32> {
    env.deployer().upload_contract_wasm(lp_contract::WASM)
}

pub fn install_token_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./contracts/soroban_token_contract.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}

pub fn deploy_token_contract<'a>(env: &Env, admin: &Address) -> token_contract::Client<'a> {
    token_contract::Client::new(env, &env.register_stellar_asset_contract(admin.clone()))
}

#[allow(clippy::too_many_arguments)]
pub fn install_stake_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./contracts/phoenix_stake.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}

pub fn install_multihop_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./contracts/phoenix_multihop.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}

pub fn deploy_factory_contract(e: &Env, admin: Address) -> Address {
    let factory_wasm = e.deployer().upload_contract_wasm(factory::WASM);
    let salt = Bytes::new(e);
    let salt = e.crypto().sha256(&salt);

    e.deployer().with_address(admin, salt).deploy(factory_wasm)
}


pub fn deploy_and_mint_tokens<'a>(
    env: &'a Env,
    admin: &'a Address,
    amount: i128,
) -> token_contract::Client<'a> {
    let token = deploy_token_contract(env, admin);
    token.mint(admin, &amount);
    token
}

pub fn deploy_and_initialize_factory(env: &Env, admin: Address) -> factory::Client {
    let factory_addr = deploy_factory_contract(env, admin.clone());
    let factory_client = factory::Client::new(env, &factory_addr);
    let multihop_wasm_hash = install_multihop_wasm(env);
    let whitelisted_accounts = vec![env, admin.clone()];

    let lp_wasm_hash = install_lp_contract(env);
    let stake_wasm_hash = install_stake_wasm(env);
    let token_wasm_hash = install_token_wasm(env);

    factory_client.initialize(
        &admin.clone(),
        &multihop_wasm_hash,
        &lp_wasm_hash,
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
    factory: &factory::Client,
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
        fee_recipient: admin.clone(),
        max_allowed_slippage_bps: 5000,
        max_allowed_spread_bps: 500,
        swap_fee_bps: fees.unwrap_or(0i64),
        max_referral_bps: 5_000,
        token_init_info,
        stake_init_info,
    };

    let lp = factory.create_liquidity_pool(
        &admin.clone(),
        &lp_init_info,
        &String::from_str(env, "Pool"),
        &String::from_str(env, "PHO/XLM"),
    );

    let lp_client = lp_contract::Client::new(env, &lp);
    lp_client.provide_liquidity(
        &admin.clone(),
        &Some(token_a_amount),
        &None,
        &Some(token_b_amount),
        &None,
        &None::<i64>,
    );
}


pub struct PhoenixTest<'a> {
    pub env: Env,
    // pub router_contract: SoroswapRouterClient<'a>,
    // pub factory_contract: SoroswapFactoryClient<'a>,
    pub token_0: TokenClient<'a>,
    pub token_1: TokenClient<'a>,
    pub token_2: TokenClient<'a>,
    pub user: Address,
    pub admin: Address
}

impl<'a> PhoenixTest<'a> {
    pub fn soroswap_setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        // let router_contract = create_soroswap_router(&env);

        let initial_user_balance = 10_000_000;

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let token_0 = deploy_token_contract(&env, &admin);
        let token_1 = deploy_token_contract(&env, &admin);
        let token_2 = deploy_token_contract(&env, &admin);
    
        // token_0.mint(&user, &initial_user_balance);
        // token_1.mint(&user, &initial_user_balance);
        // token_2.mint(&user, &initial_user_balance);

        // let factory_contract = create_soroswap_factory(&env, &admin);
        // env.budget().reset_unlimited();

        // let ledger_timestamp = 100;
        // let desired_deadline = 1000;
    
        // assert!(desired_deadline > ledger_timestamp);
    
        // env.ledger().with_mut(|li| {
        //     li.timestamp = ledger_timestamp;
        // });
    
        // let amount_0: i128 = 1_000_000_000_000_000_000;
        // let amount_1: i128 = 4_000_000_000_000_000_000;
        // let amount_2: i128 = 8_000_000_000_000_000_000;
        // let expected_liquidity: i128 = 2_000_000_000_000_000_000;
    
        // // Check initial user value of every token:
        // assert_eq!(token_0.balance(&user), initial_user_balance);
        // assert_eq!(token_1.balance(&user), initial_user_balance);
        // assert_eq!(token_2.balance(&user), initial_user_balance);
    
        // router_contract.initialize(&factory_contract.address);

        // assert_eq!(factory_contract.pair_exists(&token_0.address, &token_1.address), false);
        // let (added_token_0_0, added_token_1_0, added_liquidity_0_1) = router_contract.add_liquidity(
        //     &token_0.address, //     token_a: Address,
        //     &token_1.address, //     token_b: Address,
        //     &amount_0, //     amount_a_desired: i128,
        //     &amount_1, //     amount_b_desired: i128,
        //     &0, //     amount_a_min: i128,
        //     &0 , //     amount_b_min: i128,
        //     &user, //     to: Address,
        //     &desired_deadline//     deadline: u64,
        // );

        // let (added_token_1_1, added_token_2_0, added_liquidity_1_2) = router_contract.add_liquidity(
        //     &token_1.address, //     token_a: Address,
        //     &token_2.address, //     token_b: Address,
        //     &amount_1, //     amount_a_desired: i128,
        //     &amount_2, //     amount_b_desired: i128,
        //     &0, //     amount_a_min: i128,
        //     &0 , //     amount_b_min: i128,
        //     &user, //     to: Address,
        //     &desired_deadline//     deadline: u64,
        // );

        // // let (added_token_0_1, added_token_2_1, added_liquidity_0_2) = router_contract.add_liquidity(
        // //     &token_0.address, //     token_a: Address,
        // //     &token_2.address, //     token_b: Address,
        // //     &amount_0, //     amount_a_desired: i128,
        // //     &amount_1, //     amount_b_desired: i128,
        // //     &0, //     amount_a_min: i128,
        // //     &0 , //     amount_b_min: i128,
        // //     &user, //     to: Address,
        // //     &desired_deadline//     deadline: u64,
        // // );

        // static MINIMUM_LIQUIDITY: i128 = 1000;
    
        // assert_eq!(added_token_0_0, amount_0);
        // assert_eq!(added_token_1_0, amount_1);
        // assert_eq!(added_token_1_1, amount_1);
        // assert_eq!(added_token_2_0, amount_2);
        // // assert_eq!(added_token_0_1, amount_0);
        // // assert_eq!(added_token_2_1, amount_1);

        // assert_eq!(added_liquidity_0_1, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());
        // assert_eq!(added_liquidity_1_2, 5656854249492379195);
        // // assert_eq!(added_liquidity_0_2, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());
    
        // assert_eq!(token_0.balance(&user), 19_000_000_000_000_000_000);
        // assert_eq!(token_1.balance(&user), 12_000_000_000_000_000_000);
        // assert_eq!(token_2.balance(&user), 12_000_000_000_000_000_000);

    PhoenixTest {
            env,
            // router_contract,
            // factory_contract,
            token_0,
            token_1,
            token_2,
            user,
            admin
        }
    }
}