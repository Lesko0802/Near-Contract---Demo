use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

use near_sdk::{ext_contract};

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct NearAppsContract {
    // Tags that identify the person, company, and purpose for running the contract
    

    // The name of a second Contract-B 
    pub contract_b_name: String,

    // Arguments to be used by Contract-B
    pub contract_b_args: Vec<String>,
}

    #[ext_contract(ext_contract_b)]
    trait ContractB {
    fn method_on_b(&self) -> String
    fn another_method_on_b(&self, some_arg: u64) -> U128;
    fn mutable_method_on_b(&mut self, some_arg: String);
}

impl NearAppsContract {
    pub fn new() -> Self {
        Self::default()
    }

    
}    
