// use near_sdk::{near_bindgen, borsh::{BorshDeserialize, BorshSerialize}, setup_alloc};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc, env};

setup_alloc!();

#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u32
}

impl Default for Counter {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[near_bindgen]
impl Counter {
    /**
     * View function: Reading the state of blockchain
     */
    pub fn read_value(&self) -> u32 {
        self.value
    }

    /**
     * Call function: Changes the state
     */
    pub fn decrement(&mut self) -> u32 {
        if self.value > 0 {
            self.value -= 1;
        }
        self.value
    }

    /**
     * Call function: Changes the state
     */
    pub fn increment(&mut self) -> u32 {
        self.value += 1;
        self.value
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