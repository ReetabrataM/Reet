#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {

    // Initialize counter
    pub fn init(env: Env) {
        env.storage().instance().set(&COUNTER, &0u32);
    }

    // Increment counter
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0);

        count += 1;

        env.storage().instance().set(&COUNTER, &count);
        count
    }

    // Get current value
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0)
    }
}