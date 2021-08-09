use super::{Hash, Hasher, Hashable, Digestable, Timestamp};
use super::transaction::Transactions;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub timestamp: Timestamp,
    pub difficulty: u128,
    pub prev_block_hash: Hash,
    pub hash: Hash,
    pub transactions: Transactions,
}

impl std::fmt::Display for Block {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Block[{:>08x}] ► {} ► {:>016} ► \"{}\"",
            self.index, hex::encode(&self.hash.0[..6]), self.timestamp, self.transactions.0.len()
        )
    }
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: Timestamp,
        difficulty: u128,
        prev_block_hash: Hash,
        transactions: Transactions,
    ) -> Self {
        Self {
            index,
            nonce: 0,
            timestamp,
            difficulty,
            prev_block_hash,
            transactions,
            hash: Default::default(),
        }
    }

    pub fn check_difficulty(&self, hash: Hash) -> bool {
        hash.difficulty() < self.difficulty
    }

    pub fn mine_range(&mut self, lo: u64, hi: u64) -> bool {
        for nonce in lo..hi {
            self.nonce = nonce;
            let hash = self.hash();
            if self.check_difficulty(hash) {
                self.hash = hash;
                return true;
            }
        }
        false
    }

    pub fn mine(&mut self) -> bool {
        self.mine_range(0, u64::max_value())
    }
}

impl Digestable for Block {
    fn digest(&self, hasher: &mut Hasher) {
        use super::*;
        hasher.update(&self.index.bytes());
        hasher.update(&self.nonce.bytes());
        hasher.update(&self.timestamp.bytes());
        hasher.update(&self.difficulty.bytes());
        hasher.update(&self.prev_block_hash);
        self.transactions.0.as_slice().digest(hasher);
    }
}
