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

    pop_last_zeros(a)
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

    pop_last_zeros(result)
}

pub fn pow(a: &Vec<Vec<u8>>, mut k: u128) -> Vec<Vec<u8>> {
    let mut result = vec![vec![0; 16]; a.len()];
    result[0][0] = 0x80; // Initialise result with 1

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

    pop_last_zeros(result)
}

pub fn divmod(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    
    let mut a = if a.is_empty() { vec![vec![0u8; 16]; 1] } else { a.clone() };
    let mut degree_a = a.len() -1;
    let degree_b = b.len() -1;

    let mut q = vec![vec![0u8; 16]; if degree_a > degree_b { degree_a - degree_b + 1 } else { 1 }];

    while degree_a >= degree_b {
        let degree_div = degree_a - degree_b;
        let factor = gf_operations::gfdiv(a[degree_a].clone(), b[degree_b].clone());
        q[degree_div] = factor.clone();

        // Multiply every coefficient of 'b' with thr 'factor' 
        let mut b_mul_fact = b.clone();
        b_mul_fact = mul(&b_mul_fact, &vec![factor]);

        // Padd 'b' at the beginning with 16 byte vectors with 0 until it has the lenght of 'a'
        for _ in 0..(degree_div) {
            b_mul_fact.insert(0, vec![0u8; 16]);
        }

        a = add(&a, &b_mul_fact); // "Reduce" 'a' polynomial with shifted b multiplied with the factor
        degree_a = a.len() -1; // New calcualtion of degree due to deleted zero-blocks in add
        
        if degree_div == 0 {
            break;
        }
    }
    (pop_last_zeros(q), pop_last_zeros(a))
}

pub fn powmod(a: &Vec<Vec<u8>>, m: &Vec<Vec<u8>>, mut k: u128) -> Vec<Vec<u8>> {
    let mut result = vec![vec![0; 16]; a.len()];
    result[0][0] = 0x80; // Initialise result with 1

    let mut base = a.clone();

    while k > 0 {
        // If k is odd, multiply result by base
        if k % 2 == 1 {
            result = mul(&result, &base);
            (_, result) = divmod(&result, &m);
        }
        // Square the base and use modular reduction
        (_, base) = divmod(&mul(&base, &base), &m);
        // Halve k
        k /= 2;
    }

    // Remove trailing zero vectors from 'result'
    pop_last_zeros(result)
}

fn pop_last_zeros(mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    while let Some(last) = input.last() {
        if input.len() > 1 && last.iter().all(|&x| x == 0) {
            input.pop();
        } else {
            break;
        }
    }
    input
}

pub fn sort(mut input: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    input.sort_by(|a, b| {
        // Compare by the number of elements in the vector (ascending order)
        a.len().cmp(&b.len())
            .then_with(|| {
                // If lengths are equal, compare the vectors lexicographically (descending order)
                a.iter().rev().cmp(b.iter().rev()).reverse()
            })
    });
    input
}