#![allow(dead_code)]

// use analyzer::syllabic_parser;
// use analyzer::syllabic_parser::{ParseResult};
use analyzer::tokenizer::Tokenizer;

pub fn main() {
    // let res = syllabic_parser::try_parse_inuktitut_syllabics("ᐃᓄᐃᑦ");
    // if let ParseResult::Success(inuktitut_word) = res {
    //     println!("{:?}", inuktitut_word.as_latin());
    // } else {
    //     println!("AHHHHH");
    // }

    let s = "Hello, world! ᐃᓄᐃᑦ 'the people', singular: Inuk, ᐃᓄᒃ, dual: Inuuk, ᐃᓅᒃ.";

    let tokens = Tokenizer::new(&s);

    for e in tokens.collect::<Vec<_>>() {
        println!("{:?}", e);
    }
}