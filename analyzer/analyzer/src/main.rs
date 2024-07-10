#![allow(dead_code)]

mod data;
mod syllabic_unit;

use std::cmp;
use crate::data::SYLLABIC_MAP;
use crate::syllabic_unit::{SyllabicUnitMap, SyllabicUnit};

use arrayvec::ArrayVec;

// I want an iterator to tokenize text into syllabic units from syllabics
// I want a higher-level function to apply collect this iterator, then return like this:
// buffer is not empty -> None
// buffer is empty -> Some(Vec<SyllabicSyllabicUnit>)
//
// Eventually I'd like for there to be some logic for detecting non-Inuk syllabics.


fn try_parse_syllabics(text: &str) -> Result<Vec<&SyllabicUnit>, ()> {
    let tokenizer = &mut SyllabicTokenizer::new(text);

    let mut v = Vec::new();

    while let Some(syllabic_unit) = tokenizer.next() {
        v.push(syllabic_unit);
    }

    match tokenizer.is_consumed() {
        true => Ok(v),
        false => Err(())
    }
}


pub struct SyllabicTokenizer<'a> {
    buffer: &'a str
}

impl<'a> SyllabicTokenizer<'a> {
    pub fn new(text: &str) -> SyllabicTokenizer {
        SyllabicTokenizer { buffer: text }
    }

    pub fn is_consumed(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<'a> Iterator for SyllabicTokenizer<'a> {
    type Item = &'a SyllabicUnit;

    fn next(&mut self) -> Option<Self::Item> {
        let r = pop_syllabic_unit(&self.buffer, &SYLLABIC_MAP);
        match r {
            Some((syllabic_unit, new_buffer)) => {
                self.buffer = new_buffer;
                Some(syllabic_unit)
            },
            None => None
        }
    }
}


fn pop_syllabic_unit<'a>(text: &'a str, map: &SyllabicUnitMap) -> Option<(&'a SyllabicUnit, &'a str)> {
    const MAX: usize = 160;
    let mut char_end_indices = ArrayVec::<usize, MAX>::new();

    for (character_index, _) in text.char_indices().skip(1) {
        if character_index > MAX {
            break;
        } else {
            char_end_indices.push(character_index);
        }
    }
    char_end_indices.push(cmp::min(text.len(), MAX));

    for &character_index in char_end_indices.iter().rev() {
        if let Some(&su) = map.get(&text[0..character_index]) {
            return Some((su, &text[character_index..]))
        }
    }
    None
}



pub fn main() {
    // println!("{:?}", pop_syllabic_unit("ᕦᐃᐃHello world!", &SYLLABIC_MAP));

    if let Ok(vec) = try_parse_syllabics("ᐃᓄᒃᑎᑐᑦ") {
        for element in vec {
            println!("{:?}", element);
        }
    }
}