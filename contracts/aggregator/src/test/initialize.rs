extern crate std;
use crate::error::AggregatorError;
use crate::test::{create_protocols_addresses, SoroswapAggregatorTest};

#[test]
fn test_initialize_and_get_values() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);

    test.env.budget().reset_default();
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);
    
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("initialize() - cpu: {}, mem: {}", cpu, mem);
    test.env.budget().print();

    // get admin
    let admin = test.aggregator_contract.get_admin();
    assert_eq!(admin, test.admin);

    // get protocols
    let protocols = test.aggregator_contract.get_adapters();
    assert_eq!(protocols, initialize_aggregator_addresses);

    // get is protocol paused
    for protocol_address in initialize_aggregator_addresses {
        let is_protocol_paused = test
            .aggregator_contract
            .get_paused(&protocol_address.protocol_id.clone());
        assert_eq!(is_protocol_paused, false);
    }
}

#[test]
fn test_get_admin_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();
    let result = test.aggregator_contract.try_get_admin();

    assert_eq!(result, Err(Ok(AggregatorError::NotInitialized)));
}

#[test]
fn test_initialize_twice() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);

    let result_second_init = test
        .aggregator_contract
        .try_initialize(&test.admin, &initialize_aggregator_addresses);
    assert_eq!(
        result_second_init,
        (Err(Ok(AggregatorError::AlreadyInitialized)))
    );
}
