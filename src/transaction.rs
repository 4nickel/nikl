use super::*;
use std::collections::{HashMap, HashSet};

const REWARD: u64 = 1;

#[derive(Debug, Clone)]
pub struct Packet {
    pub to: String,
    pub id: u64,
    pub value: u64,
}

impl Digestable for Packet {
    fn digest(&self, hasher: &mut Hasher) {
        hasher.update(self.to.as_bytes());
        hasher.update(self.id.bytes());
        hasher.update(self.value.bytes());
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<Packet>,
    pub outputs: Vec<Packet>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            outputs: Default::default(),
        }
    }
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|p| p.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|p| p.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs.iter().map(|p| p.hash()).collect()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs.iter().map(|p| p.hash()).collect()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Digestable for Transaction {
    fn digest(&self, hasher: &mut Hasher) {
        for packet in self.inputs.iter().chain(self.outputs.iter()) {
            packet.digest(hasher);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transactions(pub Vec<Transaction>);

impl Transactions {
    pub fn input_value(&self) -> u64 {
        self.0.iter().map(|t| t.input_value()).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.0.iter().map(|t| t.output_value()).sum()
    }

    pub fn has_coinbase(&self) -> bool {
        self.0.len() > 0 && self.0[0].is_coinbase()
    }

    pub fn fee(&self) -> u64 {
        self.input_value().saturating_sub(self.output_value())
    }

    pub fn with_coinbase(mut self, id: u64, to: String) -> Self {
        let share = self.fee() + REWARD;
        let transactions = self.0;
        self.0 = vec![Transaction {
            inputs: vec![],
            outputs: vec![Packet {
                to,
                id,
                value: share,
            }],
        }];
        self.0.extend(transactions);
        self
    }
}

#[derive(Debug)]
pub struct Balance(HashMap<String, i64>);

impl Default for Balance {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Balance {

    pub fn combine(self, other: Balance) -> Self {
        let mut balance = Self::default();
        for key in self.0.keys().chain(other.0.keys()) {
            balance.0.insert(
                key.clone(),
                self.0.get(key).map(|total| *total).unwrap_or(0)
                    + other.0.get(key).map(|total| *total).unwrap_or(0),
            );
        }
        balance
    }

    pub fn from_transactions(transactions: &Transactions) -> Self {
        let mut balance = Self::default();
        for transaction in transactions.0.iter() {
            balance = balance.combine(Balance::from_transaction(transaction));
        }
        balance
    }

    pub fn from_transaction(transaction: &Transaction) -> Self {
        let mut balance = Self::default();
        for packet in transaction.outputs.iter() {
            match balance.0.get_mut(&packet.to) {
                Some(total) => {
                    *total += packet.value as i64;
                }
                None => {
                    balance.0.insert(packet.to.clone(), packet.value as i64);
                }
            }
        }
        for packet in transaction.inputs.iter() {
            match balance.0.get_mut(&packet.to) {
                Some(total) => {
                    *total -= packet.value as i64;
                }
                None => {
                    balance.0.insert(packet.to.clone(), -(packet.value as i64));
                }
            }
        }
        balance
    }
}

impl std::fmt::Display for Balance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, total) in self.0.iter() {
            write!(
                formatter,
                "{:>8} ► {}\n",
                name, total
            )?;
        }
        Ok(())
    }
}
