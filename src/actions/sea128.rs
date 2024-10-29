use openssl::symm::{Cipher, Crypter, Mode};

pub fn execute(mode: String, key: &Vec<u8>, input: Vec<u8>) -> Vec<u8> {

    let xor: Vec<u8> = vec![0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0x11];

    match mode.as_str() {
        "encrypt" => encrypt(&key, input, xor),
        "decrypt" => decrypt(&key, input, xor),
        _ => input
    }
}

fn encrypt(key: &Vec<u8>, input: Vec<u8>, xor: Vec<u8>) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, None).unwrap();

    crypter.pad(false);

    let mut ciphertext = vec![0; 32]; // Prepare a 32-byte buffer for OpenSSL, even though we only need 16 bytes for the ciphertext
    let count = crypter.update(&input, &mut ciphertext).unwrap();

    ciphertext.truncate(count); // Truncate the buffer to the actual output size

    for i in 0..ciphertext.len() {
        ciphertext[i] ^= xor[i];
    }
    ciphertext
}

fn decrypt(key: &Vec<u8>, input: Vec<u8>, xor: Vec<u8>) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key, None).unwrap();

    crypter.pad(false);

    let mut xor_result = input.clone();
    for i in 0..xor_result.len() {
        xor_result[i] ^= xor[i];
    }

    let mut plaintext = vec![0; 32]; // Prepare a 32-byte buffer for OpenSSL, even though we only need 16 bytes for the plaintext
    let count = crypter.update(&xor_result, &mut plaintext).unwrap();

    plaintext.truncate(count); // Truncate the buffer to the actual decrypted size (should be 16 bytes)

    plaintext
}