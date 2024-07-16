use once_cell::sync::Lazy;
use regex::{Regex};

use crate::syllabic_parser;
use syllabic_parser::{InuktitutWord, ParseResult};

static NON_INUK_ASCII: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A[a-z]*[defowxyz][a-z]*").unwrap());
static SKIP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A[!#$%&'()*+,\\/:;<=>?."\s\[\]0-9]+"#).unwrap());
static OTHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A[^!#$%&'()*+,\\/:;<=>?."\s\[\]0-9]+"#).unwrap());


pub struct Tokenizer<'a> {
    buffer: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &str) -> Tokenizer {
        Tokenizer { buffer: text }
    }

    pub fn is_consumed(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let bare_patterns = [
            (TokenTag::NonInuktitutWord, &NON_INUK_ASCII),
            (TokenTag::Skip, &SKIP),
        ];

        for (tag, pattern) in bare_patterns {
            if let Some(result) = pattern.find(self.buffer) {
                self.buffer = &self.buffer[result.len()..];
                return Some(Token { tag, substring: result.as_str() });
            }
        }

        if let Some(result) = &OTHER.find(self.buffer) {
            let tag = match syllabic_parser::try_parse_inuktitut_syllabics(result.as_str()) {
                ParseResult::Success(inuktitut_word) => TokenTag::InuktitutWord(inuktitut_word),
                ParseResult::Failure => TokenTag::NonInuktitutWord
            };
            self.buffer = &self.buffer[result.len()..];
            return Some(Token { tag, substring: result.as_str() });
        }

        None
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    tag: TokenTag<'a>,
    substring: &'a str
}

#[derive(Debug)]
pub enum TokenTag<'a> {
    InuktitutWord(InuktitutWord<'a>),
    NonInuktitutWord,
    Skip,
}