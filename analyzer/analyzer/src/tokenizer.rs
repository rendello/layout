use once_cell::sync::Lazy;
use regex::{Regex};


static NON_INUK_ASCII: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A[a-z]*[defowxyz][a-z]*").unwrap());
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\s+").unwrap());
static SKIP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A[!#$%&'()*+,\\/:;<=>?."\[\]0-9]+"#).unwrap());
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
        let patterns = [
            (TokenTag::NonInukASCII, &NON_INUK_ASCII),
            (TokenTag::Whitespace, &WHITESPACE),
            (TokenTag::Skip, &SKIP),
            (TokenTag::Other, &OTHER),
        ];

        for (tag, pattern) in patterns {
            if let Some(result) = pattern.find(self.buffer) {
                self.buffer = &self.buffer[result.len()..];
                return Some(Token { tag, substring: result.as_str() });
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    tag: TokenTag,
    substring: &'a str
}

#[derive(Debug)]
pub enum TokenTag {
    NonInukASCII,
    Whitespace,
    Skip,
    Other
}