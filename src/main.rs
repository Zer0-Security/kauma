mod parser;
mod actions;

use serde_json::{json, Value};
use std::env;
use parser::TestCase;
use actions::*;

fn main() {
    // Collect the path from command-line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Parse the JSON file into test cases
    let test_cases = parser::parse_test_cases(path).expect("Failed to parse JSON");
    let mut responses = serde_json::Map::new();

    for (id, test_case) in test_cases {
        let result = match test_case {
            TestCase::add_numbers { number1, number2 } => {
                let sum = add_numbers::execute(number1, number2);
                json!({"sum": sum})
            }
            TestCase::sub_numbers { number1, number2 } => {
                let difference = subtract_numbers::execute(number1, number2);
                json!({"difference": difference})
            }
            TestCase::poly2block { semantic, coefficients } => {
                let byte_vect = poly2byte::execute(&semantic, coefficients);
                json!({"block": de_encode_base64::encode(byte_vect)}) // encoding byte_vect to base 64 String
            }
            TestCase::block2poly { semantic, block } => {
                let coefficients = block2poly::execute(&semantic, block);
                json!({"coefficients": coefficients})
            }
            TestCase::gfmul { semantic, a, b } => {
                let a = de_encode_base64::decode( a).unwrap();
                let b = de_encode_base64::decode( b).unwrap();

                let product = gf_operations::gfmul(&semantic, a, b);
                json!({"product": de_encode_base64::encode(product)}) // encoding to base 64
            }
            TestCase::gfdiv { a, b } => {
                let a = de_encode_base64::decode( a).unwrap();
                let b = de_encode_base64::decode( b).unwrap();

                let quotient = gf_operations::gfdiv(a, b);
                json!({"q": de_encode_base64::encode(quotient)}) // encoding to base 64
            }
            TestCase::sea128 { mode, key, input } => {
                let key = de_encode_base64::decode( key).unwrap();
                let input = de_encode_base64::decode( input).unwrap();

                let output = aes_sea_128::execute(&"sea128".to_string(), &mode, &key, input);
                json!({"output": de_encode_base64::encode(output)}) // encoding to base 64
            }
            TestCase::xex { mode, key, tweak, input } => {
                let output = xex::execute(mode, key, tweak, input);
                json!({"output": de_encode_base64::encode(output)}) // encoding to base 64
            }
            TestCase::gcm_encrypt { algorithm, nonce, key, plaintext, ad } => {
                let nonce = de_encode_base64::decode(nonce).unwrap();
                let key = de_encode_base64::decode(key).unwrap();
                let plaintext = de_encode_base64::decode(plaintext).unwrap();
                let ad = de_encode_base64::decode(ad).unwrap();

                let output = gcm::encrypt(algorithm, nonce, key, plaintext, ad);
                json!({
                    "ciphertext": de_encode_base64::encode(output.0),
                    "tag": de_encode_base64::encode(output.1),
                    "L": de_encode_base64::encode(output.2),
                    "H": de_encode_base64::encode(output.3)
                })
            }
            TestCase::gcm_decrypt { algorithm, nonce, key, ciphertext, ad, tag } => {
                let nonce = de_encode_base64::decode(nonce).unwrap();
                let key = de_encode_base64::decode(key).unwrap();
                let ciphertext = de_encode_base64::decode(ciphertext).unwrap();
                let ad = de_encode_base64::decode(ad).unwrap();
                let tag = de_encode_base64::decode(tag).unwrap();
                
                let output = gcm::decrypt(algorithm, nonce, key, ciphertext, ad, tag);
                json!({
                    "authentic": output.0,
                    "plaintext": de_encode_base64::encode(output.1)
                })
            }
            TestCase::padding_oracle { hostname, port, iv, ciphertext } => {
                let iv = de_encode_base64::decode(iv).unwrap();
                let ciphertext = de_encode_base64::decode(ciphertext).unwrap();
                
                let plaintext = padding_oracle::execute(hostname, port, iv, ciphertext).unwrap();
                json!({
                    "plaintext": de_encode_base64::encode(plaintext)
                })
            }
            TestCase::gfpoly_add { A, B } => {
                let a = de_encode_base64::decode_vectors(A);
                let b = de_encode_base64::decode_vectors(B);
                
                let summ = gfpoly_operations::add(&a, &b);
                json!({
                    "S": de_encode_base64::encode_vectors(summ)
                })
            }
            TestCase::gfpoly_mul { A, B } => {
                let a = de_encode_base64::decode_vectors(A);
                let b = de_encode_base64::decode_vectors(B);
                
                let product = gfpoly_operations::mul(&a, &b);
                json!({
                    "P": de_encode_base64::encode_vectors(product)
                })
            }
            TestCase::gfpoly_pow { A, k } => {
                let a = de_encode_base64::decode_vectors(A);
                
                let power = gfpoly_operations::pow(&a, k);

                json!({
                    "Z": de_encode_base64::encode_vectors(power)
                })
            }
            TestCase::gfpoly_divmod { A, B } => {
                let a = de_encode_base64::decode_vectors(A);
                let b = de_encode_base64::decode_vectors(B);
                
                let (q, r) = gfpoly_operations::divmod(&a, &b);

                json!({
                    "Q": de_encode_base64::encode_vectors(q),
                    "R": de_encode_base64::encode_vectors(r)
                })
            }
            TestCase::gfpoly_powmod { A, M, k } => {
                let a = de_encode_base64::decode_vectors(A);
                let m = de_encode_base64::decode_vectors(M);
                
                let power = gfpoly_operations::powmod(&a, &m, k);

                json!({
                    "Z": de_encode_base64::encode_vectors(power)
                })
            }
            TestCase::gfpoly_sort { polys } => {
                let mut input: Vec<Vec<Vec<u8>>> = Vec::new();
                let mut sorted: Vec<Vec<String>> = Vec::new();

                for poly in polys {
                    input.push(de_encode_base64::decode_vectors(poly));
                }

                input = gfpoly_operations::sort(input);

                for poly in input {
                    sorted.push(de_encode_base64::encode_vectors(poly));
                }
                json!({
                    "sorted_polys": sorted
                })
            }
            TestCase::gfpoly_make_monic { A } => {
                let mut a = de_encode_base64::decode_vectors(A);
                
                a = gfpoly_operations::make_monic(&a);

                json!({
                    "A*": de_encode_base64::encode_vectors(a)
                })
            }
            TestCase::gfpoly_sqrt { Q } => {
                let mut q = de_encode_base64::decode_vectors(Q);
                
                q = gfpoly_operations::sqrt(&q);

                json!({
                    "S": de_encode_base64::encode_vectors(q)
                })
            }
            TestCase::gfpoly_diff { F } => {
                let mut f = de_encode_base64::decode_vectors(F);
                
                f = gfpoly_operations::diff(f);

                json!({
                    "F'": de_encode_base64::encode_vectors(f)
                })
            }
            TestCase::gfpoly_gcd { A, B } => {
                let a = de_encode_base64::decode_vectors(A);
                let b = de_encode_base64::decode_vectors(B);
                
                let product = gfpoly_operations::gcd(&a, &b);
                json!({
                    "G": de_encode_base64::encode_vectors(product)
                })
            }
            TestCase::gfpoly_factor_sff { F } => {
                let factors = de_encode_base64::decode_vectors(F);

                let results: Vec<Value> = gfpoly_operations::sff(&factors)
                    .into_iter()
                    .map(|(f, e)| {
                        json!({
                            "factor": de_encode_base64::encode_vectors(f),
                            "exponent": e
                        })
                    })
                    .collect();

                json!({
                    "factors": results
                })
            }
            TestCase::gfpoly_factor_ddf { F } => {
                let factors = de_encode_base64::decode_vectors(F);

                let results: Vec<Value> = gfpoly_operations::ddf(&factors)
                    .into_iter()
                    .map(|(f, d)| {
                        json!({
                            "factor": de_encode_base64::encode_vectors(f),
                            "degree": d
                        })
                    })
                    .collect();

                json!({
                    "factors": results
                })
            }
            TestCase::gfpoly_factor_edf { F, d } => {
                let f = de_encode_base64::decode_vectors(F);
                
                let result = gfpoly_operations::edf(&f, d as usize);

                let encoded_factors: Vec<Vec<String>> = result
                    .iter()
                    .map(|factor| de_encode_base64::encode_vectors(factor.clone()))
                    .collect();

                json!({
                    "factors": encoded_factors
                })
            }
            TestCase::gcm_crack { nonce, m1, m2, m3, forgery } => {
                
                let nonce = de_encode_base64::decode(nonce).unwrap();

                // Decode m1 fields and create tuple
                let m1_ciphertext = de_encode_base64::decode(m1.ciphertext).unwrap();
                let m1_associated_data = de_encode_base64::decode(m1.associated_data).unwrap();
                let m1_tag = de_encode_base64::decode(m1.tag).unwrap();
                let m1 = (m1_ciphertext, m1_associated_data, m1_tag);
        
                // Decode m2 fields and create tuple
                let m2_ciphertext = de_encode_base64::decode(m2.ciphertext).unwrap();
                let m2_associated_data = de_encode_base64::decode(m2.associated_data).unwrap();
                let m2_tag = de_encode_base64::decode(m2.tag).unwrap();
                let m2 = (m2_ciphertext, m2_associated_data, m2_tag);
        
                // Decode m3 fields and create tuple
                let m3_ciphertext = de_encode_base64::decode(m3.ciphertext).unwrap();
                let m3_associated_data = de_encode_base64::decode(m3.associated_data).unwrap();
                let m3_tag = de_encode_base64::decode(m3.tag).unwrap();
                let m3 = (m3_ciphertext, m3_associated_data, m3_tag);
        
                // Decode forgery fields and create tuple
                let forgery_ciphertext = de_encode_base64::decode(forgery.ciphertext).unwrap();
                let forgery_associated_data = de_encode_base64::decode(forgery.associated_data).unwrap();
                let forgery = (forgery_ciphertext, forgery_associated_data);

                let (tag, h, mask) = gcm_crack::execute(nonce, m1, m2, m3, forgery);

                json!({
                    "tag": de_encode_base64::encode(tag),
                    "H": de_encode_base64::encode(h),
                    "mask": de_encode_base64::encode(mask)
                })
            }
        };

        // Add the result to the responses map with the ID as the key
        responses.insert(id, result);
    }

    // Prepare and print the final JSON output
    let output = json!({ "responses": responses });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());

}