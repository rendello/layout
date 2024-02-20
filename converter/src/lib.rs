use phf;

pub mod maps;

#[cfg(feature = "wasm")]
pub mod wasm;

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

pub struct Dialect {
    maps: &'static [&'static PMap],
    key_lengths: &'static [usize],
}

pub fn f2(dialect: Dialect, text_: &str) -> String {
    let text = text_.as_bytes();
    let text_len = text.len();

    let mut buffer = String::with_capacity(text_len*2);
    let mut index = 0;

    'main: loop {
        if index == text_len { break buffer }

        for &length in bind_key_lengths(dialect.key_lengths, text_len-index+1) {
            if let Some(v) = get(dialect.maps, &text[index..index+length]) {
                buffer.push_str(&v);
                index += length;
                continue 'main;
            }
        }
        let jump = next_jump(text[index]);
        buffer.push_str(std::str::from_utf8(&text[index..index+jump]).unwrap());
        index += jump;
    }
}

pub fn to_lat(text: &str) -> String {
    let key_lengths = maps::KEY_LENGTHS_TO_LAT;
    let maps = [&maps::BASE_TO_LAT];
    let string = String::new();
    f(&maps[..], &key_lengths, &text.as_bytes(), string)
}


static M: [&'static PMap; 1]  = [&maps::BASE_TO_SYL];

pub fn to_syl(text: &str) -> String {
    let d = Dialect {
        maps: &M,
        key_lengths: &maps::KEY_LENGTHS_TO_SYL,
    };
    f2(d, text)
}


// pub fn to_syl(text: &str) -> String {
//     let key_lengths = maps::KEY_LENGTHS_TO_SYL;
//     let maps = [&maps::BASE_TO_SYL];
//     let string = String::new();
//     f(&maps[..], &key_lengths, &text.as_bytes(), string)
// }


pub fn normalize_syl(text: &str) -> String {
    to_syl(&to_lat(&text))
}


pub fn normalize_lat(text: &str) -> String {
    to_lat(&to_syl(&text))
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
