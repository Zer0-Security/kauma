use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::vec;

pub fn execute(hostname: String, port: u32, mut iv: Vec<u8>, ciphertext: Vec<u8>) -> Result<Vec<u8>, io::Error> {

    let mut plaintext = Vec::<u8>::new();

    for block in ciphertext.chunks(16).enumerate() {

        let mut intermediate_state: Vec<u8> = vec![0; 16];

        // Connect to the server
        let mut stream = TcpStream::connect( hostname.clone() + ":" + &port.to_string())?;   

        // Input Ciphertext 
        stream.write(&block.1)?;
        stream.flush()?;

        for (i, _byte) in block.1.iter().rev().enumerate() {
            intermediate_state[15-i] = correct_padding(&stream, &intermediate_state, 15 - i).unwrap();
        } 
        
        for i in 0..intermediate_state.len() {
            intermediate_state[i] = iv[i];
        }

        // Append the chunk to the output vector
        plaintext.append(&mut intermediate_state);
        iv = block.1.to_vec();
    }

    // Return the response as a Vec<u8>
    Ok(plaintext)
}

fn correct_padding(mut stream: &TcpStream, intermediate_state: &Vec<u8>, byte_num: usize) -> Result<u8, io::Error>{

    // Prepare q
    let mut q: Vec<u8> = vec![0; 16];
    for i in byte_num..=15 {
        q[i] = intermediate_state[i] ^ (16 - byte_num as u8);
    }
    
    // Create the 4-byte header (representing the number of 16-byte blocks sent to server)
    const LENGHT: usize = 256;
    let header = (LENGHT as u16).to_le_bytes();

    // Write Length to server
    stream.write(&header)?;
    stream.flush()?;

    for i in 0..LENGHT {
        q[byte_num] = i as u8;
        stream.write_all(&q)?;
        stream.flush()?;
    }

    let mut buffer = [0; LENGHT];
    let _bytes_read = stream.read(&mut buffer)?;

    if byte_num != 15 {
        for (i, byte) in buffer.iter().enumerate() {
            if  *byte == 1 {
                return Ok(i as u8 ^ (16 - byte_num as u8))
            }
        }
        return Ok(0) // No correct padding found
    } else {
        for (i, byte) in buffer.iter().enumerate() {
            if  *byte == 1 {
                
                // Create the 4-byte header (representing the number of 16-byte blocks sent to server)
                let lenght: u16 = 1;
                let header = (lenght as u16).to_le_bytes();

                // Write Length to server
                stream.write(&header)?;
                stream.flush()?;

                // Prepare q
                let mut q: Vec<u8> = vec![0; 16];
                q[15] = i as u8;
                q[14] = 1;

                // Write TCP to server
                stream.write_all(&q)?;
                stream.flush()?;
                
                // Get resposne from server
                let mut buffer = [0; 1];
                let byte_read = stream.read(&mut buffer)?;

                if byte_read == 1 {
                    return Ok(i as u8 ^ (16 - byte_num as u8))
                }
            }
        }
        return Ok(0) // No correct padding found
    }
}