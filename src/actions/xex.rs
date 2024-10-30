use crate::actions::{gfmul, sea128};
use super::de_encode_base64;

pub fn execute(mode: String, key: String, tweak: String, input: String) -> Vec<u8>{

    let key = de_encode_base64::decode(key).unwrap_or(vec![0; 32]);
    
    let key1: Vec<u8> = key[0..16].to_vec();
    let key2: Vec<u8> = key[16..32].to_vec();

    let tweak = de_encode_base64::decode(tweak).unwrap_or(vec![0; 16]);
    let input= de_encode_base64::decode( input).unwrap_or(vec![0; 16]);

    if input.is_empty() || input.len() < 16 {
       return input // If no input is given exit and return the empty string
    }

    let tweak_encrypted = sea128::execute(String::from("encrypt"), &key2, tweak);

   match mode.as_str() {
        "encrypt" => de_encrypt(mode, key1, tweak_encrypted, input),
        "decrypt" => de_encrypt(mode, key1, tweak_encrypted, input),
        _ => input
    }
}

fn de_encrypt(mode: String, key1: Vec<u8>, mut tweak_encrypted: Vec<u8>, input: Vec<u8>) -> Vec<u8> {

    let mut output: Vec<u8> = Vec::new();

    for chunk in input.chunks_exact(16) {
        let mut chunk = chunk.to_vec();

        for i in 0..chunk.len() {
            chunk[i] ^= tweak_encrypted[i];
        }

        chunk = sea128::execute(String::from(mode.as_str()), &key1.clone(), chunk);

        for i in 0..chunk.len() {
            chunk[i] ^= tweak_encrypted[i];
        }

        output.append(&mut chunk);
        tweak_encrypted = gfmul::execute(de_encode_base64::byte_to_u128(&"xex".to_string(), tweak_encrypted), 0x2).to_le_bytes().to_vec();
    }
    output
}