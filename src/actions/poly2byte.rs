pub fn execute(sematic: &String, mut coefficients: Vec<u8>) -> Vec<u8>{
    let byte_vec: Vec<u8> = vec![0x0; 16];

    coefficients.sort();

    match sematic.as_str() {
        "xex" => xex(coefficients,byte_vec),
        _ => byte_vec
    }
}

fn xex(coefficients: Vec<u8>, mut byte_vec: Vec<u8>) -> Vec<u8> {
    for coefficient in coefficients {
        let byte: u8 = coefficient / 8;
        let bit = coefficient % 8;

        byte_vec[byte as usize] ^=  1 << bit; 
    }
    byte_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xex_empty_coefficients() {
        let result = execute(&"xex".to_string(), vec![]);
        assert_eq!(result, vec![0;16]);
    }

    #[test]
    fn xex_four_exponents() {
        let result = execute( &"xex".to_string(), vec![127, 0, 12, 9]);
        assert_eq!(result, vec![1, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]);
    }
}