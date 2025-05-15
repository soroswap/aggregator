#![cfg(test)]
extern crate alloc;
extern crate std;

use crate::{Deployer, DeployerClient};
use alloc::vec;
use soroban_sdk::{
    symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, xdr::{self, ContractIdPreimage, ContractIdPreimageFromAddress, CreateContractArgsV2, Uint256}, Address, BytesN, Env, IntoVal, String, Symbol, Val, Vec, vec as sorovec,
    xdr::VecM
};
use soroswap_aggregator_contract::Adapter;

// Defining contracts
mod soroswap_adapter_contract {
    soroban_sdk::contractimport!(
        file =
            "../target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm"
    );
}

mod phoenix_adapter_contract {
  soroban_sdk::contractimport!(
      file =
          "../target/wasm32-unknown-unknown/release/phoenix_adapter.optimized.wasm"
  );
}

mod soroswap_aggregator_contract {
  soroban_sdk::contractimport!(
      file =
          "../target/wasm32-unknown-unknown/release/soroswap_aggregator.optimized.wasm"
  );
}

#[test]
fn test_deploy_from_contract_soroswap_adapter() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register(Deployer, ()));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(soroswap_adapter_contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = Symbol::new(&env, &("initialize"));

    let protocol_id = String::from_str(&env, "soroswap");
    let protocol_address = Address::generate(&env);

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&env);

    env.mock_all_auths();
    let (contract_id, init_result) = deployer_client.deploy(
        &deployer_client.address,
        &wasm_hash,
        &salt,
        &init_fn,
        &init_fn_args,
    );

    assert!(init_result.is_void());
    // No authorizations needed - the contract acts as a factory.
    assert_eq!(env.auths(), vec![]);

    // Invoke contract to check that it is initialized.
    let client = soroswap_adapter_contract::Client::new(&env, &contract_id);
    let get_protocol_id = client.get_protocol_id();
    assert_eq!(get_protocol_id, protocol_id);

    let get_protocol_address = client.get_protocol_address();
    assert_eq!(get_protocol_address, protocol_address);
}

#[test]
fn test_deploy_from_address_soroswap_adapter() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register(Deployer, ()));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(soroswap_adapter_contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = Symbol::new(&env, &("initialize"));
    
    let protocol_id = String::from_str(&env, "soroswap");
    let protocol_address = Address::generate(&env);

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&env);

    env.mock_all_auths();
    let (contract_id, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn, &init_fn_args);

    assert!(init_result.is_void());

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            deployer_client.address,
            symbol_short!("deploy"),
            (
                deployer.clone(),
                wasm_hash.clone(),
                salt,
                init_fn,
                init_fn_args,
            )
                .into_val(&env),
        )),
        // From `deploy` function the 'create contract' host function has to be
        // authorized.
        sub_invocations: vec![AuthorizedInvocation {
            function: AuthorizedFunction::CreateContractV2HostFn(
                CreateContractArgsV2 {
                contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
                    address: deployer.clone().try_into().unwrap(),
                    salt: Uint256([0; 32]),
                    
                }),
                executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into_val(&env))),
                constructor_args: VecM::default(),
            }),
            
            
            sub_invocations: vec![],
        }],
    };
    assert_eq!(env.auths(), vec![(deployer, expected_auth)]);

    // Invoke contract to check that it is initialized.
    let client = soroswap_adapter_contract::Client::new(&env, &contract_id);
    let get_protocol_id = client.get_protocol_id();
    assert_eq!(get_protocol_id, protocol_id);

    let get_protocol_address = client.get_protocol_address();
    assert_eq!(get_protocol_address, protocol_address);
}

