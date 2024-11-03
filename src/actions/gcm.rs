use openssl::symm::Mode;

use super::{rsa_sea_128, de_encode_base64, gfmul};

pub fn encrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, plaintext: Vec<u8>, ad: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mode = &"encrypt".to_string();
    let counter: u32 = 1;

    let y = nonce.iter().chain(counter.to_be_bytes().iter()).cloned().collect(); 

    let auth_key = rsa_sea_128::execute(&algorithm, mode, &key, vec![0; 16]);
    let y0_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

    let ciphertext = de_encrypt(algorithm, mode, plaintext, nonce, key);
    let (mut q, l) = ghash(ciphertext.clone(), auth_key.clone(), ad);

    for i in 0..q.len() {
        q[i] ^= y0_encrypted[i];
    }

    (ciphertext, q, l, auth_key)
}

pub fn decrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, ciphertext: Vec<u8>, ad: Vec<u8>, tag: Vec<u8>) -> (bool, Vec<u8>) {

    let mode = &"encrypt".to_string();
    let counter: u32 = 1;

    let y = nonce.iter().chain(counter.to_be_bytes().iter()).cloned().collect(); 

    let auth_key = rsa_sea_128::execute(&algorithm, mode, &key, vec![0; 16]);
    let y0_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

    let (mut q, _l) = ghash(ciphertext.clone(), auth_key.clone(), ad);

    for i in 0..q.len() {
        q[i] ^= y0_encrypted[i];
    }

    let plaintext = de_encrypt(algorithm, mode, ciphertext, nonce, key);
    
    let authentic: bool;
    if q == tag {
        authentic = true;
    } else {
        authentic = false;
    }

    (authentic, plaintext)
}

fn de_encrypt(algorithm: String, mode: &String ,plaintext: Vec<u8>, nonce: Vec<u8>, key: Vec<u8>) -> Vec<u8> {

    let mut ciphertext: Vec<u8> = Vec::new();

    // Seperate the plaintext (byte vector) into chunks of 16 byte and iterate over them 
    for plaintext in plaintext.chunks(16).enumerate() {
        let counter: u32 = plaintext.0 as u32 + 2;
        let mut plaintext = plaintext.1.to_vec();

        let y = nonce.iter().chain(counter.to_be_bytes().iter()).cloned().collect();

        // En- or decrypt the chunk
        let y_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

        // XOR the chunk and the encrypted tweak
        for i in 0..plaintext.len() {
            plaintext[i] ^= y_encrypted[i];
        }

        // Append the chunk to the output vector
        ciphertext.append(&mut plaintext);

    }
    ciphertext
}

fn ghash(ciphertext: Vec<u8>, auth_key: Vec<u8>, ad: Vec<u8>) -> (Vec<u8>, Vec<u8>) {

    let counter_ad = de_encode_base64::u64_to_byte(&"xex".to_string(), (ad.len() * 8) as u64);
    let counter_ciphertext = de_encode_base64::u64_to_byte(&"xex".to_string(), (ciphertext.len() * 8) as u64);
    let l: Vec<u8> = counter_ad.iter().rev().chain(counter_ciphertext.iter().rev()).cloned().collect();

    let mut q: Vec<u8> = vec![0; 16];

    for chunk in ad.chunks(16) {
        q = ghash_round(q, chunk, &auth_key);
    }

    for chunk in ciphertext.chunks(16) {
        q = ghash_round(q, chunk, &auth_key);
    }

    q = ghash_round(q, &l, &auth_key);

    (q, l)
}

fn ghash_round(mut q: Vec<u8>, chunk: &[u8], auth_key: &Vec<u8>) -> Vec<u8> {
    let mut buffer = [0u8; 16]; // Prepare a 16-byte buffer
    let semantic = "gcm".to_string();

    buffer[..chunk.len()].copy_from_slice(chunk); // Copy chunk into the buffer, padding with zeros if chunk is smaller than 16 bytes

    for i in 0..chunk.len() {
        q[i] ^= chunk[i];
    }
        
    gfmul::execute(&semantic, q, auth_key.clone())
}
    