use base64::{engine::general_purpose, DecodeError, Engine as _};
//use super::{block2poly, poly2byte};

pub fn decode(block: String) -> Result<Vec<u8>, DecodeError> {
    match general_purpose::STANDARD.decode(block) {
        Ok(block) => Ok(block),
        Err(e) => Err(e)
    }
}

pub fn encode<T: AsRef<[u8]>>(byte_vect: T) -> String {
   general_purpose::STANDARD.encode(byte_vect)
}

// pub fn block_to_u128(semantic: &String, block: String) -> u128 {
    
//     let byte_vec = poly2byte::execute(&semantic, block2poly::execute(&semantic, block));
    
//     byte_to_u128(semantic, byte_vec)
// }

pub fn byte_to_u128(semantic: &String, byte_vec: Vec<u8>) -> u128 {
    let mut result: u128 = 0;

    match semantic.as_str() {
        "xex" => {
            for byte in byte_vec.iter().rev() {
                result = (result << 8) | *byte as u128;
            }
            result
        }
        "gcm" => {
            for byte in byte_vec.iter().rev() {
                result = (result << 8) | byte.reverse_bits() as u128;
            }
            result
        }
        _ => 0
    }

}

pub fn u128_to_byte(semantic: &String, num: u128) -> Vec<u8> {

    match semantic.as_str() {
        "xex" => {
            num.to_le_bytes().to_vec()
        }
        "gcm" => {
            let mut byte_vec = num.to_le_bytes().to_vec();

            for byte in byte_vec.iter_mut().rev() {
                *byte = byte.reverse_bits();
            }
            byte_vec
        }
        _ => vec![0; 16]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_normal() {
        let result = decode("ARIAAAAAAAAAAAAAAAAAgA==".to_string()).unwrap();

        let expected: Vec<u8> = vec![0x01, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];

        assert_eq!(result, expected);
    }

    #[test]
    fn encode_normal() {

        let input: Vec<u8> = vec![0x01, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        let result = encode(input);

        let expected= "ARIAAAAAAAAAAAAAAAAAgA==".to_string();

        assert_eq!(result, expected);
    }

    // #[test]
    // fn xex_block_to_u128() {

    //     let result = block_to_u128(&"xex".to_string(), "ARIAAAAAAAAAAAAAAAAAgA==".to_string());

    //     assert_eq!(result, 0x80000000000000000000000000001201);
    // }

    #[test]
    fn xex_byte_to_u128() {

        let input: Vec<u8> = vec![0x01, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        let result = byte_to_u128(&"xex".to_string(), input);

        assert_eq!(result, 0x80000000000000000000000000001201);
    }
}