#[test]
fn test_deploy_from_address_phoenix_adapter() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register(Deployer, ()));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(phoenix_adapter_contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = Symbol::new(&env, &("initialize"));
    
    let protocol_id = String::from_str(&env, "phoenix");
    let protocol_address = Address::generate(&env);

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&env);

    env.mock_all_auths();
    let (contract_id, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn, &init_fn_args);

    assert!(init_result.is_void());

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            deployer_client.address,
            symbol_short!("deploy"),
            (
                deployer.clone(),
                wasm_hash.clone(),
                salt,
                init_fn,
                init_fn_args,
            )
                .into_val(&env),
        )),
        // From `deploy` function the 'create contract' host function has to be
        // authorized.
        sub_invocations: vec![AuthorizedInvocation {
            function: AuthorizedFunction::CreateContractV2HostFn(CreateContractArgsV2 {
                contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
                    address: deployer.clone().try_into().unwrap(),
                    salt: Uint256([0; 32]),
                }),
                executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into_val(&env))),
                constructor_args: VecM::default(),
            }),
            sub_invocations: vec![],
        }],
    };
    assert_eq!(env.auths(), vec![(deployer, expected_auth)]);

    // Invoke contract to check that it is initialized.
    let client = phoenix_adapter_contract::Client::new(&env, &contract_id);
    let get_protocol_id = client.get_protocol_id();
    assert_eq!(get_protocol_id, protocol_id);

    let get_protocol_address = client.get_protocol_address();
    assert_eq!(get_protocol_address, protocol_address);
}

#[test]
fn test_deploy_from_address_soroswap_aggregator() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register(Deployer, ()));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(soroswap_aggregator_contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = Symbol::new(&env, &("initialize"));
    
    // fn initialize(e: Env, admin: Address, adapter_vec: Vec<Adapter>)
    // -> Result<(), AggregatorError>;

    let adapter_address = Address::generate(&env);

    let adapter = Adapter {
      protocol_id: String::from_str(&env, "soroswap"),
      address: adapter_address.clone(),
      paused: false,
  };

    let adapter_vec: Vec<Adapter> = sorovec![&env, adapter];

    // Convert the arguments into a Vec<Val>
    let init_fn_args: Vec<Val> = (deployer.clone(), adapter_vec.clone()).into_val(&env);

    env.mock_all_auths();
    let (contract_id, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn, &init_fn_args);

    assert!(init_result.is_void());

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            deployer_client.address,
            symbol_short!("deploy"),
            (
                deployer.clone(),
                wasm_hash.clone(),
                salt,
                init_fn.clone(),
                init_fn_args.clone(),
            )
                .into_val(&env),
        )),
        // From `deploy` function the 'create contract' host function has to be
        // authorized.
        sub_invocations: vec![
          AuthorizedInvocation {
            function: AuthorizedFunction::CreateContractV2HostFn(CreateContractArgsV2 {
                contract_id_preimage: ContractIdPreimage::Address(ContractIdPreimageFromAddress {
                    address: deployer.clone().try_into().unwrap(),
                    salt: Uint256([0; 32]),
                }),
                executable: xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into_val(&env))),
                constructor_args: VecM::default(),
            }),
            sub_invocations: vec![],
        },
          AuthorizedInvocation {
          function: AuthorizedFunction::Contract((contract_id.clone(), init_fn, init_fn_args)),
          sub_invocations: vec![],
      }],
    };
    // AuthorizedInvocation { function: Contract((Contract(CBRIAA73VOIKPZYM5G3LGPF3NGCFXLR3IW22MKEYJAB3QBOMTUTRCASK), Symbol(initialize), Vec(Ok(Address(obj#123)), Ok(Vec(obj#135))))), sub_invocations: [] }] })]
    assert_eq!(env.auths(), vec![(deployer.clone(), expected_auth)]);

    // Invoke contract to check that it is initialized.
    let client = soroswap_aggregator_contract::Client::new(&env, &contract_id);

    // Checking admin
    let get_aggregator_admin = client.get_admin();
    assert_eq!(get_aggregator_admin, deployer);

    // Checking Adapters
    let get_aggregator_adapters = client.get_adapters();
    assert_eq!(get_aggregator_adapters, adapter_vec);


}