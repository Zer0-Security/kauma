pub fn execute(a: u128, b: u128) -> u128{
    
    let mut result = 0;
    let mut a = a;
    let mut b = b;
    let reduction_poly: u128 = 0x87;

    while b != 0 {
        // If the lowest bit of 'b' is set, XOR the result with 'a'
        if (b & 1) != 0 {
            result ^= a;
        }

        let carry = a & (1 << 127); // Check if the high bit of 'a' is set (i.e., overflow beyond 128 bits)

        a <<= 1; // Shift 'a' to the left (multiply by alpha)

        // If 'a' had overflow, XOR with the reduction polynomial
        if carry != 0 {
            a ^= reduction_poly;
        }

        b >>= 1; // Shift 'b' to the right to process the next bit
    }
    result
}
