use std::{thread, time::Duration};

use hashchain_rs::{Block, HashChain};

fn main() {
    // let bytes = todo!();
    // let mut chain = HashChain::from_bytes(bytes);

    let mut chain = HashChain::new();
    println!("{:?}", chain.to_bytes());

    for i in 1..=5 {
        thread::sleep(Duration::from_secs(1));

        let prev_hash = chain.latest_block().unwrap().hash();
        let content = format!("{}-block", i);
        let new_block = Block::new(prev_hash, content);

        chain.add_block(new_block);
    }

    println!("{:#?}", chain);
    println!("{:?}", chain.to_bytes());
}
