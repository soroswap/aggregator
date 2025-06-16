extern crate std;
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, generate_adapter_objects_for_deployer};

#[test]
fn test_get_adapters() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = generate_adapter_objects_for_deployer(
        &test.env, 
        test.soroswap_router_address.clone(), 
        test.phoenix_multihop_address.clone(), 
        test.comet_router_address.clone(),
        test.aqua_setup.router.address.clone(),
    );

    let result = test.aggregator_contract.get_adapters();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_adapters_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract_not_initialized.try_get_adapters();
    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}
