#![allow(dead_code)]

mod data;
mod syllabic_unit;

use crate::data::SYLLABIC_MAP;
use crate::syllabic_unit::{SyllabicUnitMap, SyllabicUnit};

pub fn main() {
    println!("{:?}", pop_syllabic_unit("ᕦᐃᐃHello world!", &SYLLABIC_MAP));
}


fn pop_syllabic_unit<'a>(text: &'a str, map: &SyllabicUnitMap) -> Option<(SyllabicUnit, &'a str)> {
    const MAX: usize = 16;
    let mut character_indices_array: [usize; MAX] = [0; MAX];
    let character_indices = {
        let mut array_index = 0;
        for (character_index, _f) in text.char_indices() {
            if character_index > MAX || array_index > MAX { break; }
            character_indices_array[array_index] = character_index;
            array_index += 1;
        }
        &character_indices_array[0..array_index]
    };

    for &character_index in character_indices.iter().rev() {
        if let Some(&su) = map.get(&text[0..character_index]) {
            return Some((su.clone(), &text[character_index..]))
        }
    }
    None
}

