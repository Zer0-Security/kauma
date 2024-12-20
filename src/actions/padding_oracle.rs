use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::vec;

pub fn execute(hostname: String, port: u32, mut iv: Vec<u8>, ciphertext: Vec<u8>) -> Result<Vec<u8>, io::Error> {

    let mut plaintext = Vec::<u8>::with_capacity(ciphertext.len());
    for block in ciphertext.chunks(16).enumerate() {
        let mut intermediate_state: Vec<u8> = vec![0; 16];

        let mut stream = TcpStream::connect(format!("{}:{}", hostname, port))?;  

        // Input Ciphertext 
        stream.write(&block.1)?;

        for (i, _byte) in block.1.iter().rev().enumerate() {
            intermediate_state[15-i] = correct_padding(&stream, &intermediate_state, 15 - i).unwrap();
        } 

        let _ = stream.shutdown(std::net::Shutdown::Both);
        
        for i in 0..iv.len() {
            intermediate_state[i] ^= iv[i];
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
    const LENGTH: usize = 256;
    let header = (LENGTH as u16).to_le_bytes();

    let mut message_buffer = Vec::with_capacity(LENGTH * 16);
    for i in 0..LENGTH {
        q[byte_num] = i as u8;
        message_buffer.extend_from_slice(&q);
    }

    // Combine the header and message buffer
    let mut combined_buffer = Vec::with_capacity(header.len() + message_buffer.len());
    combined_buffer.extend_from_slice(&header);
    combined_buffer.extend_from_slice(&message_buffer);

    stream.write_all(&combined_buffer)?; // write to server

    let mut buffer = [0; LENGTH];
    let _bytes_read = stream.read(&mut buffer)?;

    if byte_num != 15 {
        for (i, byte) in buffer.iter().enumerate() {
            if  *byte == 1 {
                return Ok(i as u8 ^ (16 - byte_num as u8))
            }
        }
        return Ok(0) // No correct padding found
    } else {
        let mut potential_padding: Vec<u8> = Vec::new();
        for (i, byte) in buffer.iter().enumerate() {
            if *byte == 1 {
                potential_padding.push(i as u8);
            }
        }

        if potential_padding.len() == 1 {
            return Ok(potential_padding[0] as u8 ^ (16 - byte_num as u8))
        }
            
        // Create the 4-byte header (representing the number of 16-byte blocks sent to server)
        let lenght: u16 = 1;
        let header = (lenght as u16).to_le_bytes();

        // Prepare q
        let mut q: Vec<u8> = vec![0; 16];
        q[15] = potential_padding[0];
        q[14] = 0xFF;

        // Combine the header and q into a single buffer
        let mut combined_buffer = Vec::with_capacity(header.len() + q.len());
        combined_buffer.extend_from_slice(&header);
        combined_buffer.extend_from_slice(&q);

        // Write the combined buffer to the server
        stream.write_all(&combined_buffer)?;

        // Get resposne from server
        let mut buffer = [0; 1];
        let _byte_read = stream.read(&mut buffer)?;
        if buffer[0] == 1 {
            return Ok(potential_padding[0] as u8 ^ (16 - byte_num as u8))
        } else {
            return Ok(potential_padding[1] as u8 ^ (16 - byte_num as u8))
        }
    }
}