mod chain;

fn main() {
    println!("Bienvenidos a Rust Chain!");
    replace_chain();
    // mine_blocks();
    // let gen = chain::block::Block::genesis();
    // let mut last_block = chain::block::mine_block(&gen,String::from("data"));

    // loop {
    //     let block2 = chain::block::mine_block(&last_block, String::from("data"));
    //     block2.pretty_print();
    //     last_block = block2;
    // }
}

fn mine_blocks() {
    let mut chain = chain::block_chain::BlockChain::new();
    loop {
        chain.add_block("Some data");
        let block = chain.chain.last().unwrap();
        block.pretty_print();
    }
}

fn replace_chain() {
    let mut chain1 = chain::block_chain::BlockChain::new();
    let mut chain2 = chain::block_chain::BlockChain::new();

    for x in 0..10 {
        chain1.add_block("data");
        chain2.add_block("data");
    }

    // No va a funcionar porque el chain es igual a chain1
    // chain1.replace_chain(chain2.chain.clone());
    
    chain2.add_block("some more data");

    // {
    //     let mut chain3 = chain2.clone();
    //     chain3.chain[1] =
    //         chain::block::mine_block(&chain3.chain.last().unwrap(), String::from("data"));
    //     // No va a funcionar porque un block en el chain es incorrecto!
    //     chain1.replace_chain(chain3.chain);
    // }

    // Si va a funcionar!
    chain1.replace_chain(chain2.chain.clone());
}
