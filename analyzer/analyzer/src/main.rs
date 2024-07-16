#![allow(dead_code)]

mod data;
mod syllabic_unit;
mod syllabic_parser;

use crate::syllabic_parser::ParseResult;

pub fn main() {
    let res = syllabic_parser::try_parse_inuktitut_syllabics("ᐃᓄᐃᑦ");
    if let ParseResult::Success(inuktitut_word) = res {
        println!("{:?}", inuktitut_word.as_latin());
    } else {
        println!("AHHHHH");
    }
}