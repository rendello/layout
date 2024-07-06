use once_cell::sync::Lazy;
use regex::{Regex, Match};

static NON_INUK_ASCII: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\A[a-z]*[defowxyz][a-z]*").unwrap());
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\s+").unwrap());
static SKIP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\A[!#$%&'()*+,\\/:;<=>?."]+"#).unwrap());
static ANY: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\S+").unwrap());

#[derive(Debug)]
struct Token<'a> {
    tag: TokenTag,
    substring: &'a str
}

#[derive(Debug)]
enum TokenTag {
    NonInukASCII,
    Whitespace,
    Skip,
    Any
}

struct Tokens<'a> {
    buffer: &'a str
}

impl<'a> Tokens<'a> {
    fn new(text: &str) -> Tokens {
        Tokens { buffer: text }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let patterns = [
            (TokenTag::NonInukASCII, &NON_INUK_ASCII),
            (TokenTag::Whitespace, &WHITESPACE),
            (TokenTag::Skip, &SKIP),
            (TokenTag::Any, &ANY),
        ];

        for (tag, pattern) in patterns {
            if let Some(result) = pattern.find(self.buffer) {
                self.buffer = &self.buffer[result.len()..];
                return Some(Token { tag: tag, substring: result.as_str() });
            }
        }
        None
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
