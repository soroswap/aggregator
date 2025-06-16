#![cfg(test)]
extern crate std;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Address, BytesN, Env, Vec, Symbol, Val, IntoVal
};

use crate::models::Adapter;
use crate::models::Protocol;
use crate::{SoroswapAggregator, SoroswapAggregatorClient};

// Soroswap
mod soroswap_setup;
use soroswap_setup::{
    create_soroswap_adapter, 
    create_soroswap_factory, 
    create_soroswap_router,
    SoroswapAggregatorAdapterForSoroswapClient,
    SoroswapRouterClient,
};

// Phoenix
// mod phoenix_setup;
use test_utils::phoenix_setup::{
    create_deployer,
    create_phoenix_adapter,
    deploy_and_initialize_factory as phoenix_deploy_and_initialize_factory,
    deploy_and_initialize_lp as phoenix_deploy_and_initialize_lp,
    deploy_multihop_contract as phoenix_deploy_multihop_contract,
    SoroswapAggregatorAdapterForPhoenixClient
};


// Comet
mod comet_setup;
use comet_setup::comet_adapter::CometAdapterClient;
use comet_setup::{create_comet_adapter, create_comet_factory, deploy_and_init_comet_pool};


// Aqua
mod aqua_setup;
use aqua_setup::{AquaSetup};





// mod deployer_contract {
//     soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/soroswap_aggregator_deployer.optimized.wasm");
//     pub type DeployerClient<'a> = Client<'a>;
// }
// pub use deployer_contract::DeployerClient;

// fn create_deployer<'a>(e: &Env) -> DeployerClient<'a> {
//     let deployer_address = &e.register(deployer_contract::WASM, ());
//     let deployer = DeployerClient::new(e, deployer_address);
//     deployer
// }

// SoroswapAggregator Contract [THE MAIN CONTRACT]
fn create_soroswap_aggregator<'a>(e: &Env) -> SoroswapAggregatorClient<'a> {
    SoroswapAggregatorClient::new(e, &e.register(SoroswapAggregator {}, ()))
}

pub mod soroswap_aggregator_contract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/soroswap_aggregator.optimized.wasm");
    pub type SoroswapAggregatorClientFromWasm<'a> = Client<'a>;
}
use soroswap_aggregator_contract::{SoroswapAggregatorClientFromWasm, Adapter as AdapterFromWasm};


// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../adapters/soroswap/soroswap_contracts/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;
pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

// pub fn install_token_wasm(env: &Env) -> BytesN<32> {
//     soroban_sdk::contractimport!(
//         file = "../adapters/soroswap/soroswap_contracts/soroban_token_contract.wasm"
//     );
//     env.deployer().upload_contract_wasm(WASM)
// }


// Helper function to initialize / update soroswap aggregator protocols
pub fn create_protocols_addresses_from_wasm(test: &SoroswapAggregatorTest) -> Vec<AdapterFromWasm> {
    vec![
        &test.env,
        AdapterFromWasm {
            protocol_id: soroswap_aggregator_contract::Protocol::Soroswap,
            router: test.soroswap_router_address.clone(),
            paused: false,
        }
    ]
}

pub fn create_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
    vec![
        &test.env,
        Adapter {
            protocol_id: Protocol::Soroswap,
            router: test.soroswap_router_address.clone(),
            paused: false,
        }
    ]
}

// pub fn create_soroswap_phoenix_addresses(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![
//         &test.env,
//         Adapter {
//             protocol_id: Protocol::Soroswap,
//             router: test.soroswap_adapter_contract.address.clone(),

//             paused: false,
//         },
//         Adapter {
//             protocol_id: String::from_str(&test.env, "phoenix"),
//             router: test.phoenix_adapter_contract.address.clone(),
//             paused: false,
//         },
//     ]
// }

