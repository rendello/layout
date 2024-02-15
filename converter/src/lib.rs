use phf;

pub mod maps;

type PMap = phf::Map<&'static [u8], &'static str>;


fn get(maps: &[&PMap], key: &[u8]) -> Option<&'static str> {
    for map in maps {
        if let Some(v) = map.get(key) {
            return Some(v)
        }
    }
    None
}

fn bind_key_lengths(key_lengths: &[usize], max: usize) -> &[usize] {
    for (i, &length) in key_lengths.iter().enumerate() {
        if length < max { return &key_lengths[i..]; }
    };
    &key_lengths[key_lengths.len()..]
}


pub fn f(maps: &[&PMap], key_lengths: &[usize], text: &[u8], mut buffer: String) -> String {
    if text.is_empty() { return buffer }

    let bounded_key_lengths = bind_key_lengths(key_lengths, text.len()+1);
    for &length in bounded_key_lengths {
        if let Some(v) = get(maps, &text[..length]) {
            buffer.push_str(&v);
            return f(maps, key_lengths, &text[length..], buffer);
        }
    }
    let jump = next_jump(text[0]);
    buffer.push_str(std::str::from_utf8(&text[..jump]).unwrap());
    return f(maps, key_lengths, &text[jump..], buffer);
}


pub fn to_lat(text: &str) -> String {
    let key_lengths = maps::KEY_LENGTHS_TO_LAT;
    let maps = [&maps::BASE_TO_LAT];
    let string = String::new();
    f(&maps[..], &key_lengths, &text.as_bytes(), string)
}


pub fn to_syl(text: &str) -> String {
    let key_lengths = maps::KEY_LENGTHS_TO_SYL;
    let maps = [&maps::BASE_TO_SYL];
    let string = String::new();
    f(&maps[..], &key_lengths, &text.as_bytes(), string)
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