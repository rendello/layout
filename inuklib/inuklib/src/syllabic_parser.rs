//! Parsing functions for Inuktitut syllabics and latin scripts.

use crate::data::{SYLLABIC_MAP, LATIN_MAP};
use crate::syllabic_unit::{SyllabicUnitMap, SyllabicUnit, SUToken};

use arrayvec::ArrayVec;

#[derive(Debug)]
pub enum Script {
    Latin,
    Syllabic
}

#[derive(Debug)]
pub struct InuktitutWord<'a> {
    buffer: Vec<SUToken<'a>>,
    pub script: Script
}

impl<'a> InuktitutWord<'a> {
    pub fn as_latin(&self) -> String {
        self.buffer.iter().map(|su_token| su_token.text).collect()
    }

    pub fn as_html(&self) -> String {
        let mut v = Vec::new();
        for su_token in &self.buffer {
            v.push(format!(r#"<span class="syllabic-unit">{}</span>"#, su_token.text));
        }
        v.join("")
    }
}

pub enum ParseResult<T> {
    Failure,
    Success(T)
}

pub fn try_parse_inuktitut_latin(text: &str) -> ParseResult<InuktitutWord> {
    try_parse(text, &LATIN_MAP, Script::Latin)
}

pub fn try_parse_inuktitut_syllabics(text: &str) -> ParseResult<InuktitutWord> {
    try_parse(text, &SYLLABIC_MAP, Script::Syllabic)
}


fn try_parse<'a>(text: &'a str, map: &'a SyllabicUnitMap, script: Script) -> ParseResult<InuktitutWord<'a>> {
    let mut tokenizer = SyllabicTokenizer::new(text, map);

    let mut v = Vec::new();

    while let Some(su_token) = tokenizer.next() {
        v.push(su_token);
    }

    match tokenizer.is_consumed() {
        true => ParseResult::Success(InuktitutWord { buffer: v, script }),
        false => ParseResult::Failure
    }
}

struct SyllabicTokenizer<'a> {
    buffer: &'a str,
    map: &'a SyllabicUnitMap
}

impl<'a> SyllabicTokenizer<'a> {
    fn new(text: &'a str, map: &'a SyllabicUnitMap) -> SyllabicTokenizer<'a> {
        SyllabicTokenizer { buffer: text, map }
    }

    fn is_consumed(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<'a> Iterator for SyllabicTokenizer<'a> {
    type Item = SUToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = pop_syllabic_unit(self.buffer, self.map);
        match r {
            Some((syllabic_unit, text, new_buffer)) => {
                self.buffer = new_buffer;
                Some(SUToken { syllabic_unit, text })
            },
            None => None
        }
    }
}


/// Given `text` and a map of `strs` to `SyllabicUnit`s, return the longest
/// match at the beginning of `text`, along with two string slices:
/// the consumed text (corresponding to the `SyllabicUnit`), and the unconsumed text.

// [nunavik] [nunavi]k [nunav]ik [nuna]vik [nun]avik [nu]avik --> Match: "nu"
// A lowercased character can have a different byte length than its counterpart.
pub fn pop_syllabic_unit<'a>(text: &'a str, map: &SyllabicUnitMap)
    -> Option<(&'a SyllabicUnit, &'a str, &'a str)>{
    const MAX: usize = 16;

    let lowercase = &text.to_lowercase();

    let mut lowercase_indices = ArrayVec::<usize, {MAX+1}>::new();
    for lowercase_index in CharEndIndices::new(lowercase, MAX) {
        lowercase_indices.push(lowercase_index);
    }

    // Seek match using lowercased text
    for (i, &lowercase_character_index) in lowercase_indices.iter().enumerate().rev() {
        if let Some(&su) = map.get(&lowercase[..lowercase_character_index]) {

            // Match found, find real character indices
            let mut text_indices = ArrayVec::<usize, {MAX+1}>::new();
            for text_index in CharEndIndices::new(text, MAX) {
                text_indices.push(text_index);
            }

            let text_index = text_indices[i];

            return Some((su, &text[..text_index], &text[text_index..]))
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
    fn new(text: &'a str, max: usize) -> CharEndIndices<'a> {
        CharEndIndices { buffer: text.as_bytes(), index: 0, max }
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