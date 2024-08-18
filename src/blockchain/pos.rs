use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub is_active: bool,
    pub last_proposed_block: u64,
}

#[derive(Debug, Clone)]
pub struct PosMechanism {
    pub validators: HashMap<String, Validator>,
    pub total_stake: u64,
    pub finalty_threshold: u64,
    pub penalty_percentage: u64,
}

impl PosMechanism {
    pub fn new(finalty_threshold: u64, penalty_percentage: u64) -> Self {
        PosMechanism {
            validators: HashMap::new(),
            total_stake: 0,
            finalty_threshold,
            penalty_percentage,
        }
    }

    pub fn add_validator(&mut self, _address: String, stake: u64) {
        self.validators.insert(_address.clone(), Validator {
            address: _address,
            stake,
            is_active: true,
            last_proposed_block: 0,
        });
        self.total_stake += stake;
    }

    pub fn remove_validator(&mut self, _address: String) {
        self.validators.remove(&_address);
        self.total_stake -= self.validators.get(&_address).unwrap().stake;
    }

    pub fn update_validator(&mut self, address: String, new_stake: u64) {
        if let Some(validator) = self.validators.get_mut(&address) {
            self.total_stake -= validator.stake;
            validator.stake = new_stake;
            self.total_stake += new_stake;
        }
    }

    pub fn select_validator(&self, current_block_number: u64) -> Option<String> {
        if self.validators.is_empty() { return None; }

        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0..self.total_stake);
        let mut cumulative_stake = 0;

        for validator in self.validators.values() {
            if !validator.is_active || current_block_number - validator.last_proposed_block < self.finalty_threshold  {
                continue;
            }
            cumulative_stake += validator.stake;
            if random_value < cumulative_stake {
                return Some(validator.address.clone());
            }
        }

        None
    }

    pub fn penalize_validator(&mut self, address: &str) {
        if let Some(validator) = self.validators.get_mut(address) {
            let penalty = validator.stake * self.penalty_percentage / 100;
            validator.stake -= penalty;
            self.total_stake -= penalty;
            validator.is_active = false;
        }
    }

    pub fn reactivate_validator(&mut self, address: &str) {
        if let Some(validator) = self.validators.get_mut(address) {
            validator.is_active = true;
        }
    }

    pub fn update_last_proposed_block(&mut self, address: &str, block_number: u64) {
        if let Some(validator) = self.validators.get_mut(address) {
            validator.last_proposed_block = block_number;
        }
    }

    pub fn is_finalized(&self, block_number: u64) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        block_number + self.finalty_threshold < current_time
    }
}