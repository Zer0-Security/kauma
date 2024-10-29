use base64::{engine::general_purpose, DecodeError, Engine as _};
use super::{block2poly, poly2byte};

pub fn decode(block: String) -> Result<Vec<u8>, DecodeError> {
    match general_purpose::STANDARD.decode(block) {
        Ok(block) => Ok(block),
        Err(e) => Err(e)
    }
}

pub fn encode<T: AsRef<[u8]>>(byte_vect: T) -> String {
   general_purpose::STANDARD.encode(byte_vect)
}

pub fn block_to_u128(semantic: &String, block: String) -> u128 {
    
    let byte_vec = poly2byte::execute(&semantic, block2poly::execute(&semantic, block));
    let mut result: u128 = 0;

    for byte in byte_vec.iter().rev() {
        result = (result << 8) | *byte as u128;
    }
    result
}