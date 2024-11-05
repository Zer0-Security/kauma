use super::{rsa_sea_128, de_encode_base64, gfmul};

pub fn encrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, plaintext: Vec<u8>, ad: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mode = &"encrypt".to_string();
    let counter: u32 = 1;

    let y = nonce.iter().chain(counter.to_be_bytes().iter()).cloned().collect(); // Concat nonce and counter

    let auth_key = rsa_sea_128::execute(&algorithm, mode, &key, vec![0; 16]);
    let y0_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

    let ciphertext = de_encrypt(algorithm, mode, plaintext, nonce, key); // Encrypt plaintext with algorithem in JSON 
    let (mut q, l) = ghash(ciphertext.clone(), auth_key.clone(), ad); // Run the GHASH function, gives back Q and L

    // XOR bytewise of Q and Y0
    for i in 0..q.len() {
        q[i] ^= y0_encrypted[i];
    }

    (ciphertext, q, l, auth_key)
}

pub fn decrypt(algorithm: String, nonce: Vec<u8>, key: Vec<u8>, ciphertext: Vec<u8>, ad: Vec<u8>, tag: Vec<u8>) -> (bool, Vec<u8>) {
    let mode = &"encrypt".to_string();
    let counter: u32 = 1;

    let y = nonce.iter().chain(counter.to_be_bytes().iter()).cloned().collect(); // Concat nonce and counter

    let auth_key = rsa_sea_128::execute(&algorithm, mode, &key, vec![0; 16]);
    let y0_encrypted = rsa_sea_128::execute(&algorithm, mode, &key, y);

    let (mut q, _l) = ghash(ciphertext.clone(), auth_key.clone(), ad); // Run the GHASH function, gives back Q and L

    // XOR bytewise of Q and Y0
    for i in 0..q.len() {
        q[i] ^= y0_encrypted[i];
    }

    let plaintext = de_encrypt(algorithm, mode, ciphertext, nonce, key); // Decrypt plaintext with algorithem in JSON 
    
    // Check if tag is authentic
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

    // Initialize both counters and concat them to get the L-block
    let counter_ad = de_encode_base64::u64_to_byte(&"xex".to_string(), (ad.len() * 8) as u64);
    let counter_ciphertext = de_encode_base64::u64_to_byte(&"xex".to_string(), (ciphertext.len() * 8) as u64);
    let l: Vec<u8> = counter_ad.iter().rev().chain(counter_ciphertext.iter().rev()).cloned().collect();

    let mut q: Vec<u8> = vec![0; 16]; // Initial state q that will change every "round" for each block inputed

    // Do a GHASH round for each A-Block
    for chunk in ad.chunks(16) {
        q = ghash_round(q, chunk, &auth_key);
    }

    // Do a GHASH round for each C-Block
    for chunk in ciphertext.chunks(16) {
        q = ghash_round(q, chunk, &auth_key);
    }

    q = ghash_round(q, &l, &auth_key); // Do the last round with the L-block

    (q, l)
}

