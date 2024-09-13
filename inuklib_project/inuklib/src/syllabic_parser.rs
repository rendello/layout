//! Parsing functions for Inuktitut syllabics and latin scripts.

use crate::data::{LOOKUP_LATIN, LOOKUP_SYLLABIC};
use crate::syllabic_unit::{SyllabicUnitLookup, SyllabicUnit, SUToken, Script};

use arrayvec::ArrayVec;


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

pub fn try_parse_inuktitut_latin(text: &str) -> Option<InuktitutWord> {
    try_parse(text, &LOOKUP_LATIN)
}

pub fn try_parse_inuktitut_syllabics(text: &str) -> Option<InuktitutWord> {
    try_parse(text, &LOOKUP_SYLLABIC)
}


fn try_parse<'a>(text: &'a str, lookup: &'a SyllabicUnitLookup) -> Option<InuktitutWord<'a>> {
    let mut tokenizer = SyllabicTokenizer::new(text, lookup);
    let mut tokens = Vec::new();

    #[allow(clippy::while_let_on_iterator)]
    while let Some(token) = tokenizer.next() {
        tokens.push(token);
    }

    match tokenizer.is_consumed() {
        true => Some(InuktitutWord { buffer: tokens, script: lookup.script.clone() }),
        false => None
    }
}

struct SyllabicTokenizer<'a> {
    buffer: &'a str,
    lookup: &'a SyllabicUnitLookup
}

impl<'a> SyllabicTokenizer<'a> {
    fn new(text: &'a str, lookup: &'a SyllabicUnitLookup) -> SyllabicTokenizer<'a> {
        SyllabicTokenizer { buffer: text, lookup }
    }

    fn is_consumed(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<'a> Iterator for SyllabicTokenizer<'a> {
    type Item = SUToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = pop_syllabic_unit(self.buffer, self.lookup);
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
fn pop_syllabic_unit<'a>(text: &'a str, lookup: &'a SyllabicUnitLookup)
    -> Option<(&'a SyllabicUnit, &'a str, &'a str)> {

    const VEC_SIZE: usize = 100;

    // Find and store the character offsets of the normalized text `n_text`.
    // `n_text` is either a lowercased copy of the original text, or simply
    // the original text itself is no normalization is required.
    let n_text = if lookup.must_normalize {
        &text.to_lowercase()
    } else {
        text
    };

    let max_key_length = lookup.key_lengths[0];
    let mut n_offsets = ArrayVec::<(usize, usize), VEC_SIZE>::new();
    for (char_number, offset) in CharEndIndices::new(n_text).enumerate() {
        if offset > max_key_length {
            break;
        } else if lookup.key_lengths.contains(&offset) {
            n_offsets.push((char_number, offset));
        }
    }

    // Look for a match at the beginning of `n_text`, from the largest
    // potential key to the smallest, eg:
    // [nunavik] [nunavi]k [nunav]ik [nuna]vik [nun]avik [nu]avik --> Match from "nu"; or
    // [ᐃᓅᒃ] [ᐃᓅ]ᒃ [ᐃ]ᓅᒃ --> Match from "ᐃ"
    for (n_char_number, n_offset) in n_offsets.into_iter().rev() {

        if let Some(&syllabic_unit) = lookup.map.get(&n_text[..n_offset]) {
            // A match has been found. If the text has been normalized, we have to
            // find the corresponding indices in the original string, as
            // the size of upper- and lowercase letters may differ in UTF-8.
            let offset = match lookup.must_normalize {
                true => {
                    CharEndIndices::new(text)
                        .enumerate()
                        .find(|(o_char_number, _)| *o_char_number == n_char_number)
                        .map(|(_, o_offset)| o_offset)
                        .expect("Corresponding indices not found for original string.")
                },
                false => n_offset
            };
            return Some((syllabic_unit, &text[..offset], &text[offset..]))
        }
    }
    None
}


struct CharEndIndices<'a> {
    buffer: &'a [u8],
    index: usize
}

impl<'a> CharEndIndices<'a> {
    fn new(text: &'a str) -> CharEndIndices<'a> {
        CharEndIndices { buffer: text.as_bytes(), index: 0 }
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
        if index == self.buffer.len() {
            None
        } else {
            let jump = Self::next_jump(self.buffer[self.index]);
            self.index += jump;
            Some(index + jump)
        }
    }
}