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

mod tests {
    use super::*;

    #[test]
    fn encrypt_normal() {
        let key: Vec<u8> = vec![0x8a, 0xcb, 0x43, 0x01, 0x27, 0xa2, 0x9d, 0xca, 0x28, 0x95, 0xea, 0xca, 0x11, 0x8a, 0xe8, 0x7e];
        let input: Vec<u8> = vec![0xca, 0xfe, 0xba, 0xbe, 0xfa, 0xce, 0xdb, 0xad, 0xde, 0xca, 0xf8, 0x88, 0x88, 0x33, 0x44, 0x55];

        let expected: Vec<u8> = vec![0x0f, 0x91, 0x43, 0xa3, 0x78, 0x95, 0x06, 0x80, 0x4d, 0xf6, 0x05, 0x62, 0xf7, 0xf3, 0x12, 0x29];

        let result = execute("encrypt".to_string(), &key, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn decrypt_normal() {
        let key: Vec<u8> = vec![0x8a, 0xcb, 0x43, 0x01, 0x27, 0xa2, 0x9d, 0xca, 0x28, 0x95, 0xea, 0xca, 0x11, 0x8a, 0xe8, 0x7e];
        let input: Vec<u8> = vec![0x0f, 0x91, 0x43, 0xa3, 0x78, 0x95, 0x06, 0x80, 0x4d, 0xf6, 0x05, 0x62, 0xf7, 0xf3, 0x12, 0x29];

        let expected: Vec<u8> = vec![0xca, 0xfe, 0xba, 0xbe, 0xfa, 0xce, 0xdb, 0xad, 0xde, 0xca, 0xf8, 0x88, 0x88, 0x33, 0x44, 0x55];

        let result = execute("decrypt".to_string(), &key, input);
        assert_eq!(result, expected);
    }
}