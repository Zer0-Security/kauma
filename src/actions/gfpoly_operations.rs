use super::gfmul;

pub fn add(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let mut a = a.clone();
    let mut b = b.clone();

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
            let mul = gfmul::execute(&"gcm".to_string(), coefficient_a.clone(), coefficient_b.clone());
            for k in 0..=15 {
                result[i+j][k] ^=  mul[k];
            }
        }
    }
    result
}

pub fn pow(a: &Vec<Vec<u8>>, k: u8) -> Vec<Vec<u8>> {

    let mut result = a.clone();

    if k == 0 {
        let mut result: Vec<u8> = vec![0; 16];
        result[0] = 0x40;
        return vec![result]
    } else if k == 1 {
        return result
    }

    for __ in 1..k {
        result = mul(&result, &a.clone());
    }
    result
}