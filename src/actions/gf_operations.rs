use super::de_encode_base64;

pub fn gfmul(semantic: &String, a: Vec<u8>, b: Vec<u8>) -> Vec<u8>{
    
    match semantic.as_str() {
        "xex" => xex(a, b),
        "gcm" => gcm(a, b),
        _ => vec![0; 16]
    }

}

fn xex(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {

    let a = de_encode_base64::byte_to_u128(&"xex".to_string(), a);
    let b = de_encode_base64::byte_to_u128(&"xex".to_string(), b);
    let result = gfmul_f128(a, b);

    de_encode_base64::u128_to_byte(&"xex".to_string(), result)
}

fn gcm(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    
    let a = de_encode_base64::byte_to_u128(&"gcm".to_string(), a);
    let b = de_encode_base64::byte_to_u128(&"gcm".to_string(), b);
    let result = gfmul_f128(a, b);

    de_encode_base64::u128_to_byte(&"gcm".to_string(), result)
}
    
fn gfmul_f128(mut a: u128, mut b: u128) -> u128 {
    let reduction_poly: u128 = 0x87;
    let mut result = 0;

    while b != 0 {
        // If the lowest bit of 'b' is set, XOR the result with 'a'
        if (b & 1) != 0 {
            result ^= a;
        }

        let carry = a & (1 << 127); // Check if the high bit of 'a' is set (i.e., overflow beyond 128 bits at shift)

        a <<= 1; // Shift 'a' to the left (multiply by alpha)

        // If 'a' had overflow, XOR with the reduction polynomial
        if carry != 0 {
            a ^= reduction_poly;
        }

        b >>= 1; // Shift 'b' to the right to process the next bit
    }
    result
}

pub fn gfdiv(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {

    let b_inverse = inverse_element_f(b, 128);
    let a = de_encode_base64::byte_to_u128(&"gcm".to_string(), a);
    let b_inverse = de_encode_base64::byte_to_u128(&"gcm".to_string(), b_inverse);
    
    let result = gfmul_f128(a, b_inverse);
    de_encode_base64::u128_to_byte(&"gcm".to_string(), result)
}

fn inverse_element_f(e: Vec<u8>, f: u32) -> Vec<u8> {

    let e = de_encode_base64::byte_to_u128(&"gcm".to_string(), e);
    let mut result: u128 = 1;
    let mut base = e.clone();

    let exponent: u128 = 2;
    let mut exponent: u128 = exponent.pow(f) - 2; // 2^f - 2

    while exponent > 0 {
        // If k is odd, multiply result by base
        if exponent % 2 == 1 {
            result = gfmul_f128(result, base);
        }
        
        base = gfmul_f128(base, base); // Square the base
        
        exponent /= 2; // Halve k
    }
    de_encode_base64::u128_to_byte(&"gcm".to_string(), result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_by_zero() {
        let result = gfmul_f128(0x3456789, 0x0);
        assert_eq!(result, 0);
    }

    #[test]
    fn two_numbers() {
        let result = gfmul_f128(0x01120000000000000000000000000080, 0x02000000000000000000000000000000);
        assert_eq!(result, 0x11cfc00000000000000000000000087);
    }

    #[test]
    fn multiply_by_one() {
        let result = gfmul_f128(0x657890543286, 0x1);
        assert_eq!(result, 0x657890543286);
    }
}
