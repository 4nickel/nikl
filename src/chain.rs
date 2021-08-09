use super::block::Block;
use super::{Hash, Hashable};
use super::transaction::Balance;
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub unspent_outputs: HashSet<Hash>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self {
            blocks: Default::default(),
            unspent_outputs: Default::default()
        }
    }
}

#[derive(Debug)]
pub enum ValidationError {
    IndexMismatch,
    HashMismatch,
    DifficultyMismatch,
    HashChainViolation,
    TimestampViolation,
    InvalidInput,
    InvalidCoinbaseTransaction,
    InsufficientInputValue,
}

impl Blockchain {

    pub fn next_block_index(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn last_block_hash(&self) -> Hash {
        self.blocks.last().map(|b| b.hash).unwrap_or(Default::default())
    }

    pub fn validate_block(&self, block: &Block) -> Result<(), ValidationError> {
        let index = self.blocks.len();
        if block.index as usize != index {
            return Err(ValidationError::IndexMismatch)
        }
        if !block.check_difficulty(block.hash) {
            return Err(ValidationError::DifficultyMismatch)
        }
        if block.hash != block.hash() {
            return Err(ValidationError::HashMismatch)
        }
        if index == 0 {
            if block.prev_block_hash != Hash::default() {
                return Err(ValidationError::HashChainViolation)
            }
        } else {
            if block.prev_block_hash != self.blocks[index-1].hash {
                return Err(ValidationError::HashChainViolation)
            }
            if block.timestamp < self.blocks[index-1].timestamp {
                return Err(ValidationError::TimestampViolation)
            }
        }
        Ok(())
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), ValidationError> {

        self.validate_block(&block)?;

        if let Some((coinbase, transactions)) = block.transactions.0.split_first() {

            if !coinbase.is_coinbase() {
                return Err(ValidationError::InvalidCoinbaseTransaction)
            }

            let mut block_spent = HashSet::<Hash>::new();
            let mut block_created = HashSet::<Hash>::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty() {
                    return Err(ValidationError::InvalidInput)
                }
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value > input_value {
                    return Err(ValidationError::InsufficientInputValue)
                }
                let fee = input_value - output_value;
                total_fee += fee;
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if block.index != 0 && coinbase.output_value() != total_fee + 1 {
                return Err(ValidationError::InvalidCoinbaseTransaction)
            }

            block_created.extend(coinbase.output_hashes());

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);
        Ok(())
    }

    pub fn balance(&self) -> Balance {
        let mut balance = Balance::default();
        for block in self.blocks.iter() {
            balance = balance.combine(Balance::from_transactions(&block.transactions));
        }
        balance
    }
}
