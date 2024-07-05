extern crate std;
use crate::error::AggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};

#[test]
fn test_get_adapters() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.get_adapters();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_adapters_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract.try_get_adapters();
    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}