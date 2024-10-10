#![cfg(test)]
// extern crate std;
use soroban_sdk::{
    vec,
    // IntoVal,
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

/* *************  PHOENIX FACTORY  *************  */

#[allow(clippy::too_many_arguments)]
pub mod factory {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/phoenix_factory.wasm"
    );
}
use crate::test::phoenix_setup::factory::{LiquidityPoolInitInfo, StakeInitInfo, TokenInitInfo};

pub fn deploy_factory_contract(e: &Env, admin: & Address) -> Address {
    let factory_wasm = e.deployer().upload_contract_wasm(factory::WASM);
    let salt = Bytes::new(&e.clone());
    let salt = e.crypto().sha256(&salt);

    e.deployer().with_address(admin.clone(), salt).deploy(factory_wasm)
}

pub use factory::Client as PhoenixFactory;

pub use factory::PoolType;

/* *************  MULTIHOP  *************  */
#[allow(clippy::too_many_arguments)]
pub mod multihop {
    soroban_sdk::contractimport!(file = "./phoenix_contracts/phoenix_multihop.wasm");
    pub type MultihopClient<'a> = Client<'a>;
}
pub use multihop::MultihopClient; 

pub fn install_multihop_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/phoenix_multihop.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}
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
        file = "./phoenix_contracts/soroban_token_contract.wasm"
    );
}

pub use token_contract::Client as TokenClient;

// pub fn create_token_contract_with_metadata<'a>(
//     env: &Env,
//     admin: &Address,
//     decimals: u32,
//     name: String,
//     symbol: String,
//     amount: i128,
// ) -> TokenClient<'a> {
//     let token =
//         TokenClient::new(env, &env.register_contract_wasm(None, token_contract::WASM));
//     token.initialize(admin, &decimals, &name.into_val(env), &symbol.into_val(env));
//     token.mint(admin, &amount);
//     token
// }

pub fn install_token_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/soroban_token_contract.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}

pub fn deploy_token_contract<'a>(env: & Env, admin: & Address) -> token_contract::Client<'a> {
    token_contract::Client::new(env, &env.register_stellar_asset_contract(admin.clone()))
}


/* *************  STABLE CONTRACT  *************  */

#[allow(clippy::too_many_arguments)]
pub mod stable_contract {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/phoenix_pool_stable.wasm"
    );
}

pub fn install_stable_contract(env: &Env) -> BytesN<32> {
    env.deployer().upload_contract_wasm(stable_contract::WASM)
}

/* *************  LP CONTRACT  *************  */

#[allow(clippy::too_many_arguments)]
pub mod lp_contract {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/phoenix_pool.wasm"
    );
}

pub fn install_lp_contract(env: &Env) -> BytesN<32> {
    env.deployer().upload_contract_wasm(lp_contract::WASM)
}


/* *************  STAKE  *************  */

#[allow(clippy::too_many_arguments)]
pub fn install_stake_wasm(env: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./phoenix_contracts/phoenix_stake.wasm"
    );
    env.deployer().upload_contract_wasm(WASM)
}


pub fn deploy_and_mint_tokens<'a>(
    env: & Env,
    admin: & Address,
    amount: i128,
) -> token_contract::Client<'a> {
    let token = deploy_token_contract(&env, &admin);
    token.mint(&admin, &amount);
    token
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

    // fn initialize(
    //     env: Env,
    //     admin: Address,
    //     multihop_wasm_hash: BytesN<32>,
    //     lp_wasm_hash: BytesN<32>,
    //     stable_wasm_hash: BytesN<32>,
    //     stake_wasm_hash: BytesN<32>,
    //     token_wasm_hash: BytesN<32>,
    //     whitelisted_accounts: Vec<Address>,
    //     lp_token_decimals: u32,
    // );

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

    // pub struct LiquidityPoolInitInfo {
    //     pub admin: Address,
    //     pub swap_fee_bps: i64,
    //     pub fee_recipient: Address,
    //     pub max_allowed_slippage_bps: i64,
    //     pub default_slippage_bps: i64,
    //     pub max_allowed_spread_bps: i64,
    //     pub max_referral_bps: i64,
    //     pub token_init_info: TokenInitInfo,
    //     pub stake_init_info: StakeInitInfo,
    // }
    
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

    // fn create_liquidity_pool(
    //     env: Env,
    //     sender: Address,
    //     lp_init_info: LiquidityPoolInitInfo,
    //     share_token_name: String,
    //     share_token_symbol: String,
    //     pool_type: PoolType,
    //     amp: Option<u64>,
    //     default_slippage_bps: i64,
    //     max_allowed_fee_bps: i64,
    // ) -> Address;

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

    // fn provide_liquidity(
    //     env: Env,
    //     depositor: Address,
    //     desired_a: Option<i128>,
    //     min_a: Option<i128>,
    //     desired_b: Option<i128>,
    //     min_b: Option<i128>,
    //     custom_slippage_bps: Option<i64>,
    //     deadline: Option<u64>,
    // );

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


pub struct PhoenixTest<'a> {
    pub env: Env,
    pub multihop_client: MultihopClient<'a>,
    pub factory_client: PhoenixFactory<'a>,
    pub token_0: TokenClient<'a>,
    pub token_1: TokenClient<'a>,
    pub token_2: TokenClient<'a>,
    pub token_3: TokenClient<'a>,
    pub user: Address,
    pub admin: Address
}

impl<'a> PhoenixTest<'a> {
    pub fn phoenix_setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        let initial_admin_balance = 10_000_000i128;

        let token_0 = deploy_and_mint_tokens(&env, &admin, initial_admin_balance);
        let token_1 = deploy_and_mint_tokens(&env, &admin, initial_admin_balance);
        let token_2 = deploy_and_mint_tokens(&env, &admin, initial_admin_balance);
        let token_3 = deploy_and_mint_tokens(&env, &admin, initial_admin_balance);

        // 1. deploy factory
        let factory_client = deploy_and_initialize_factory(&env.clone(), admin.clone());

        deploy_and_initialize_lp(
            &env,
            &factory_client,
            admin.clone(),
            token_0.address.clone(),
            1_000_000,
            token_1.address.clone(),
            1_000_000,
            None,
        );
        deploy_and_initialize_lp(
            &env,
            &factory_client,
            admin.clone(),
            token_1.address.clone(),
            1_000_000,
            token_2.address.clone(),
            1_000_000,
            None,
        );
        deploy_and_initialize_lp(
            &env,
            &factory_client,
            admin.clone(),
            token_2.address.clone(),
            1_000_000,
            token_3.address.clone(),
            1_000_000,
            None,
        );

        // Setup multihop
        let multihop_client = deploy_multihop_contract(&env, admin.clone(), &factory_client.address);
        token_0.mint(&user, &1000i128);

        // Check initial user value of every token:

        assert_eq!(token_0.balance(&user), 1000i128);
        assert_eq!(token_1.balance(&user), 0i128);
        assert_eq!(token_2.balance(&user), 0i128);
        assert_eq!(token_3.balance(&user), 0i128);

    PhoenixTest {
            env: env.clone(),
            multihop_client,
            factory_client,
            token_0,
            token_1,
            token_2,
            token_3,
            user,
            admin: admin.clone()
        }
    }
}