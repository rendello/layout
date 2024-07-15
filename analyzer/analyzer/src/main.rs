#![allow(dead_code)]

mod data;
mod syllabic_unit;

use crate::data::SYLLABIC_MAP;
use crate::syllabic_unit::{SyllabicUnitMap, SyllabicUnit};

use arrayvec::ArrayVec;

pub enum ParseResult<T> {
    Failure,
    Success(T),
}

fn try_parse_syllabics(text: &str) -> ParseResult<Vec<&SyllabicUnit>> {
    let tokenizer = &mut SyllabicTokenizer::new(text);

    let mut v = Vec::new();

    while let Some(syllabic_unit) = tokenizer.next() {
        v.push(syllabic_unit);
    }

    match tokenizer.is_consumed() {
        true => ParseResult::Success(v),
        false => ParseResult::Failure
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
    const MAX: usize = 16;

    let mut char_end_indices = ArrayVec::<usize, MAX>::new();
    for char_end_index in CharEndIndices::new(text, MAX) {
        char_end_indices.push(char_end_index);
    }

    for &index in char_end_indices.iter().rev() {
        if let Some(&su) = map.get(&text[..index]) {
            return Some((su, &text[index..]))
        }
    }
    None
}

struct CharEndIndices<'a> {
    buffer: &'a [u8],
    index: usize,
    max: usize
}

impl<'a> CharEndIndices<'a> {
    pub fn new(text: &'a str, max: usize) -> CharEndIndices<'a> {
        CharEndIndices { buffer: text.as_bytes(), index: 0, max: max }
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
}

impl<'a> Iterator for CharEndIndices<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index == self.buffer.len() || index > self.max {
            None
        } else {
            let jump = Self::next_jump(self.buffer[self.index]);
            self.index += jump;
            Some(index + jump)
        }
    }
}


pub fn main() {
    let res = try_parse_syllabics("ᐃᓄᐃᑦ");
    if let ParseResult::Success(vec) = res {
        for element in vec {
            print!("{}", element.normalized_string());
        }
    }
}