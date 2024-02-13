
use std::cmp;

use phf;

pub mod maps;

type PMap = phf::Map<&'static [u8], &'static str>;

const MAX_KEY_LENGTH: usize = 5;



fn get(maps: &[&PMap], key: &[u8]) -> Option<&'static str> {
    for map in maps {
        if let Some(v) = map.get(key) {
            return Some(v)
        }
    }
    None
}

pub fn f(maps: &[&PMap], text: &[u8], mut buffer: String) -> String {
    if text.is_empty() { return buffer }

    let max_key_length = cmp::min(MAX_KEY_LENGTH, text.len());
    for length in (1..(max_key_length+1)).rev() {
        if let Some(v) = get(maps, &text[..length]) {
            buffer.push_str(&v);
            return f(maps, &text[length..], buffer);
        }
    }
    let jump = next_jump(text[0]);
    buffer.push_str(std::str::from_utf8(&text[..jump]).unwrap());
    return f(maps, &text[jump..], buffer);
}


fn next_jump(byte: u8) -> usize {
    if byte & 0b1000_0000 == 0 {
        1
    } else if 0b1110_0000 & byte == 0b1100_0000 {
        2
    } else if 0b1111_0000 & byte == 0b1110_0000 {
        3
    } else if 0b1111_1000 & byte == 0b1111_0000 {
        4
    } else {
        unreachable!();
    }
}