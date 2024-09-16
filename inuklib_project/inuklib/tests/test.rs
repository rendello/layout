use inuklib::tokenizer::{Tokenizer};
use inuklib::data::utf8_case_data::*;
use inuklib::syllabic_parser::try_parse_inuktitut_latin;

use proptest::char::CharStrategy;
use proptest::sample::select;


const UTF8_TROUBLESOME_CASE_CHARS_SLICES: &[&[&str]] = &[
    &LOWERCASING_CONTRACTS,
    &LOWERCASING_EXPANDS,
    &LOWERCASING_EXPANDS_MULTI_CHAR,
    &UPPERCASING_CONTRACTS,
    &UPPERCASING_CONTRACTS_MULTI_CHAR,
    &UPPERCASING_EXPANDS,
    &UPPERCASING_EXPANDS_MULTI_CHAR
];


fn char_utf8_changes_size_on_case_change() -> BoxedStrategy<String> {
    select(UTF8_TROUBLESOME_CASE_CHARS_SLICES)
        .prop_flat_map(|slice| select(slice))
        .prop_map(|s| s.to_string())
        .boxed()
}

fn syllabic_char() -> CharStrategy<'static> {
    CharStrategy::new_borrowed(&[], &[], &[
        '\u{1400}'..='\u{167F}',	/* UCAS */
        '\u{18B0}'..='\u{18F5}',	/* UCAS Extended */
        '\u{11AB0}'..='\u{11ABF}'	/* UCAS Extended-A */
    ])
}

fn syllabic_string() -> impl Strategy<Value = String> {
    prop::collection::vec(syllabic_char(), 1..10).prop_map(|v| v.into_iter().collect())
}

fn syllabic_fuzz() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            1 => r".+",
            1 => r"[a-zA-Z]+",
            5 => char_utf8_changes_size_on_case_change(),
            10 => r"[ \t]{1,2}",
            10 => syllabic_string()
        ], 1..10).prop_map(|v| v.into_iter().collect())
}


use proptest::prelude::*;
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 100_000,
        .. ProptestConfig::default()
    })]


    #[test]
    fn tokens_iter_fully_consumes_str(s in syllabic_fuzz()) {
        let mut tokens = Tokenizer::new(&s);
        assert_eq!(tokens.is_consumed(), false);

        while let Some(_token) = tokens.next() { }

        assert_eq!(tokens.is_consumed(), true);
    }

    #[test]
    fn inuktitut_latin_fuzz(s in syllabic_fuzz()) {
        try_parse_inuktitut_latin(&s);
    }
}

