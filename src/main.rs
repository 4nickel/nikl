use chainlib::block::Block;
use chainlib::chain::{ValidationError, Blockchain};
use chainlib::transaction::{Transactions, Transaction, Packet};
use chainlib::Timestamp;

const DIFFICULTY: u128 = 0x00ffffffffffffffffffffffffffffff;

fn main() -> Result<(), ValidationError> {

    let transactions = vec![
        Transactions(vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Packet {
                        to: "Alice".into(),
                        id: 0,
                        value: 4,
                    },
                    Packet {
                        to: "Bob".into(),
                        id: 1,
                        value: 4,
                    }
                ],
            }
        ]),
        Transactions(vec![
            Transaction {
                inputs: vec![
                    Packet {
                        to: "Alice".into(),
                        id: 0,
                        value: 4,
                    },
                ],
                outputs: vec![
                    Packet {
                        to: "Alice".into(),
                        id: 4,
                        value: 2,
                    },
                    Packet {
                        to: "Bob".into(),
                        id: 5,
                        value: 1,
                    },
                ],
            }
        ]),
        Transactions(vec![
            Transaction {
                inputs: vec![
                    Packet {
                        to: "Bob".into(),
                        id: 1,
                        value: 4,
                    }
                ],
                outputs: vec![
                    Packet {
                        to: "Bob".into(),
                        id: 6,
                        value: 2,
                    },
                    Packet {
                        to: "Dave".into(),
                        id: 7,
                        value: 1,
                    },
                    Packet {
                        to: "Eve".into(),
                        id: 8,
                        value: 1,
                    },
                ],
            }
        ]),
    ];

    let mut chain = Blockchain::default();
    let mut genesis = Block::new(chain.next_block_index(), Timestamp::now(), DIFFICULTY, chain.last_block_hash(), transactions[0].clone());
    genesis.mine();
    chain.update_with_block(genesis)?;

    for i in 1..transactions.len() {
        let transaction = transactions[i].clone().with_coinbase((1337+i) as u64, "Chris".into());
        let mut block = Block::new(chain.next_block_index(), Timestamp::now(), DIFFICULTY, chain.last_block_hash(), transaction);
        block.mine();
        chain.update_with_block(block)?;
        println!("{}", chain.balance());
    }

    for b in chain.blocks.iter() {
        println!("{}", b);
    }

    Ok(())
}
