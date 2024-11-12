use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct TestCases {
    pub testcases: HashMap<String, TestCase>,
}

#[derive(Deserialize, Debug)]
pub struct TestCase {
    pub action: String,
    pub arguments: Arguments,
}


#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub enum Arguments {
    AddSubNumbers { number1: i32, number2: i32 },
    Poly2Block { semantic: String, coefficients: Vec<u8> },
    Block2Poly { semantic: String, block: String },
    GfMul { semantic: String, a: String, b: String },
    GfDiv { a: String, b: String },
    Sea128 { mode: String, key: String, input: String },
    Xex { mode: String, key: String, tweak: String, input: String },
    GcmEncrypt { algorithm: String, nonce: String, key: String, plaintext: String, ad: String },
    GcmDecrypt { algorithm: String, nonce: String, key: String, ciphertext: String, ad: String, tag: String },
    PaddingOracle { hostname: String, port: u32, iv: String, ciphertext: String },
    GfpolyTwoNum { A: Vec<String>, B: Vec<String> },
    GfpolyPow { A: Vec<String>, k: u8 }
}

pub fn parse_test_cases(path: &str) -> Result<HashMap<String, TestCase>, serde_json::Error> {
    // Read the JSON file from the path
    let file_content = fs::read_to_string(path).expect("Failed to read file");
    let test_cases: TestCases = serde_json::from_str(&file_content)?;
    Ok(test_cases.testcases)
}
