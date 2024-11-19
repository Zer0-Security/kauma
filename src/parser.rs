use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct TestCases {
    pub testcases: HashMap<String, TestCase>,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
#[serde(tag = "action", content = "arguments")]
pub enum TestCase {
    add_numbers { number1: i32, number2: i32 },
    sub_numbers { number1: i32, number2: i32 },
    poly2block { semantic: String, coefficients: Vec<u8> },
    block2poly { semantic: String, block: String },
    gfmul { semantic: String, a: String, b: String },
    gfdiv { a: String, b: String },
    sea128 { mode: String, key: String, input: String },
    xex { mode: String, key: String, tweak: String, input: String },
    gcm_encrypt { algorithm: String, nonce: String, key: String, plaintext: String, ad: String },
    gcm_decrypt { algorithm: String, nonce: String, key: String, ciphertext: String, ad: String, tag: String },
    padding_oracle { hostname: String, port: u32, iv: String, ciphertext: String },
    gfpoly_add { A: Vec<String>, B: Vec<String> },
    gfpoly_mul { A: Vec<String>, B: Vec<String> },
    gfpoly_divmod { A: Vec<String>, B: Vec<String> },
    gfpoly_pow { A: Vec<String>, k: u128 },
    gfpoly_powmod { A: Vec<String>, M: Vec<String>, k: u128 },
    gfpoly_sort { polys: Vec<Vec<String>> },
    gfpoly_make_monic { A: Vec<String> },
    gfpoly_sqrt { Q: Vec<String> },
    gfpoly_diff { F: Vec<String> },
    gfpoly_gcd { A: Vec<String>, B: Vec<String> },
    gfpoly_factor_sff { F: Vec<String> },
    gfpoly_factor_ddf { F: Vec<String> },
    gfpoly_factor_edf { F: Vec<String>, d: u128 }
}


pub fn parse_test_cases(path: &str) -> Result<HashMap<String, TestCase>, serde_json::Error> {
    // Read the JSON file from the path
    let file_content = fs::read_to_string(path).expect("Failed to read file");
    let test_cases: TestCases = serde_json::from_str(&file_content)?;
    Ok(test_cases.testcases)
}
