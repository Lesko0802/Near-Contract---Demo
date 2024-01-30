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

//Defining trait for contractB
#[ext_contract(ext_contract_b)]
trait ContractB {
    fn method_on_b(&self) -> String;
    fn another_method_on_b(&self, some_arg: u64) -> U128;
    fn mutable_method_on_b(&mut self, some_arg: String);
}

// Callback
#[ext_contract(ext_self)]
trait MyContract {
    fn my_callback(&self) -> String;
}

impl NearAppsContract {
    // Create an array that stores all the names of the approved contracts
    pub fn get_approved_contracts(&self) -> Vec<String> {
        let mut approved_contracts = Vec::new();
        approved_contracts.push(self.contract_b_name.clone());
        approved_contracts
    }

    pub fn new() -> Self {
        Self::default()
    }

    //NearApps will wait for a callback indicating that Contract-B has run successfully
    pub fn my_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );
      
        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
          PromiseResult::NotReady => unreachable!(),
          PromiseResult::Failed => "oops!".to_string(),
          PromiseResult::Successful(result) => {
              let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
              if balance.0 > 100000 {
                  "Wow!".to_string()
              } else {
                  "Hmmmm".to_string()
              }
          },
        }
      }
      
      pub fn my_method(&self) -> Promise {
          ext_ft::ft_balance_of(
              "some_account_id.near".to_string(), // ft_balance_of takes an account_id as a parameter
              &"wrap.near", // contract account id
              0, // yocto NEAR to attach
              5_000_000_000_000 // gas to attach
          )
          .then(ext_self::my_callback(
              &env::current_account_id(), // this contract's account id
              0, // yocto NEAR to attach to the callback
              5_000_000_000_000 // gas to attach to the callback
          ))
      }
}    
