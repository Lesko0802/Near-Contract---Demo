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

pub struct NearAppsTags {
    // Tags that identify the person, company, and purpose for running the contract on the blockchain
    pub person_tag: String,
    pub company_tag: String,
    pub purpose_tag: String,
}

pub struct NearAppsContract { 
    // The name of a second Contract-B 
    pub contract_b_name: String,

    // Arguments to be used by Contract-B
    pub contract_b_args: Vec<String>,
}

//Defining trait for contractB
#[ext_contract(ext_contract_b)]
trait ContractB {
    fn method_on_b(&self, is_tags: bool) -> String;
    fn another_method_on_b(&self, is_tags: bool) -> U128;
    fn mutable_method_on_b(&mut self, is_tags: bool);
}

// Callback
#[ext_contract(ext_self)]
trait MyContract {
    fn my_callback(&self) -> String;
}

pub mod gas {
    use near_sdk::Gas;

    /// The base amount of gas for a regular execution.
    const BASE: Gas = 25_000_000_000_000;

    /// The amount of Gas the contract will attach to the promise to create the lockup.
    pub const LOCKUP_NEW: Gas = BASE;

    /// The amount of Gas the contract will attach to the callback to itself.
    /// The base for the execution and the base for cash rollback.
    pub const CALLBACK: Gas = BASE;
}

const MIN_ATTACHED_BALANCE: Balance = 3_500_000_000_000_000_000_000_000;

impl NearAppsContract {

    // The constructor for the contract
    pub fn new(contract_b_name: String, contract_b_args: Vec<String>) -> Self {
        Self {
            contract_b_name,
            contract_b_args,
        }
    }

    // The function that checks if the tags are present
    pub fn check_tags(&self) -> bool {
        let tags = env::state_read::<NearAppsTags>();
        tags.person_tag == "person" && tags.company_tag == "company" && tags.purpose_tag == "purpose"
    }

    // Create an array that stores all the names of the approved contracts
    pub fn get_approved_contracts(&self) -> Vec<String> {
        let mut approved_contracts = Vec::new();
        approved_contracts.push(self.contract_b_name.clone());
        // remove a contract from the list
        // approved_contracts.remove(some_contract_name.clone());
        approved_contracts
    }

    //NearApps will wait for a callback indicating that Contract-B has run successfully

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