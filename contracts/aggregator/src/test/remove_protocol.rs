extern crate std;
use soroban_sdk::{Address, Vec, vec, String, testutils::Address as _};
use soroban_sdk::{
    IntoVal,
    testutils::{
        MockAuth,
        MockAuthInvoke,
        AuthorizedInvocation,
        AuthorizedFunction
    },
    Symbol
};
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, create_soroswap_router};
use crate::models::{ProxyAddressPair};

pub fn new_protocol_vec(
    test: &SoroswapAggregatorTest,
    protocol_id: &String,
) -> Vec<ProxyAddressPair> {
    let new_router = create_soroswap_router(&test.env);
    vec![&test.env,
        ProxyAddressPair {
            protocol_id: protocol_id.clone(),
            address: new_router.address,
        },
    ]
}

#[test]
fn test_remove_protocol() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);
    
    test.aggregator_contract.remove_protocol(&String::from_str(&test.env, "soroswap"));

    let mut updated_protocols = test.aggregator_contract.get_protocols();
    let expected_empty_vec = vec![&test.env];
    assert_eq!(updated_protocols, expected_empty_vec);

    //add new protocol
    let new_protocol_0 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_0"));
    test.aggregator_contract.update_protocols(&new_protocol_0);
    
    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, new_protocol_0);

    // add new protoco 1
    let new_protocol_1 = new_protocol_vec(&test, &String::from_str(&test.env, "new_protocol_1"));
    test.aggregator_contract.update_protocols(&new_protocol_1);

    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols.get(0), new_protocol_0.get(0));
    assert_eq!(updated_protocols.get(1), new_protocol_1.get(0));

    // remove new protocol 0
    test.aggregator_contract.remove_protocol(&String::from_str(&test.env, "new_protocol_0"));
    updated_protocols = test.aggregator_contract.get_protocols();
    assert_eq!(updated_protocols, new_protocol_1);

}

// test non initialized
#[test]
fn test_remove_protocol_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract.try_remove_protocol(&String::from_str(&test.env, "soroswap"));

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

// update protocols can only be called by admin

#[test]
fn test_update_protocols_with_mock_auth() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    // check initial protocol values
    let protocols = test.aggregator_contract.get_protocols();
    assert_eq!(protocols, initialize_aggregator_addresses);

    let protocol_id_to_remove = String::from_str(&test.env, "soroswap");    
    
    //  MOCK THE SPECIFIC AUTHORIZATION
    test.aggregator_contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &test.aggregator_contract.address.clone(),
                    fn_name: "remove_protocol",
                    args: (protocol_id_to_remove.clone(),).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .remove_protocol(&protocol_id_to_remove.clone());
    
    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
               function: AuthorizedFunction::Contract((
                   test.aggregator_contract.address.clone(),
                   Symbol::new(&test.env, "remove_protocol"),
                   (protocol_id_to_remove.clone(),).into_val(&test.env)
               )),
               sub_invocations: std::vec![]
           }
        )]
   );
}