pub fn generate_adapter_objects_for_deployer(
    env: &Env, 
    soroswap_router_address: Address, 
    phoenix_multihop_address: Address, 
    comet_router_address: Address,
    aqua_router_address: Address,
) -> Vec<AdapterFromWasm> {
    vec![
        env,
        AdapterFromWasm {
            protocol_id: soroswap_aggregator_contract::Protocol::Soroswap,
            router: soroswap_router_address.clone(),
            paused: false,
        },
        AdapterFromWasm {
            protocol_id: soroswap_aggregator_contract::Protocol::Phoenix,
            router: phoenix_multihop_address.clone(),
            paused: false,
        },
        AdapterFromWasm {
            protocol_id: soroswap_aggregator_contract::Protocol::Comet,
            router: comet_router_address.clone(),
            paused: false,
        },
        AdapterFromWasm {
            protocol_id: soroswap_aggregator_contract::Protocol::Aqua,
            router: aqua_router_address.clone(),
            paused: false,
        },
    ]
}

// pub fn new_update_adapters_addresses(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![
//         &test.env,
//         Adapter {
//             protocol_id: Protocol::Soroswap,
//             router: test.soroswap_router_address.clone(),
//             paused: false,
//         },
//     ]
// }

// pub fn new_update_adapters_addresses_deployer(test: &SoroswapAggregatorTest) -> Vec<AdapterFromWasm> {
//     vec![
//         &test.env,
//         AdapterFromWasm {
//             protocol_id: soroswap_aggregator_contract::Protocol::Soroswap,
//             router: test.soroswap_router_address.clone(),
//             paused: false,
//         },
//     ]
// }

// pub fn create_only_soroswap_protocol_address(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![&test.env,
//         Adapter {
//             protocol_id: dex_constants::SOROSWAP,
//             router: test.soroswap_router_contract.address.clone(),
//         },
//     ]
// }

// pub fn create_only_phoenix_protocol_address(test: &SoroswapAggregatorTest) -> Vec<Adapter> {
//     vec![&test.env,
//         Adapter {
//             protocol_id: dex_constants::PHOENIX,
//             router: test.soroswap_router_contract.address.clone(),
//         },
//     ]
// }

pub fn generate_salt(initial: u8) -> [u8; 32] {
    let mut salt = [0u8; 32];
    salt[0] = initial;
    salt
}

pub struct SoroswapAggregatorTest<'a> {
    env: Env,
    aggregator_contract: SoroswapAggregatorClientFromWasm<'a>,
    aggregator_contract_not_initialized: SoroswapAggregatorClient<'a>,
    soroswap_router_contract: SoroswapRouterClient<'a>,
    // soroswap_factory_contract: SoroswapFactoryClient<'a>,
    soroswap_adapter_contract: SoroswapAggregatorAdapterForSoroswapClient<'a>,
    phoenix_adapter_contract: SoroswapAggregatorAdapterForPhoenixClient<'a>,
    comet_adapter_contract: CometAdapterClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    user: Address,
    admin: Address,
    soroswap_router_address: Address,
    phoenix_multihop_address: Address,
    comet_router_address: Address,
    aqua_setup: AquaSetup<'a>,
}


