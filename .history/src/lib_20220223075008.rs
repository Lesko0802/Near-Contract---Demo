use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::Serialize;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Promise};

near_sdk::setup_alloc!();

const NO_DEPOSIT: Balance = 0;

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct NearAppsContract {
    // Tags that identify the person, company, and purpose for running the contract on the blockchain 
    pub person_tag: String,
    pub company_tag: String,
    pub purpose_tag: String,
    // The name of a second Contract-B 
    pub contract_b_name: String,

    // Arguments to be used by Contract-B
    pub contract_b_args: Vec<String>,
}

//Defining trait for contractB
#[ext_contract(ext_contract_b)]
trait ContractB {
    fn method_on_b(&self, arg_1: String) -> String;
    fn another_method_on_b(&self, arg_1: String) -> U128;
    fn mutable_method_on_b(&mut self, arg_1: String);
}

// Callback
#[ext_contract(ext_self)]
trait MyContract {
    fn my_callback(&self) -> String;
}


impl NearAppsContract {

    // The constructor for the contract
    pub fn new(contract_b_name: String, contract_b_args: Vec<String>) -> Self {
        Self {
            person_tag: "Person".to_string(),
            company_tag: "Company".to_string(),
            purpose_tag: "Purpose".to_string(),
            contract_b_name,
            contract_b_args,
        }
    }

    // Create an array that stores all the names of the approved contracts
    pub fn get_approved_contracts(&self) -> Vec<String> {
        let mut approved_contracts = Vec::new();
        approved_contracts.push(self.contract_b_name.clone());
        // remove a contract from the list
        // approved_contracts.remove(some_contract_name.clone());
        approved_contracts
    }

    // Write a Function to update the approved contracts list, and add a new contract, or remove a contract
    pub fn update_approved_contracts(&mut self, new_contract_name: String) {
        // add a new contract to the list
        self.approved_contracts.push(new_contract_name);
        // remove a contract from the list
        self.approved_contracts.remove(some_contract_name);
    }

    // Check if a contract is approved
    pub fn is_approved_contract(&self, contract_name: String) -> bool {
        self.get_approved_contracts().contains(&contract_name)
    }
    
    // Check if all the tags are set
    pub fn is_tags_set(&self) -> bool {
        self.person_tag.len() > 0 && self.company_tag.len() > 0 && self.purpose_tag.len() > 0
    }

    // assert!(self.is_tags_set() && self.is_approved_contract(self.contract_b_name.clone()));    
    //NearApps will wait for a callback indicating that Contract-B has run successfully
    pub fn my_first_cross_contract_call(&self) -> Promise {
    ext_contract_b::method_on_b(
        "arg_1".to_string(),
        &"contract-b.near", // contract account id
        0, // yocto NEAR to attach
        5_000_000_000_000 // gas to attach
    )
    // When the cross contract call from A to B finishes the my_callback method is triggered.
    // Since my_callback is a callback, it will have access to the returned data from B
    .then(ext_self::my_callback(
        &env::current_account_id(), // this contract's account id
        0, // yocto NEAR to attach to the callback
        5_000_000_000_000 // gas to attach to the callback
    ))
    }

    pub fn my_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );
    }
}
