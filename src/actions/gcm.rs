use super::{rsa_sea_128, de_encode_base64};

pub fn encrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, plaintext: Vec<u8>, ad: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mode = &"encrypt".to_string();
    let semantic = &"gcm".to_string();
    let counter: u32 = 1;

    let y = nonce.iter().chain(de_encode_base64::u32_to_byte(semantic, counter).iter()).cloned().collect(); 

    let auth_key = rsa_sea_128::execute(&algorithm, mode, &key, vec![0; 16]);
    let y0_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

    let mut ciphertext: Vec<u8> = Vec::new();

    // Seperate the plaintext (byte vector) into chunks of 16 byte and iterate over them 
    for plaintext in plaintext.chunks(16).enumerate() {
        let counter: u32 = plaintext.0 as u32 + 2;
        let mut plaintext = plaintext.1.to_vec();

        let y = nonce.iter().chain(de_encode_base64::u32_to_byte(semantic, counter).iter()).cloned().collect();

        println!("Nonce{:?}", nonce);
        println!("Y{:?}", y);
        println!("Plaintext1{:?}", plaintext);

        // En- or decrypt the chunk
        let y_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

        // XOR the chunk and the encrypted tweak
        for i in 0..plaintext.len() {
            plaintext[i] ^= y_encrypted[i];
        }
        println!("Y_Encrypted{:?}", y_encrypted);
        println!("Plaintext2{:?}", plaintext);

        // Append the chunk to the output vector
        ciphertext.append(&mut plaintext);

    }
    println!("ciphertext{:?}", ciphertext);
    println!("-------------------");
    (ciphertext, vec![0; 16], vec![0; 16], auth_key)
}

pub fn decrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, ciphertext: Vec<u8>, ad: Vec<u8>, tag: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    (vec![0; 16], vec![0; 16])
}
