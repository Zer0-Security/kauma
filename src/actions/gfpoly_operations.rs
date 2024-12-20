use std::mem;
use num::BigUint;
use num::{One, Zero};
use rand::Rng;

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

fn comparator(a_vec: &Vec<Vec<u8>>, b_vec: &Vec<Vec<u8>>) -> std::cmp::Ordering {
    a_vec.len().cmp(&b_vec.len())
        .then_with(|| {
            let mut a_iter = a_vec.iter().rev();
            let mut b_iter = b_vec.iter().rev();

            loop {
                match (a_iter.next(), b_iter.next()) {
                    (Some(a_inner), Some(b_inner)) => {
                        match a_inner.iter().rev().map(|&x| x.reverse_bits())
                            .cmp(b_inner.iter().rev().map(|&x| x.reverse_bits()))
                        {
                            std::cmp::Ordering::Equal => continue,
                            ord => return ord,
                        }
                    }
                    (None, None) => return std::cmp::Ordering::Equal,
                    (None, _) => return std::cmp::Ordering::Less,
                    (_, None) => return std::cmp::Ordering::Greater,
                }
            }
        })
}

pub fn sort(mut input: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    input.sort_by(|a, b| comparator(a, b));
    input
}

pub fn sort_tuples(mut input: Vec<(Vec<Vec<u8>>, u128)>) -> Vec<(Vec<Vec<u8>>, u128)> {
    input.sort_by(|a, b| comparator(&a.0, &b.0));
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

    // Compute the derivative of 'f' and calculate GCD with 'f' to find repeated factors
    let mut c = gcd(&f, &diff(f.clone()));
    (f, _) = divmod(&f, &c);

    let mut one_vect = vec![vec![0u8; 16]];
    one_vect[0][0] = 0x80; // The constant polynomial '1'

    let mut e: u128 = 1;

    while f != one_vect {
        let y = gcd(&f, &c);
        if f != y {
            let (factor, _) = divmod(&f, &y);
            factor_found.push((factor, e));
        }

        f = y.clone();
        (c, _) = divmod(&c, &y);
        e += 1;
    }
    
    if c != one_vect {
        for (factor, e) in sff(&sqrt(&c)) {
            factor_found.push((factor, e * 2));
        }
    }
    sort_tuples(factor_found)
}

pub fn ddf(f: &Vec<Vec<u8>>) -> Vec<(Vec<Vec<u8>>, u128)> {
    let mut z: Vec<(Vec<Vec<u8>>, u128)> = Vec::new();
    let mut d: u32 = 1;

    // Represent the polynomial 'x'
    let mut x = vec![vec![0u8; 16]; 2];
    x[1][0] = 0x80;

    // The constant polynomial '1'
    let mut one_vect = vec![vec![0u8; 16]];
    one_vect[0][0] = 0x80; 

    let mut fstar = f.clone();

    while (fstar.len() as u32 - 1) >= 2 * d {
        let mut h = x.clone();

        // Compute h = x^{2^{128d}} mod fstar by performing 128*d squarings
        for _ in 0..(128 * d) {
            h = mul(&h, &h); // Square 'h'
            (_, h) = divmod(&h, &fstar); // Reduce modulo 'fstar' to keep degrees manageable
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
        z.push((fstar.clone(), (fstar.len() as u128 - 1)));
    } else if z.len() == 0 {
        z.push((f.clone(), 1 ));
    }
    sort_tuples(z) 
}

// Modular exponentiation for polynomials with BigUint exponent
pub fn powmod_bigint(base: &Vec<Vec<u8>>, exponent: &BigUint, modulus: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let mut result = vec![vec![0u8; 16]];
    result[0][0] = 0x80;

    let mut base = base.clone();
    let mut exponent = exponent.clone();

    // Perform exponentiation using the square-and-multiply algorithm
    while !exponent.is_zero() {
        // If the least significant bit of the exponent is '1'
        if &exponent & BigUint::one() == BigUint::one() {
            result = mul(&result, &base);
            (_, result) = divmod(&result, modulus);
        }
        // Devide by 2 --> Shift the exponent right by 1 bit
        exponent >>= 1;
        if !exponent.is_zero() {

            base = mul(&base, &base);   // Square the base polynomial
            (_, base) = divmod(&base, modulus); // Reduce modulo 'modulus'
        }
    }
    result
}

fn random_poly(max_degree: usize) -> Vec<Vec<u8>> {
    let deg_h = rand::thread_rng().gen_range(1..=max_degree);
    let mut h = vec![vec![0u8; 16]; deg_h + 1];
    for coeff in &mut h {
        for byte in coeff.iter_mut() {
            *byte = rand::random();
        }
    }
    h
}

pub fn edf(f: &Vec<Vec<u8>>, d: usize) -> Vec<Vec<Vec<u8>>> {
    let q: BigUint = BigUint::from(2u32).pow(128); // Compute q = 2^128

    let n = (f.len() - 1) / d; // Ensure integer division
    let mut z = vec![f.clone()];

    // The polynomial 1
    let mut one_vect = vec![vec![0u8; 16]];
    one_vect[0][0] = 0x80;

    // Compute the exponent: (q^d - 1) / 3
    let exponent = (&q.pow(d as u32) - BigUint::one()) / BigUint::from(3u32);

    while z.len() < n {
        // Generate a random polynomial 'h' of degree less than deg(f)
        let deg_f = f.len() -1;
        let h = random_poly(deg_f - 1);

        // Compute g = (h^((q^d - 1)/3) - 1) mod f
        let mut g = powmod_bigint(&h, &exponent, f);
        g = add(&g, &one_vect);

        let mut new_z = Vec::new(); // Temporary vector to store updated factors

        // Attempt to factor each polynomial 'u' in 'z'
        for u in &z {
            if u.len() -1  > d {
                let j = gcd(u, &g);
                if j != one_vect && j != *u {
                    let (quotient, _) = divmod(u, &j);
                    new_z.push(j);
                    new_z.push(quotient);
                } else {
                    new_z.push(u.clone());
                }
            } else {
                new_z.push(u.clone());
            }
        }
        z = new_z; // Update 'z' with the new set of factors
    }
    sort(z) // Sort and return the list of factors of degree 'd'
}