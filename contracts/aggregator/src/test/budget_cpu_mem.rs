extern crate std;
use crate::test::{create_protocols_addresses, SoroswapAggregatorTest};
use crate::test::{
    new_update_adapters_addresses, create_soroswap_phoenix_addresses
};
use soroban_sdk::{String, Vec, Address, testutils::{Address as _}};
use crate::DexDistribution;


#[test]
fn budget() {
    let test = SoroswapAggregatorTest::setup();
    
    //initialize ()
    let initialize_aggregator_addresses = create_protocols_addresses(&test);

    test.env.budget().reset_unlimited();
    
    test.aggregator_contract
        .initialize(&test.admin, &initialize_aggregator_addresses);
    
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("initialize()                                             | cpu: {},      mem: {}", cpu, mem);

    // update_adapters()
    let update_aggregator_addresses = new_update_adapters_addresses(&test);

    test.env.budget().reset_unlimited();
    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("update_adapters()                                        | cpu: {},      mem: {}", cpu, mem);


    //set_pause()
    test.env.budget().reset_unlimited();
    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "soroswap"), &true);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("set_pause()                                              | cpu: {},      mem: {}", cpu, mem);
    //unpause
    test.aggregator_contract
    .set_pause(&String::from_str(&test.env, "soroswap"), &false);

   

    // set_admin
    let new_admin = Address::generate(&test.env);
    test.env.budget().reset_unlimited();
    test.aggregator_contract.set_admin(&new_admin);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("set_admin()                                              | cpu: {},      mem: {}", cpu, mem);


    // TODO: UPGRADE


    let deadline: u64 = test.env.ledger().timestamp() + 1000;


    
    //swap_exact_tokens_for_tokens TWO PROTOCOLS SOROSWAP AND PHOENIX- ONE HOP
    let update_aggregator_addresses = create_soroswap_phoenix_addresses(&test);
    test.aggregator_contract
        .update_adapters(&update_aggregator_addresses);
    // now we have soroswap and phoenix
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    let mut distribution_vec = Vec::new(&test.env);

    let expected_amount_in_0 = 30864197;
    let expected_amount_in_1 = 92592592;// total_expected_amount_in - expected_amount_in_0;


    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path: path.clone(),
        amount: expected_amount_in_0,
    };
    let distribution_1 = DexDistribution {
        protocol_id: String::from_str(&test.env, "phoenix"),
        path: path.clone(),
        amount: expected_amount_in_1,
    };
    distribution_vec.push_back(distribution_0);
    distribution_vec.push_back(distribution_1);

    let total_expected_amount_in = 123_456_789;
    test.env.budget().reset_unlimited();
    test.aggregator_contract.swap_exact_tokens_for_tokens(
        &test.token_0.address.clone(),
        &test.token_1.address.clone(),
        &total_expected_amount_in,
        &(0),
        &distribution_vec,
        &test.user.clone(),
        &deadline,
    );
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
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
        amount: 123,
    };

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.budget().reset_unlimited();
        test.aggregator_contract.swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_1.address.clone(),
            &(123*n),
            &(0),
            &distribution_vec,
            &test.user.clone(),
            &deadline,
        );
        let mem = test.env.budget().memory_bytes_cost();
        let cpu = test.env.budget().cpu_instruction_cost();
        
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
        amount: 123,
    };
    let amount_in = 123_456_789;

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.budget().reset_unlimited();
        test.aggregator_contract.swap_exact_tokens_for_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &(123*n),
            &(0),
            &distribution_vec.clone(),
            &test.user.clone(),
            &deadline,
        );
        let mem = test.env.budget().memory_bytes_cost();
        let cpu = test.env.budget().cpu_instruction_cost();
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
        amount: 123,
    };
    let expected_amount_out = 123_456_789;

    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.budget().reset_unlimited();
        test.aggregator_contract.swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_1.address.clone(),
            &(123*n),
            &i128::MAX,
            &distribution_vec,
            &test.user.clone(),
            &deadline,
        );
        let mem = test.env.budget().memory_bytes_cost();
        let cpu = test.env.budget().cpu_instruction_cost();
        
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
        amount: 123,
    };

    // makle FOR cycl N from 1 to 5r
    for n in 1..7 {
        distribution_vec.push_back(distribution_0.clone());
        test.env.budget().reset_unlimited();
        test.aggregator_contract.swap_tokens_for_exact_tokens(
            &test.token_0.address.clone(),
            &test.token_2.address.clone(),
            &(123*n),
            &i128::MAX,
            &distribution_vec,
            &test.user.clone(),
            &deadline,
        );
        let mem = test.env.budget().memory_bytes_cost();
        let cpu = test.env.budget().cpu_instruction_cost();
        
        std::println!("swap_tokens_for_exact_tokens({}_protocol_two_hop)       | cpu: {},    mem: {}", n, cpu, mem);
    
    }
    std::println!("-----------------------------------");

    


    

   

     //remove_adapter()
     test.env.budget().reset_unlimited();
     test.aggregator_contract
     .remove_adapter(&String::from_str(&test.env, "soroswap"));
     let mem = test.env.budget().memory_bytes_cost();
     let cpu = test.env.budget().cpu_instruction_cost();
     std::println!("remove_adapter()                                         | cpu: {},      mem: {}", cpu, mem);
    

    
}