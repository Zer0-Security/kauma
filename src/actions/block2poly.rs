use super::de_encode_base64;

pub fn execute(sematic: &String, block: String) -> Vec<u8>{
    let coefficients: Vec<u8> = Vec::new();  
    let byte_vec = de_encode_base64::decode(block).unwrap_or(vec![0x0; 16]);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xex_normal() {
        let result = execute(&"xex".to_string(), "ARIAAAAAAAAAAAAAAAAAgA==".to_string());
        assert_eq!(result, vec![0, 9, 12, 127]);
    }

    #[test]
    fn xex_empty_input() {
        let result = execute(&"xex".to_string(), "".to_string());
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    #[should_panic]
    fn xex_check_sorted_output() {
        let result = execute(&"xex".to_string(), "ARIAAAAAAAAAAAAAAAAAgA==".to_string());
        assert_eq!(result, vec![9, 0, 127, 12]);
    }
}