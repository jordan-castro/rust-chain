use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time;

/// Block para BlockChain
#[derive(PartialEq, Clone)]
pub struct Block {
    timestamp: u128,
    last_hash: String,
    hash: String,
    data: String,
    difficulty: u32,
    nonce: u32,
    pub number: u128,
}

impl Block {
    /// Una forma mas facil para saber la data de un Block.
    pub fn pretty_print(&self) {
        println!(
            "Info de Block #{}: \ntimestamp: {}\nlast_hash: {}\nhash: {}\ndata: {}\ndifficulty: {}\nnonce: {}",
            self.number, self.timestamp, self.last_hash, self.hash, self.data, self.difficulty, self.nonce
        );
    }

    // Todo fn to_json(&self) -> Map {}

    /// Crea el Block de GENISIS
    ///
    /// **Returns** `Block`
    pub fn genesis() -> Block {
        return Block {
            timestamp: 0,
            last_hash: String::from("GENSIS_LAST_HASH"),
            hash: String::from("GENSIS_HASH"),
            data: String::from("GENESIS"),
            difficulty: 1,
            nonce: 1,
            number: 0,
        };
    }
}

/// Convertamos un Hex a un Binary.
///
/// **Params**
/// - `hexidecimal: String` El hex.
///
/// **Returns** `String`
fn hex_to_binary(hexidecimal: &String) -> String {
    // Tabla de hex => binary
    let mut conversion_table = HashMap::new();
    conversion_table.insert('0', "0000");
    conversion_table.insert('1', "0001");
    conversion_table.insert('2', "0010");
    conversion_table.insert('3', "0011");
    conversion_table.insert('4', "0100");
    conversion_table.insert('5', "0101");
    conversion_table.insert('6', "0110");
    conversion_table.insert('7', "0111");
    conversion_table.insert('8', "1000");
    conversion_table.insert('9', "1001");
    conversion_table.insert('a', "1010");
    conversion_table.insert('b', "1011");
    conversion_table.insert('c', "1100");
    conversion_table.insert('d', "1101");
    conversion_table.insert('e', "1110");
    conversion_table.insert('f', "1111");

    let mut binary_string = String::new();

    // Loop por los ch en `hexidecimal`
    for c in hexidecimal.chars() {
        let b_value = conversion_table.get(&c).expect("Gotcha");
        binary_string.push_str(b_value);
    }
    binary_string
}

/// Creamos un hash para el block. Regresa en forma SHA-256
///
/// **Params**
/// - `timestamp: u128` El tiempo ahora.
/// - `last_hash: String` El hash ultimo.
/// - `data: String` La data.
/// - `difficulty: u32` La difficultad.
/// - `nonce: u32` El nonce.
///
/// **Returns** `String`
fn crypto_hash(
    timestamp: &u128,
    last_hash: &String,
    data: &String,
    difficulty: &u32,
    nonce: &u32,
) -> String {
    // Ponemos todo la data en un String
    let data_for_hash =
        timestamp.to_string() + last_hash + data + &difficulty.to_string() + &nonce.to_string();
    // El objecto para ser hashing
    let mut hasher = Sha256::new();
    // Hacemos el sha
    hasher.update(data_for_hash.as_bytes());
    // La resulta
    let result = hasher.finalize();
    // El hash final
    format!("{:x}", result)
}

/// Suba o baja la difficultad del miner.
///
/// **Params**
/// - `last_block: Block` El ultimo block.
/// - `new_timestamp: u128` El timestamp currentamente.
///
/// **Returns** `u32`
fn adjust_difficulty(last_block: &Block, new_timestamp: u128) -> u32 {
    // El mine_rate es 4 segundos
    let mine_rate = time::Duration::new(4, 0).as_nanos();

    // La differencia de los dos tiempos
    let time_diff = new_timestamp - last_block.timestamp;

    // Si la differencia es menos del mine rate, queremos subir la difficultad
    if time_diff < mine_rate {
        return last_block.difficulty + 1;
    }

    // Si la difficultad menos 1 es mas de uno entonces bajamos.
    if last_block.difficulty - 1 > 1 {
        last_block.difficulty - 1
    } else {
        // Si no entonces usamos 1 por default.
        1
    }
}

/// Hacemos un Mine para obtener un nuevo block.
///
/// **Params**
/// - `last_block: Block` El ultimo block en el chain.
/// - `data: String` La data a poner en el chain.
///
/// **Returns** `Block`
pub fn mine_block(last_block: &Block, data: String) -> Block {
    let last_hash = &last_block.hash;
    let mut nonce = 0;

    loop {
        // El timestamp para AHORA!
        let time = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos(),
            Err(_) => 0,
        };
        let difficulty = adjust_difficulty(last_block, time);
        let block_hash = crypto_hash(&time, &last_hash, &data, &difficulty, &nonce);
        let binary = hex_to_binary(&block_hash);
        let splice_range = usize::try_from(difficulty).unwrap();

        let binary_string = {
            // Ponemos en el string mas y mas
            let mut bin = String::new();
            for x in 0..difficulty {
                bin.push('0');
            }
            bin
        };

        // Chequea proof de trabajo
        if &binary[0..splice_range] == binary_string {
            return Block {
                timestamp: time,
                last_hash: last_hash.to_string(),
                hash: block_hash.to_string(),
                data: data,
                difficulty: difficulty,
                nonce: nonce,
                number: last_block.number + 1,
            };
        }
        // Ponemos uno a nonce cada vez
        nonce += 1;
    }
}

/// Verifica un block.
///
/// **Params**
/// - `last_block: &Block` El block ultimo.
/// - `block: &Block` El block para verificar.
///
/// **Returns** `bool`
pub fn is_valid_block(last_block: &Block, block: &Block) -> bool {
    // Chequea que el last_hash del block es igual al hash de last_block
    if last_block.hash != block.last_hash {
        return false;
    }

    // Chequea la difficultad.
    // Crea el binary
    let binary_string = {
        let mut bin = String::new();
        for x in 0..block.difficulty {
            bin.push('0');
        }
        bin
    };
    let binary_hash = hex_to_binary(&block.hash);

    // Chequea que el binary hash tiene los correcto numeros de 0's
    if binary_hash[0..usize::try_from(block.difficulty).unwrap()] != binary_string {
        return false;
    }

    // Chequea que la difficultad solo cambia por uno al maximo
    // i32 porque puede ser negativo.
    if i32::abs(
        i32::try_from(last_block.difficulty).unwrap() - i32::try_from(block.difficulty).unwrap(),
    ) > 1
    {
        return false;
    }

    // Crea el hash denuevo y chequea que es igual a el block.hash
    let recon_hash = crypto_hash(
        &block.timestamp,
        &block.last_hash,
        &block.data,
        &block.difficulty,
        &block.nonce,
    );
    if recon_hash != block.hash {
        return false;
    }

    // Chequea el numero es uno mas del ultimo
    if block.number != last_block.number + 1 {
        return false;
    }

    // Es valido!
    true
}
