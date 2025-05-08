extern crate std;
use crate::test::{create_soroswap_phoenix_comet_addresses_for_deployer, new_update_adapters_addresses_deployer, SoroswapAggregatorTest};
use soroban_sdk::{String, Vec, Address, testutils::Address as _};
use super::soroswap_aggregator_contract::DexDistribution;


#[test]
fn budget() {
    let test = SoroswapAggregatorTest::setup();
    
    //initialize ()
    // let initialize_aggregator_addresses = create_protocols_addresses(&test);

    test.env.cost_estimate().budget().reset_unlimited();
    
    // test.aggregator_contract_not_initialized
    //     .initialize(&test.admin, &initialize_aggregator_addresses);
    
    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("initialize()                                             | cpu: {},      mem: {}", cpu, mem);

    // update_adapters()
    let update_aggregator_addresses = new_update_adapters_addresses_deployer(&test);

    test.env.cost_estimate().budget().reset_unlimited();
    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);
    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("update_adapters()                                        | cpu: {},      mem: {}", cpu, mem);


    //set_pause()
    test.env.cost_estimate().budget().reset_unlimited();
    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "soroswap"), &true);
    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("set_pause()                                              | cpu: {},      mem: {}", cpu, mem);
    //unpause
    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "soroswap"), &false);

   

    // set_admin
    let new_admin = Address::generate(&test.env);
    test.env.cost_estimate().budget().reset_unlimited();
    test.aggregator_contract.set_admin(&new_admin);
    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("set_admin()                                              | cpu: {},      mem: {}", cpu, mem);


    // TODO: UPGRADE


    let deadline: u64 = test.env.ledger().timestamp() + 1000;


    
    //swap_exact_tokens_for_tokens TWO PROTOCOLS SOROSWAP AND PHOENIX- ONE HOP
    let update_aggregator_addresses = create_soroswap_phoenix_comet_addresses_for_deployer(&test.env, test.soroswap_adapter_contract.address.clone(), test.phoenix_adapter_contract.address.clone(), test.comet_adapter_contract.address.clone());

    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);
    // now we have soroswap and phoenix
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    let distribution_1 = DexDistribution {
        protocol_id: String::from_str(&test.env, "phoenix"),
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_in = 123_456_789;
    test.env.cost_estimate().budget().reset_unlimited();
    test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &(0),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("swap_exact_tokens_for_tokens(s_p_protocol_one_hop)       | cpu: {},    mem: {}", cpu, mem);
    std::println!("-----------------------------------");

    //swap_exact_tokens_for_tokens N PROTOCOLS - ONE HOP
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
        bytes: None
    };

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.cost_estimate().budget().reset_unlimited();
        test.aggregator_contract.swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_1.address.clone(),
            &total_expected_amount_in,
            &(0),
            &distribution_vec,
            &test.user.clone(),
            &deadline,

        );
        let mem = test.env.cost_estimate().budget().memory_bytes_cost();
        let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
        
        std::println!("swap_exact_tokens_for_tokens({}_protocol_one_hop)       | cpu: {},    mem: {}", n, cpu, mem);
    
    }
    std::println!("-----------------------------------");



    // swap_exact_tokens_for_tokens N PROTOCOL TWO HOPS
    let mut distribution_vec = Vec::new(&test.env);
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path,
        parts: 1,
        bytes: None
    };
    let amount_in = 123_456_789;

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.cost_estimate().budget().reset_unlimited();
        test.aggregator_contract.swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &amount_in,
            &(0),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,

        );
        let mem = test.env.cost_estimate().budget().memory_bytes_cost();
        let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
        std::println!("swap_exact_tokens_for_tokens({}_protocol_two_hop)       | cpu: {},    mem: {}", n, cpu, mem);
    }
    std::println!("-----------------------------------");

    
    // swap_tokens_for_exact_tokens() N PROTOCOLS ONE HOP
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
        bytes: None
    };
    let expected_amount_out = 123_456_789;

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.cost_estimate().budget().reset_unlimited();
        test.aggregator_contract.swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_1.address.clone(),
            &expected_amount_out,
            &i128::MAX,
            &distribution_vec,
            &test.user.clone(),
            &deadline,

        );
        let mem = test.env.cost_estimate().budget().memory_bytes_cost();
        let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
        
        std::println!("swap_tokens_for_exact_tokens({}_protocol_one_hop)       | cpu: {},    mem: {}", n, cpu, mem);
    
    }
    std::println!("-----------------------------------");


    // swap_tokens_for_exact_tokens() N PROTOCOLS TWO HOP
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(test.token_2.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        parts: 1,
        bytes: None
    };

    // makle FOR cycl N from 1 to 5r
    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.cost_estimate().budget().reset_unlimited();
        test.aggregator_contract.swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &expected_amount_out,
            &i128::MAX,
            &distribution_vec,
            &test.user.clone(),
            &deadline,

        );
        let mem = test.env.cost_estimate().budget().memory_bytes_cost();
        let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
        
        std::println!("swap_tokens_for_exact_tokens({}_protocol_two_hop)       | cpu: {},    mem: {}", n, cpu, mem);
    
    }
    std::println!("-----------------------------------");

    


    

   

     //remove_adapter()
     test.env.cost_estimate().budget().reset_unlimited();
     test.aggregator_contract
        .remove_adapter(&String::from_str(&test.env, "soroswap"));
     let mem = test.env.cost_estimate().budget().memory_bytes_cost();
     let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
     std::println!("remove_adapter()                                         | cpu: {},      mem: {}", cpu, mem);
    

    
}