use super::block::{Block, is_valid_block, mine_block};
use std::convert::TryFrom;

/// El Blockchain!
#[derive(Clone)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    /// Crecemos el chain con nuevo block.
    /// 
    /// **Params**
    /// - `data: String` La data del block.
    pub fn add_block(&mut self, data: &str) {
        let block = mine_block(&self.chain.last().unwrap(), String::from(data));
        self.chain.push(block);
    }

    /// Crea un nuevo blockchain. Impieza con GENESIS
    /// 
    /// **Returns** `BlockChain`
    pub fn new() -> BlockChain {
        BlockChain {
            chain: vec![
                Block::genesis()
            ]
        }
    }

    /// Hacemos un Print bonito para el blockchain.
    pub fn pretty_print(&self) {
        println!("Block Chain: \n");
        println!("Numero de blocks: {}", self.chain.len());
        for block in self.chain.iter() {
            block.pretty_print();
            println!();
        }
    }

    /// Llama para cambiar el chain. Tira error si no es valido.
    /// 
    /// **Params**
    /// - `chain: Vec<Block>` El chain para que queramos cambiar,
    pub fn replace_chain(&mut self, chain: Vec<Block>) {
        // Chequea que es mas grande del local
        if chain.len() <= self.chain.len() {
            panic!("El chain que viene tiene que ser mas grande.");
        } 

        // Chequea que el chain es valido.
        let valid_chain = validate_chain(chain.clone());

        if !valid_chain {
            panic!("El chain no es valido.");
        }

        // El chain se puede cambiar.
        self.chain = chain;
    }
}

/// Verifica un Chain.
/// 
/// **Params**
/// - `chain: Vec<Block>` El chain para verificar.
/// 
/// **Returns** `bool`
fn validate_chain(chain: Vec<Block>) -> bool {
    // Chequea que el genesis es correcto
    if chain[0] != Block::genesis() {
        return false;
    }

    // Ahora verifica cada block en el chain
    for block in chain.iter() {
        // El index del last block es el numero del block menos 1. 
        let index = i32::try_from(block.number).unwrap() - 1;
        // Si el index es -1, significando que es GENESIS.
        if index == -1 {
            continue;
        }
        let last_block = &chain[usize::try_from(index).unwrap()];

        // Verifica
        let valid_block = is_valid_block(last_block, block);
        if !valid_block {
            return false;
        }
    }

    true
}