impl<'a> SoroswapAggregatorTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let aggregator_contract_not_initialized = create_soroswap_aggregator(&env);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let token_0 = create_token_contract(&env, &admin);
        let token_1 = create_token_contract(&env, &admin);
        let token_2 = create_token_contract(&env, &admin);
        let token_3 = create_token_contract(&env, &admin);

        let initial_user_balance = 20_000_000_000_000_000_000;
        token_0.mint(&user, &initial_user_balance);
        token_1.mint(&user, &initial_user_balance);
        token_2.mint(&user, &initial_user_balance);
        token_3.mint(&user, &initial_user_balance);

        token_0.mint(&admin, &initial_user_balance);
        token_1.mint(&admin, &initial_user_balance);
        token_2.mint(&admin, &initial_user_balance);
        token_3.mint(&admin, &initial_user_balance);
    
        
        /*  INITIALIZE SOROSWAP FACTORY, ROUTER AND LPS */
        /************************************************/
        env.cost_estimate().budget().reset_unlimited();
        let soroswap_router_contract = create_soroswap_router(&env);
        let soroswap_factory_contract = create_soroswap_factory(&env, &admin);
        soroswap_router_contract.initialize(&soroswap_factory_contract.address);

        let ledger_timestamp = 100;
        let desired_deadline = 1000;
        assert!(desired_deadline > ledger_timestamp);
        env.ledger().with_mut(|li| {
            li.timestamp = ledger_timestamp;
        });

        let amount_0: i128 = 1_000_000_000_000_000_000;
        let amount_1: i128 = 4_000_000_000_000_000_000;
        let amount_2: i128 = 8_000_000_000_000_000_000;
        let expected_liquidity: i128 = 2_000_000_000_000_000_000;

        // Check initial user value of every token:
        assert_eq!(token_0.balance(&user), initial_user_balance);
        assert_eq!(token_1.balance(&user), initial_user_balance);
        assert_eq!(token_2.balance(&user), initial_user_balance);


        assert_eq!(
            soroswap_factory_contract.pair_exists(&token_0.address, &token_1.address),
            false
        );
        let (added_token_0_0, added_token_1_0, added_liquidity_0_1) = soroswap_router_contract
            .add_liquidity(
                &token_0.address,  //     token_a: Address,
                &token_1.address,  //     token_b: Address,
                &amount_0,         //     amount_a_desired: i128,
                &amount_1,         //     amount_b_desired: i128,
                &0,                //     amount_a_min: i128,
                &0,                //     amount_b_min: i128,
                &user,             //     to: Address,
                &desired_deadline, //     deadline: u64,
            );

        let (added_token_1_1, added_token_2_0, added_liquidity_1_2) = soroswap_router_contract
            .add_liquidity(
                &token_1.address,  //     token_a: Address,
                &token_2.address,  //     token_b: Address,
                &amount_1,         //     amount_a_desired: i128,
                &amount_2,         //     amount_b_desired: i128,
                &0,                //     amount_a_min: i128,
                &0,                //     amount_b_min: i128,
                &user,             //     to: Address,
                &desired_deadline, //     deadline: u64,
            );

        static MINIMUM_LIQUIDITY: i128 = 1000;

        assert_eq!(added_token_0_0, amount_0);
        assert_eq!(added_token_1_0, amount_1);
        assert_eq!(added_token_1_1, amount_1);
        assert_eq!(added_token_2_0, amount_2);

        assert_eq!(
            added_liquidity_0_1,
            expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap()
        );
        assert_eq!(added_liquidity_1_2, 5656854249492379195);
        // assert_eq!(added_liquidity_0_2, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());

        assert_eq!(token_0.balance(&user), 19_000_000_000_000_000_000);
        assert_eq!(token_1.balance(&user), 12_000_000_000_000_000_000);
        assert_eq!(token_2.balance(&user), 12_000_000_000_000_000_000);


        /* INITIALIZE PHOENIX FACTORY, LP AND MULTIHOP */
        /************************************************/
        let phoenix_factory_client = phoenix_deploy_and_initialize_factory(&env.clone(), admin.clone());


        phoenix_deploy_and_initialize_lp(
            &env,
            &phoenix_factory_client,
            admin.clone(),
            token_0.address.clone(),
            1_000_000_000_000_000_000,
            token_1.address.clone(),
            1_000_000_000_000_000_000,
            None,
        );
        phoenix_deploy_and_initialize_lp(
            &env,
            &phoenix_factory_client,
            admin.clone(),
            token_1.address.clone(),
            1_000_000_000_000_000_000,
            token_2.address.clone(),
            1_000_000_000_000_000_000,
            None,
        );
        phoenix_deploy_and_initialize_lp(
            &env,
            &phoenix_factory_client,
            admin.clone(),
            token_2.address.clone(),
            1_000_000_000_000_000_000,
            token_3.address.clone(),
            1_000_000_000_000_000_000,
            None,
        );

        let phoenix_multihop_client = phoenix_deploy_multihop_contract(
            &env,
            admin.clone(),
            &phoenix_factory_client.address);

        /* INITIALIZE COMET FACTORY AND LP */
        /************************************************/
        let comet_factory: comet_setup::factory::Client<'_> = create_comet_factory(&env);
        let comet_pair = deploy_and_init_comet_pool(&env, &admin, &vec![&env, token_0.address.clone(), token_2.address.clone()], comet_factory);

        // SETTING UP DEPLOYER
        let deployer_client = create_deployer(&env);
        let salt = BytesN::from_array(&env, &generate_salt(2));
        let init_fn = Symbol::new(&env, &("initialize"));

        /* CREATE ADAPTERS */
        let soroswap_adapter_contract = create_soroswap_adapter(&env, &deployer_client, soroswap_router_contract.address.clone(), admin.clone());

        let phoenix_adapter_contract = create_phoenix_adapter(&env, &deployer_client, phoenix_multihop_client.address.clone(), admin.clone());

        let comet_adapter_contract = create_comet_adapter(&env, &deployer_client, comet_pair.address.clone(), admin.clone());

        let wasm_hash = env.deployer().upload_contract_wasm(soroswap_aggregator_contract::WASM);


        // AQUA
        let aqua_setup = AquaSetup::aqua_setup(&env);

        // Deploy aggregator using deployer, and include an init function to call.
        let initialize_aggregator_addresses = vec![
            &env,
            AdapterFromWasm {
                protocol_id: soroswap_aggregator_contract::Protocol::Soroswap,
                router: soroswap_router_contract.address.clone(),
                paused: false,
            },
            AdapterFromWasm {
                protocol_id: soroswap_aggregator_contract::Protocol::Phoenix,
                router: phoenix_multihop_client.address.clone(),
                paused: false,
            },
            AdapterFromWasm {
                protocol_id: soroswap_aggregator_contract::Protocol::Comet,
                router: comet_pair.address.clone(),
                paused: false,
            },
            AdapterFromWasm {
                protocol_id: soroswap_aggregator_contract::Protocol::Aqua,
                router: aqua_setup.router.address.clone(),
                paused: false,
            },
        ];

        // Convert the arguments into a Vec<Val>
        let init_fn_args: Vec<Val> = (admin.clone(), initialize_aggregator_addresses).into_val(&env);

        let (contract_id, _init_result) = deployer_client.deploy(
            &admin.clone(),
            &wasm_hash,
            &salt,
            &init_fn,
            &init_fn_args,
        );

        let aggregator_contract = SoroswapAggregatorClientFromWasm::new(&env, &contract_id);
        let soroswap_router_address = soroswap_router_contract.address.clone();

        SoroswapAggregatorTest {
            env,
            aggregator_contract,
            aggregator_contract_not_initialized,
            soroswap_router_contract,
            // soroswap_factory_contract,
            soroswap_adapter_contract,
            phoenix_adapter_contract,
            comet_adapter_contract,
            token_0,
            token_1,
            token_2,
            user,
            admin,
            soroswap_router_address, 
            phoenix_multihop_address: phoenix_multihop_client.address.clone(),
            comet_router_address: comet_pair.address.clone(),
            aqua_setup,
        }
    }
}

// pub mod events;
pub mod get_adapters;
pub mod initialize;
// pub mod remove_adapter;
pub mod set_pause_get_paused;
pub mod swap_exact_tokens_for_tokens;
pub mod swap_tokens_for_exact_tokens;
// pub mod update_adapters;
//     pub mod budget_cpu_mem;
// // pub mod swap;
//     pub mod admin;
// test upgrade wasm

