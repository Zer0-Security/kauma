use crate::actions::{gfmul, sea128};
use super::de_encode_base64;

pub fn execute(mode: String, key: String, tweak: String, input: String) -> Vec<u8>{

    let key = de_encode_base64::decode(key).unwrap_or(vec![0; 32]);
    
    let key1: Vec<u8> = key[0..16].to_vec();
    let key2: Vec<u8> = key[16..32].to_vec();

    let tweak = de_encode_base64::decode(tweak).unwrap_or(vec![0; 16]);
    let input= de_encode_base64::decode( input).unwrap_or(vec![0; 16]);

    if input.is_empty() {
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
    let alpha = vec![0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

    // Seperate the input(byte vector) into chunks of 16 byte and iterate over them 
    for chunk in input.chunks_exact(16) {
        let mut chunk = chunk.to_vec();

        // XOR the chunk and the encrypted tweak
        for i in 0..chunk.len() {
            chunk[i] ^= tweak_encrypted[i];
        }

        // En- or decrypt the chunk
        chunk = sea128::execute(String::from(mode.as_str()), &key1.clone(), chunk);

        // XOR the chunk and the encrypted tweak
        for i in 0..chunk.len() {
            chunk[i] ^= tweak_encrypted[i];
        }

        // Append the chunk to the output vector
        output.append(&mut chunk);

        // Multiply the tweak by alpha
        tweak_encrypted = gfmul::execute( "xex".to_string(), tweak_encrypted, alpha.clone());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_normal() {
        let key = "B1ygNO/CyRYIUYhTSgoUysX5Y/wWLi4UiWaVeloUWs0=".to_string();
        let tweak = "6VXORr+YYHrd2nVe0OlA+Q==".to_string();
        let input = "/aOg4jMocLkBLkDLgkHYtFKc2L9jjyd2WXSSyxXQikpMY9ZRnsJE76e9dW9olZIW".to_string();

        let expected = de_encode_base64::decode("mHAVhRCKPAPx0BcufG5BZ4+/CbneMV/gRvqK5rtLe0OJgpDU5iT7z2P0R7gEeRDO".to_string()).unwrap();

        let result = execute("encrypt".to_string(), key, tweak, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn decrypt_normal() {
        let key = "B1ygNO/CyRYIUYhTSgoUysX5Y/wWLi4UiWaVeloUWs0=".to_string();
        let tweak = "6VXORr+YYHrd2nVe0OlA+Q==".to_string();
        let input = "lr/ItaYGFXCtHhdPndE65yg7u/GIdM9wscABiiFOUH2Sbyc2UFMlIRSMnZrYCW1a".to_string();

        let expected = de_encode_base64::decode("SGV5IHdpZSBrcmFzcyBkYXMgZnVua3Rpb25pZXJ0IGphIG9mZmVuYmFyIGVjaHQu".to_string()).unwrap();

        
        let result = execute("decrypt".to_string(), key, tweak, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn encrypt_empty() {
        let key = "B1ygNO/CyRYIUYhTSgoUysX5Y/wWLi4UiWaVeloUWs0=".to_string();
        let tweak = "6VXORr+YYHrd2nVe0OlA+Q==".to_string();
        let input = "".to_string();

        let expected = de_encode_base64::decode("".to_string()).unwrap();

        
        let result = execute("encrypt".to_string(), key, tweak, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn decrypt_empty() {
        let key = "B1ygNO/CyRYIUYhTSgoUysX5Y/wWLi4UiWaVeloUWs0=".to_string();
        let tweak = "6VXORr+YYHrd2nVe0OlA+Q==".to_string();
        let input = "".to_string();

        let expected = de_encode_base64::decode("".to_string()).unwrap();

        
        let result = execute("decrypt".to_string(), key, tweak, input);
        assert_eq!(result, expected);
    }
}