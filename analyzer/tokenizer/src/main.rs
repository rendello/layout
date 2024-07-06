use once_cell::sync::Lazy;
use regex::{Regex, Match};

static NON_INUK_ASCII: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A[a-z]*[defowxyz][a-z]*").unwrap());
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\s+").unwrap());
static SKIP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A[!#$%&'()*+,\\/:;<=>?."]+"#).unwrap());
static ANY: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\S+").unwrap());

#[derive(Debug, PartialEq)]
struct Token<'a> {
    tag: TokenTag,
    substring: &'a str
}

#[derive(Debug, PartialEq)]
enum TokenTag {
    NonInukASCII,
    Whitespace,
    Skip,
    Other
}

struct Tokens<'a> {
    buffer: &'a str,
    cached_token: Option<Token<'a>>
}

impl<'a> Tokens<'a> {
    fn new(text: &str) -> Tokens {
        Tokens { buffer: text, cached_token: None }
    }

    fn advance(&mut self, token: &Token) {
        self.buffer = &self.buffer[token.substring.len()..];
    }

    fn try_pattern(&mut self, index: usize) -> Option<Token<'a>> {
        let patterns = [
            (TokenTag::NonInukASCII, &NON_INUK_ASCII),
            (TokenTag::Whitespace, &WHITESPACE),
            (TokenTag::Skip, &SKIP),
        ];

        for (tag, pattern) in patterns {
            println!("=== [{}]\n{:?}\n", index, &self.buffer[index..]);
            if let Some(result) = pattern.find(&self.buffer[index..]) {
                return Some(Token { tag: tag, substring: result.as_str() });
            }
        }
        None
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        if self.buffer.is_empty() {
            assert_eq!(self.cached_token, None);
            return None;
        }

        if let Some(token) = self.cached_token.take() {
            self.cached_token = None;
            self.advance(&token);
            return Some(token);
        }

        let mut index = 0;
        loop {
            if let Some(token) = self.try_pattern(index) {
                if index == 0 {
                    self.advance(&token);
                    return Some(token);
                } else {
                    self.cached_token = Some(token);
                    let other_token = Token {
                        tag: TokenTag::Other,
                        substring: &self.buffer[..index+1]
                    };
                    self.advance(&other_token);
                    return Some(other_token);
                }
            }
            if self.buffer.len() >= index {
                let token = Token {
                    tag: TokenTag::Other,
                    substring: self.buffer
                };
                self.advance(&token);
                return Some(token)
            }
            index += 1;
        }
    }
}

fn main() {
    // let s = "Here is a sample text with \nsome patterns 123 and symbols $%^&.";

    let s = r#"
Inuktitut (/ɪˈnʊktətʊt/ ih-NUUK-tə-tuut;[3] Inuktitut: [inuktiˈtut], syllabics ᐃᓄᒃᑎᑐᑦ; from inuk, 'person' + -titut, 'like', 'in the manner of'), also known as Eastern Canadian Inuktitut, is one of the principal Inuit languages of Canada. It is spoken in all areas north of the North American tree line, including parts of the provinces of Newfoundland and Labrador, Quebec, to some extent in northeastern Manitoba as well as the Northwest Territories and Nunavut. It is one of the aboriginal languages written with Canadian Aboriginal syllabics.[4] 
    "#;

    let tokens = Tokens::new(s);
    for t in tokens {
        println!("{:?}", t);
    }
}
