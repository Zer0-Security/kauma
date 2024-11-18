use std::mem;

use super::gf_operations::{self, gfmul};

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
        // Step 1: Compare by the number of elements in the outer vector (ascending)
        a.len().cmp(&b.len())
            .then_with(|| {
                // Step 2: Compare lexicographically by the inner vectors (descending order)
                let mut a_iter = a.iter().rev();
                let mut b_iter = b.iter().rev();

                loop {
                    match (a_iter.next(), b_iter.next()) {
                        (Some(a_inner), Some(b_inner)) => {
                            // Compare individual `u8` values
                            match a_inner.iter().rev().map(|&x| x.reverse_bits())
                                .cmp(b_inner.iter().rev().map(|&x| x.reverse_bits())) 
                                {
                                std::cmp::Ordering::Equal => continue, // If equal -> check the next vectors
                                ord => return ord, // Larger numbers take precedence
                            }
                        }
                        (None, None) => return std::cmp::Ordering::Equal, // Both have same inner vectors
                        (None, _) => return std::cmp::Ordering::Less,    // `a` has fewer inner vectors
                        (_, None) => return std::cmp::Ordering::Greater, // `b` has fewer inner vectors
                    }
                }
            })
    });
    input
}

pub fn make_monic(a: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let divisor = a.last().map(|v| vec![v.clone()]).unwrap_or(vec![vec![0u8; 16]]);
    let result = divmod(&a, &divisor);
    result.0
}

pub fn sqrt(a: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let semantic = "gcm".to_string();
    let mut result_vect: Vec<Vec<u8>> = Vec::new();

    for (i, poly) in a.iter().enumerate() {
        if i%2 == 0 {
            let mut k: u128 = 2; 
            k = k.pow(127);
            let mut poly = poly.clone();
            let mut result: Vec<u8> = vec![0x80];
            while k > 0 {
                // If k is odd, multiply result by base
                if k % 2 == 1 {
                    result = gfmul(&semantic, result, poly.clone());
                }
                // Square the base
                poly = gfmul(&semantic, poly.clone(), poly.clone());
                // Halve k
                k /= 2;
            } 
            result_vect.push(result);
        }
    }
    result_vect
}

pub fn diff(mut a: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if a.len() == 1 {
        return vec![vec![0u8; 16]];
    }
    a.drain(0..1);

    for i in (1..a.len()).step_by(2) {
        a[i] = vec![0; 16];
    }
    pop_last_zeros(a)
}

pub fn gcd(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut a = a.clone();
    let mut b = b.clone();

    // Swap a and b if b is larger than a
    if b.len() > a.len(){
        mem::swap(&mut a, &mut b);
    }
    let break_condition = vec![vec![0u8; 16]];
    while b != break_condition  {
        let remainder = divmod(&a, &b);
        a = b;
        b = remainder.1;
    }
    make_monic(&a)
}

pub fn sff(f: &Vec<Vec<u8>>) -> Vec<(Vec<Vec<u8>>, u128)> {
    let mut f = f.clone();
    let mut factor_found: Vec<(Vec<Vec<u8>>, u128)> = Vec::new();
    let mut c = gcd(&f, &diff(f.clone()));
    (f, _) = divmod(&f, &c);

    let mut one_vect = vec![vec![0u8; 16]];
    one_vect[0][0] = 0x80; // The constant polynomial 1

    let mut e: u128 = 1;
    while f != one_vect {
        let y = gcd(&f, &c);
        let (factor, _) = divmod(&f, &y);
        factor_found.push((factor, e));

        f = y.clone();
        (c, _) = divmod(&c, &y);
        e += 1;
    }
    
    if c != one_vect {
        for (factor, e) in sff(&sqrt(&f)) {
            factor_found.push((factor, e * 2));
        }
    }

    factor_found
}

pub fn ddf(f: &Vec<Vec<u8>>) -> Vec<(Vec<Vec<u8>>, u128)> {
    let mut z: Vec<(Vec<Vec<u8>>, u128)> = Vec::new();
    let mut d:u32 = 1;

    let mut x = vec![vec![0u8; 16]; 2];
    x[1][0] = 0x80; // x is the polynomial x

    let mut one_vect = vec![vec![0u8; 16]];
    one_vect[0][0] = 0x80; // The constant polynomial 1

    let mut fstar = f.clone();

    while fstar.len() as u32 -1 >= 2 * d {
        let mut h = x.clone();
        // Compute h = x^{2^{128d}} mod fstar
        for _ in 0..(128 * d) {
            h = mul(&h, &h);
            (_, h) = divmod(&h, &fstar);
        }

        h = add(&h, &x);

        let g = gcd(&h, &fstar);

        if g != one_vect {
            z.push((g.clone(), d as u128));
            (fstar, _) = divmod(&fstar, &g);
        }

        d += 1;
    }

    if fstar != one_vect {
        z.push((fstar.clone(), fstar.len() as u128 -1 ));
    } else if z.len() == 0 {
        z.push((f.clone(), 1 ));
    }
    z
}