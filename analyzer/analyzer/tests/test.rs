use analyzer::tokenizer::{Tokenizer};

use proptest::char::CharStrategy;

/*
0x1400..0x167F,
0x18B0..0x18F5,
0x11AB0..0x11ABF
*/

fn syllabic_char() -> CharStrategy<'static> {
    CharStrategy::new_borrowed(&[], &[], &[
        '\u{1400}'..='\u{167F}',
        '\u{18B0}'..='\u{18F5}',
        '\u{11AB0}'..='\u{11ABF}'
    ])
}

fn syllabic_fuzz() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![
            1 => r".+",
            1 => r"[a-zA-Z ]+",
            1 => r"[ \t]{1,5}",
            10 => syllabic_string()
        ], 1..10).prop_map(|v| v.into_iter().collect())
}


pub fn syllabic_string() -> impl Strategy<Value = String> {
    prop::collection::vec(syllabic_char(), 1..10).prop_map(|v| v.into_iter().collect())
}


use proptest::prelude::*;
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000,
        .. ProptestConfig::default()
    })]


    #[test]
    fn tokens_iter_fully_consumes_str(s in syllabic_fuzz()) {
        let mut tokens = Tokenizer::new(&s);
        // println!("{:?}", s);
        assert_eq!(tokens.is_consumed(), false);

        while let Some(_token) = tokens.next() { }

        assert_eq!(tokens.is_consumed(), true);
    }
}

