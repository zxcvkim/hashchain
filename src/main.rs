use hashchain_rs::{Block, HashChain};

fn main() {
    // let bytes = todo!();
    // let mut chain = HashChain::from_bytes(bytes);

    let mut chain = HashChain::new();

    let number_of_blocks = 100;

    for i in 1..=number_of_blocks {
        let prev_hash = chain.latest_block().unwrap().hash();
        let content = format!("Block #{}", i);
        let new_block = Block::new(prev_hash, content.clone());

        chain.add_block(new_block);
        println!("{} created!", content);
    }
}
