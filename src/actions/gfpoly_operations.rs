use super::gf_operations;

pub fn add(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let mut a = a.clone();
    let mut b = b.clone();

    // Match the lenght of the longer number and pad the shorter one
    if a.len() < b.len() {
        a.extend(vec![vec![0; 16]; b.len() - a.len()]);
    } else if b.len() < a.len() {
        b.extend(vec![vec![0; 16]; a.len() - b.len()]);
    }
    
    for i in 0..a.len() {
        for j in 0..=15 {
            a[i][j] ^= b[i][j];
        }
    }
    a
}

pub fn mul(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let max_length = a.len() + b.len() - 1;
    let mut result: Vec<Vec<u8>> = vec![vec![0; 16]; max_length];
    
    for (i, coefficient_a) in a.iter().enumerate() {
        for (j, coefficient_b) in b.iter().enumerate() {
            let mul = gf_operations::gfmul(&"gcm".to_string(), coefficient_a.clone(), coefficient_b.clone());
            for k in 0..=15 {
                result[i+j][k] ^=  mul[k];
            }
        }
    }
    result
}

pub fn pow(a: &Vec<Vec<u8>>, mut k: u8) -> Vec<Vec<u8>> {
    // Identity element for multiplication (based on your original code)
    let mut result = vec![vec![0; 16]; a.len()];
    result[0][0] = 0x80;

    // Base starts as `a`
    let mut base = a.clone();

    while k > 0 {
        // If k is odd, multiply result by base
        if k % 2 == 1 {
            result = mul(&result, &base);
        }
        // Square the base
        base = mul(&base, &base);
        // Halve k
        k /= 2;
    }

    // Remove trailing zero vectors from `result`
    while let Some(last) = result.last() {
        if last.iter().all(|&x| x == 0) {
            result.pop();
        } else {
            break;
        }
    }

    result
}