use base64::{engine::general_purpose, Engine as _};

pub fn execute(sematic: String, block: String) -> Vec<u8>{
    let coefficients: Vec<u8> = Vec::new();  

    match general_purpose::STANDARD.decode(block) {
        Ok(block) => {
            match sematic.as_str() {
                "xex" => xex(block, coefficients),
                _ => coefficients
            }
        }
        Err(e) => {
            println!("Failed to decode Base64: {}", e);
            coefficients
        }
    }
}

fn xex(block: Vec<u8>, mut coefficients: Vec<u8>) -> Vec<u8> {
    for (byte_index, byte) in block.iter().enumerate() {
        for bit_index in 0..8 {
            let bit = (byte >> bit_index) & 1;
            if bit == 1 {
                coefficients.push(8 * byte_index as u8 + bit_index); 
            }
        }
    }
    coefficients
}