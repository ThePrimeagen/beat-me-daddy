use std::collections::HashMap;

use itertools::Itertools;

fn encode(s: &str, length: usize) -> HashMap<String, u32> {
    return s.as_bytes().windows(length).fold(HashMap::new(), |mut map, chunk| {
        let str = chunk.iter().map(|x| char::from_u32(*x as u32).unwrap()).collect::<String>();
        if str.len() == length {
            *map.entry(str).or_insert(0) += 1;
        }
        return map;
    });
}

fn deflate(s: &str, r: &str, w: &str) -> String {
    return s.split(r).join(w);
}

fn encode_recurse(s: &str, encode_amounts: Vec<usize>) -> String {
}

fn main() {
    let string = "00808080800802a00g808008g080bbz0z0z0z0l0808082a0808082808080ad82808082aaq0104q0g4z0z0002r022az0z0005r0g1z0z0z0z0k010828085808280d080808582a08082d53z0m09z0z0z0p02s012dadadadadadadadadadadadadadadadfg05g05g05g05g05g05g05002dz0z0x";

    println!("Size: 2, {:#?}", );
    println!("Size: 4, {:#?}", encode(string, 4));
    println!("Size: 6, {:#?}", encode(string, 6));
}


