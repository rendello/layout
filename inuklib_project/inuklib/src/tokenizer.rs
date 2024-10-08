use once_cell::sync::Lazy;
use regex::{Regex};

use crate::syllabic_unit::Script::{Latin, Syllabic};
use syllabic_parser::{InuktitutWord};

use crate::syllabic_parser;
use crate::data::lookups::ENGLISH_WORDS;


static NON_INUK_ASCII: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A[a-z]*[defowxyz][a-z]*").unwrap());
static NUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A(?:\d+\.?)+"#).unwrap());
static SKIP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A\W+"#).unwrap());
static OTHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A\w+"#).unwrap());


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
            (TokenTag::Skip, &NUMERIC),
        ];

        for (tag, pattern) in bare_patterns {
            if let Some(result) = pattern.find(self.buffer) {
                self.buffer = &self.buffer[result.len()..];
                return Some(Token { tag, substring: result.as_str() });
            }
        }

        if let Some(result) = &OTHER.find(self.buffer) {
            let tag = match syllabic_parser::try_parse_inuktitut_syllabics(result.as_str()) {
                Some(inuktitut_word) => TokenTag::InuktitutWord(inuktitut_word),
                None => {
                    if ENGLISH_WORDS.contains(&result.as_str().to_lowercase()) {
                        TokenTag::NonInuktitutWord
                    } else {
                        match syllabic_parser::try_parse_inuktitut_latin(result.as_str()) {
                            Some(inuktitut_word) => TokenTag::InuktitutWord(inuktitut_word),
                            None => TokenTag::NonInuktitutWord
                        }
                    }
                }
            };
            self.buffer = &self.buffer[result.len()..];
            return Some(Token { tag, substring: result.as_str() });
        }
        None
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub tag: TokenTag<'a>,
    pub substring: &'a str
}

#[derive(Debug)]
pub enum TokenTag<'a> {
    InuktitutWord(InuktitutWord<'a>),
    NonInuktitutWord,
    Skip,
}

impl<'a> Token<'a> {
    pub fn tag_as_html_class_name(&self) -> String {
        match &self.tag {
            TokenTag::NonInuktitutWord => "non-inuktitut-word",
            TokenTag::Skip => "skip",
            TokenTag::InuktitutWord(InuktitutWord { script: Latin, .. }) => "inuktitut-word latin",
            TokenTag::InuktitutWord(InuktitutWord { script: Syllabic, .. }) => "inuktitut-word syllabic"
        }.to_owned()
    }

    pub fn as_latin(&self) -> String {
        match &self.tag {
            TokenTag::NonInuktitutWord | TokenTag::Skip => self.substring.to_owned(),
            TokenTag::InuktitutWord(iw) => iw.as_latin(),
        }
    }

    pub fn as_html(&self) -> String {
        let body_text = match &self.tag {
            TokenTag::NonInuktitutWord | TokenTag::Skip => self.substring.to_owned(),
            TokenTag::InuktitutWord(iw) => iw.as_html()
        };
        format!(r#"<span class="{}">{}</span>"#, &self.tag_as_html_class_name(), body_text)
    }
}