fn ghash_round(mut q: Vec<u8>, chunk: &[u8], auth_key: &Vec<u8>) -> Vec<u8> {
    let mut buffer = [0u8; 16]; // Prepare a 16-byte buffer
    let semantic = "gcm".to_string();

    buffer[..chunk.len()].copy_from_slice(chunk); // Copy chunk into the buffer, padding with zeros if chunk is smaller than 16 bytes

    // XOR the state Q and the inputed A- or C-block
    for i in 0..chunk.len() {
        q[i] ^= chunk[i];
    }
        
    gfmul::execute(&semantic, q, auth_key.clone()) // gfmul the state Q and the Key H
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_sea128_normal() {
        // Input preparation
        let algorithm = "sea128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("RGFzIGlzdCBlaW4gVGVzdA==".to_string()).unwrap();
        let ad= de_encode_base64::decode("QUQtRGF0ZW4=".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("0cI/Wg4R3URfrVFZ0hw/vg==".to_string()).unwrap();
        let tag = de_encode_base64::decode("ysDdzOSnqLH0MQ+Mkb23gw==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAEAAAAAAAAAAgA==".to_string()).unwrap();
        let h = de_encode_base64::decode("xhFcAUT66qWIpYz+Ch5ujw==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_sea128_no_plaintext() {
        // Input preparation
        let algorithm = "sea128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("".to_string()).unwrap();
        let ad= de_encode_base64::decode("QUQtRGF0ZW4=".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("".to_string()).unwrap();
        let tag = de_encode_base64::decode("kfqxEgz6sEge1QNdt6fviw==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAEAAAAAAAAAAAA==".to_string()).unwrap();
        let h = de_encode_base64::decode("xhFcAUT66qWIpYz+Ch5ujw==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_sea128_no_ad() {
        // Input preparation
        let algorithm = "sea128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("RGFzIGlzdCBlaW4gVGVzdA==".to_string()).unwrap();
        let ad= de_encode_base64::decode("".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("0cI/Wg4R3URfrVFZ0hw/vg==".to_string()).unwrap();
        let tag = de_encode_base64::decode("c0M+Xu7FOByIidpMyfvBjw==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAAAAAAAAAAAAgA==".to_string()).unwrap();
        let h = de_encode_base64::decode("xhFcAUT66qWIpYz+Ch5ujw==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_sea128_no_ad_and_plaintext() {
        // Input preparation
        let algorithm = "sea128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("".to_string()).unwrap();
        let ad= de_encode_base64::decode("".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("".to_string()).unwrap();
        let tag = de_encode_base64::decode("KS3GYAYuX8gzMKC7ymd2Cg==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAAAAAAAAAAAAAA==".to_string()).unwrap();
        let h = de_encode_base64::decode("xhFcAUT66qWIpYz+Ch5ujw==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn decrypt_sea128_normal() {
        // Input preparation
        let algorithm = "sea128".to_string();
        let nonce= de_encode_base64::decode("VOkKCCnH4EYE1z4L".to_string()).unwrap();
        let key= de_encode_base64::decode("ByMrTiLP7isfBDL7vsKkOQ==".to_string()).unwrap();
        let ciphertext= de_encode_base64::decode("UdpDzPAafM+y".to_string()).unwrap();
        let ad= de_encode_base64::decode("UknNF3AKBaF/8GUnFUw=".to_string()).unwrap();
        let tag= de_encode_base64::decode("sN0+1fG+WSOHMswF7IBnZA==".to_string()).unwrap();

        // Expected Output preparation
        let plaintext = de_encode_base64::decode("AxSiKm93Gr2+".to_string()).unwrap();
        let authentic = false;
        let expected = (authentic, plaintext);

        let result = decrypt(algorithm, nonce, key, ciphertext, ad, tag);
        assert_eq!(result, expected);
    }    

    #[test]
    fn encrypt_aes128_normal() {
        // Input preparation
        let algorithm = "aes128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("RGFzIGlzdCBlaW4gVGVzdA==".to_string()).unwrap();
        let ad= de_encode_base64::decode("QUQtRGF0ZW4=".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("ET3RmvH/Hbuxba63EuPRrw==".to_string()).unwrap();
        let tag = de_encode_base64::decode("Mp0APJb/ZIURRwQlMgNN/w==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAEAAAAAAAAAAgA==".to_string()).unwrap();
        let h = de_encode_base64::decode("Bu6ywbsUKlpmZXMQyuGAng==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_aes128_no_plaintext() {
        // Input preparation
        let algorithm = "aes128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("".to_string()).unwrap();
        let ad= de_encode_base64::decode("QUQtRGF0ZW4=".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("".to_string()).unwrap();
        let tag = de_encode_base64::decode("LTddUNp22HsA7W/rs5irPw==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAEAAAAAAAAAAAA==".to_string()).unwrap();
        let h = de_encode_base64::decode("Bu6ywbsUKlpmZXMQyuGAng==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_aes128_no_ad() {
        // Input preparation
        let algorithm = "aes128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("RGFzIGlzdCBlaW4gVGVzdA==".to_string()).unwrap();
        let ad= de_encode_base64::decode("".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("ET3RmvH/Hbuxba63EuPRrw==".to_string()).unwrap();
        let tag = de_encode_base64::decode("KV3D0XTHFDJXkDU9lCqUAQ==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAAAAAAAAAAAAgA==".to_string()).unwrap();
        let h = de_encode_base64::decode("Bu6ywbsUKlpmZXMQyuGAng==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_aes128_no_ad_and_plaintext() {
        // Input  preparation
        let algorithm = "aes128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let plaintext= de_encode_base64::decode("".to_string()).unwrap();
        let ad= de_encode_base64::decode("".to_string()).unwrap();

        // Expected Output preparation
        let ciphertext = de_encode_base64::decode("".to_string()).unwrap();
        let tag = de_encode_base64::decode("6dIooPnAnzfd8F9VCpiYGw==".to_string()).unwrap();
        let l = de_encode_base64::decode("AAAAAAAAAAAAAAAAAAAAAA==".to_string()).unwrap();
        let h = de_encode_base64::decode("Bu6ywbsUKlpmZXMQyuGAng==".to_string()).unwrap();
        let expected = (ciphertext, tag, l, h);

        let result = encrypt(algorithm, nonce, key, plaintext, ad);
        assert_eq!(result, expected);
    }

    #[test]
    fn decrypt_aes128_normal() {
        // Input  preparation
        let algorithm = "aes128".to_string();
        let nonce= de_encode_base64::decode("4gF+BtR3ku/PUQci".to_string()).unwrap();
        let key= de_encode_base64::decode("Xjq/GkpTSWoe3ZH0F+tjrQ==".to_string()).unwrap();
        let ciphertext= de_encode_base64::decode("ET3RmvH/Hbuxba63EuPRrw==".to_string()).unwrap();
        let ad= de_encode_base64::decode("QUQtRGF0ZW4=".to_string()).unwrap();
        let tag= de_encode_base64::decode("Mp0APJb/ZIURRwQlMgNN/w==".to_string()).unwrap();

        // Expected Output preparation
        let plaintext = de_encode_base64::decode("RGFzIGlzdCBlaW4gVGVzdA==".to_string()).unwrap();
        let authentic = true;
        let expected = (authentic, plaintext);

        let result = decrypt(algorithm, nonce, key, ciphertext, ad, tag);
        assert_eq!(result, expected);
    }
}

