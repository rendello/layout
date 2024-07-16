#![allow(dead_code)]

mod data;
mod syllabic_unit;
mod syllabic_parser;

use crate::syllabic_parser::ParseResult;

pub fn main() {
    let res = syllabic_parser::try_parse_inuktitut_syllabics("ᐃᓄᐃᑦ");
    if let ParseResult::Success(vec) = res {
        for element in vec {
            print!("{}", element.normalized_string());
        }
    }
}