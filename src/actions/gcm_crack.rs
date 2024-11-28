use super::{de_encode_base64, gcm, gf_operations, gfpoly_operations};

pub fn execute(
    _nonce: Vec<u8>, 
    m1: (Vec<u8>, Vec<u8>, Vec<u8>), 
    m2: (Vec<u8>, Vec<u8>, Vec<u8>), 
    m3: (Vec<u8>, Vec<u8>, Vec<u8>), 
    forgery: (Vec<u8>, Vec<u8>)
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {

    // Create L Block for m1
    let counter_ad = de_encode_base64::u64_to_byte(&"xex".to_string(), (m1.1.len() * 8) as u64);
    let counter_ciphertext = de_encode_base64::u64_to_byte(&"xex".to_string(), (m1.0.len() * 8) as u64);
    let l: Vec<u8> = counter_ad.iter().rev().chain(counter_ciphertext.iter().rev()).cloned().collect(); 

    let mut m1_whole: Vec<Vec<u8>> = Vec::new();
    m1_whole.push(m1.2.clone()); // Tag
    m1_whole.push(l); // L
    if m1.0.len() != 0 { m1_whole.extend(reorder_vector(m1.0.clone()));} // Reverse C-Blocks
    if m1.1.len() != 0 { m1_whole.extend(reorder_vector(m1.1.clone()));} // Reverse A-Blocks

    // Create L Block for m2
    let counter_ad = de_encode_base64::u64_to_byte(&"xex".to_string(), (m2.1.len() * 8) as u64);
    let counter_ciphertext = de_encode_base64::u64_to_byte(&"xex".to_string(), (m2.0.len() * 8) as u64);
    let l: Vec<u8> = counter_ad.iter().rev().chain(counter_ciphertext.iter().rev()).cloned().collect(); 

    let  mut m2_whole: Vec<Vec<u8>> = Vec::new();
    m2_whole.push(m2.2.clone()); // Tag
    m2_whole.push(l); // L
    if m2.0.len() != 0 { m2_whole.extend(reorder_vector(m2.0.clone()));} // Reverse C-Blocks
    if m2.1.len() != 0 { m2_whole.extend(reorder_vector(m2.1.clone()));} // Reverse A-Blocks

    // Initialize m1.ciphertext + m2.ciphertext
    let new_poly = gfpoly_operations::add(&m1_whole, &m2_whole);
    
    let sff = gfpoly_operations::sff(&new_poly);

    let mut ddf: Vec<(Vec<Vec<u8>>, u128)> = Vec::new();
    for poly in sff {
        ddf.extend(gfpoly_operations::ddf(&poly.0));
    }

    // Get all H-Candiadtates
    let mut h_candidates: Vec<Vec<u8>> = Vec::new();
    for poly in ddf {
        if poly.1 == 1 {
            if poly.0.len() == 2 {
                h_candidates.push(poly.0[0].clone());
                continue;
            }
            let edf = gfpoly_operations::edf(&poly.0, poly.1 as usize);
            for h in edf {
                h_candidates.push(h[0].clone());
            }
        }
    }

    // Get the coressponding H_ek to the H-Canditates
    let mut h_ek: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for h in h_candidates {
        let result = gcm::ghash(m1.0.clone(), h.clone(), m1.1.clone());
        h_ek.push((h.clone(), gf_operations::add_vec(&result.0, &m1.2.clone())));
    }

    // Check what tuple sis the correct one
    let mut correct_h_ek: (Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new());
    for tuple in h_ek {
        let result = gcm::ghash(m3.0.clone(), tuple.0.clone(), m3.1.clone());
        let tag = gf_operations::add_vec(&result.0, &tuple.1);

        if tag == m3.2 {
            correct_h_ek = tuple;
        }
    }

    // Authenticate m4
    let result = gcm::ghash(forgery.0.clone(), correct_h_ek.0.clone(), forgery.1.clone());
    let auth_tag = gf_operations::add_vec(&result.0, &correct_h_ek.1);

    (auth_tag, correct_h_ek.0, correct_h_ek.1)
}

fn reorder_vector(vec: Vec<u8>) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = vec.chunks(16)
        .map(|chunk| {
            let mut block: Vec<u8> = chunk.iter().cloned().collect();
            while block.len() < 16 {
                block.push(0); // Pad the block with 0s
            }
            block
        })
        .collect();

    blocks.reverse(); // Reverse the order of the blocks
    blocks
    // Flatten the reversed blocks back into a single Vec<u8>
}
