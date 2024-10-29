use super::de_encode_base64;

pub fn execute(sematic: &String, block: String) -> Vec<u8>{
    let coefficients: Vec<u8> = Vec::new();  
    let byte_vec = de_encode_base64::decode(block).unwrap_or(vec![0, 16]);

    match sematic.as_str() {
        "xex" => xex(byte_vec, coefficients),
        _ => coefficients
    }
}

fn xex(byte_vec: Vec<u8>, mut coefficients: Vec<u8>) -> Vec<u8> {
    for (byte_index, byte) in byte_vec.iter().enumerate() {
        for bit_index in 0..8 {
            let bit = (byte >> bit_index) & 1;
            if bit == 1 {
                coefficients.push(8 * byte_index as u8 + bit_index); 
            }
        }
    }
    coefficients
}