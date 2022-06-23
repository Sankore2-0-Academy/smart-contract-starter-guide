// use near_sdk::{near_bindgen, borsh::{BorshDeserialize, BorshSerialize}, setup_alloc};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc, env, Promise};
use near_sdk::collections::{Vector, LookupMap};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    count: LookupMap<String, u64>,
    names: Vector<String>
}

impl Default for Counter {
    fn default() -> Self {
        Self { 
            count: LookupMap::new(b"c"),
            names: Vector::new(b"n")
         }
    }
}

#[near_bindgen]
impl Counter {
    pub fn save_name(&mut self, name: String) -> String {
        self.names.push(&name);
        name
    }

    pub fn get_names(&self) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        for name in self.names.iter() {
            results.push(name.to_string());
        }
        results
    }

    pub fn remove_name(&mut self) {
        self.names.pop();
    }
    

    /**
     * View function: Reading the state of blockchain
     */
    pub fn read_value(&self) -> u64 {
        let signer = env::signer_account_id();
        match self.count.get(&signer) {
            Some(counter) => counter,
            None => 0
        }
    }

    /**
     * Call function: Changes the state
     */
    pub fn decrement(&mut self) -> u64 {
        let signer = env::signer_account_id();
        if let Some(counter) = self.count.get(&signer) {
            let mut new_counter: u64 = 0;
            if counter > 0 {
                new_counter = counter - 1;
                self.count.insert(&signer, &new_counter);
            }
            new_counter
        } else {
            0
        }
    }

    /**
     * Call function: Changes the state
     */
    #[payable]
    pub fn increment(&mut self) -> u64 {
        let signer = env::signer_account_id();
        let initial_storage = env::storage_usage();
        let attached_deposit = env::attached_deposit();

        if let Some(counter) = self.count.get(&signer) {
            let new_counter = counter + 1;
            self.count.insert(&signer, &new_counter);

            let current_storage = env::storage_usage();
            let used_storage = current_storage - initial_storage;
            let storage_unit_price = env::storage_byte_cost();

            if let Some(payable_storage_cost) = u128::checked_mul(storage_unit_price, used_storage.into()) {
                assert!(attached_deposit >= payable_storage_cost);

                let surplus = attached_deposit - payable_storage_cost;
                let excess = if surplus > 0 { surplus } else { 0 };

                if excess > 0 {
                    let promise = Promise::new(signer);
                    promise.transfer(excess);
                }
            }

            new_counter
        } else {
            let new_counter = 1;
            self.count.insert(&signer, &new_counter);

            let current_storage = env::storage_usage();
            let used_storage = current_storage - initial_storage;
            let storage_unit_price = env::storage_byte_cost();

            if let Some(payable_storage_cost) = u128::checked_mul(storage_unit_price, used_storage.into()) {
                assert!(attached_deposit >= payable_storage_cost);

                let surplus = attached_deposit - payable_storage_cost;
                let excess = if surplus > 0 { surplus } else { 0 };

                if excess > 0 {
                    let promise = Promise::new(signer);
                    promise.transfer(excess);
                }
                
            }

            new_counter
        }
    }

    /**
     * Call function
     */
    pub fn wallet_address(&self) -> String {
        env::signer_account_id()
    }

    #[payable]
    pub fn attached_near(&mut self) -> u128 {
        env::attached_deposit()
    }

    pub fn gas(&self) -> (u64, u64) {
        let attached_gas = env::prepaid_gas();
        let gas_used = env::used_gas();
        (attached_gas, gas_used)
    }
}