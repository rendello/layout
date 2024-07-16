#![allow(dead_code)]

use analyzer::syllabic_parser;
use analyzer::syllabic_parser::{ParseResult};

pub fn main() {
    let res = syllabic_parser::try_parse_inuktitut_syllabics("ᐃᓄᐃᑦ");
    if let ParseResult::Success(inuktitut_word) = res {
        println!("{:?}", inuktitut_word.as_latin());
    } else {
        println!("AHHHHH");
    }
}