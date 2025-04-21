#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod event;
mod storage;
mod protocol_interface;
mod test;

use storage::{
    extend_instance_ttl, 
    set_initialized, 
    is_initialized, 
    set_protocol_id,
    get_protocol_id,
    set_protocol_address, 
    get_protocol_address, 
};
use adapter_interface::{AdapterTrait, AdapterError};
use protocol_interface::{protocol_swap_exact_tokens_for_tokens,
    protocol_swap_tokens_for_exact_tokens};

fn check_initialized(e: &Env) -> Result<(), AdapterError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(AdapterError::NotInitialized)
    }
}

fn check_deadline(e: &Env, deadline: u64) -> Result<(), AdapterError> {
    if e.ledger().timestamp() >= deadline{
        Err(AdapterError::DeadlineExpired)
    }else{
        Ok(())
    }
}

#[contract]
struct CometAggregatorAdapter;

#[contractimpl]
impl AdapterTrait for CometAggregatorAdapter {
    fn initialize(
        e: Env,
        protocol_id: String,
        protocol_address: Address,
    ) -> Result<(), AdapterError> {
        if check_initialized(&e).is_ok() {
            return Err(AdapterError::AlreadyInitialized);
        }
    
        set_protocol_id(&e, protocol_id.clone());
        set_protocol_address(&e, protocol_address.clone());
    
        set_initialized(&e);
        event::initialized(&e, true, protocol_id, protocol_address);
        extend_instance_ttl(&e);
        Ok(())
    }
    
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        check_deadline(&e, deadline)?;
        to.require_auth();

        let swap_result = protocol_swap_exact_tokens_for_tokens(
            &e, 
            &amount_in, 
            &amount_out_min, 
            &path, 
            &to, 
        )?;

        event::swap(&e, amount_in, path, to);
        Ok(swap_result)
    }

    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        check_deadline(&e, deadline)?;
        to.require_auth();

        let swap_result = protocol_swap_tokens_for_exact_tokens(
            &e, 
            &amount_out, 
            &amount_in_max, 
            &path, 
            &to, 
        )?;

        event::swap(&e, amount_in_max, path, to);
        Ok(swap_result)
    }

    /*  *** Read only functions: *** */
    fn get_protocol_id(e: &Env) -> Result<String, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_protocol_id(e)?)
    }    
    
    fn get_protocol_address(e: &Env) -> Result<Address, AdapterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_protocol_address(e)?)
    }    
}
