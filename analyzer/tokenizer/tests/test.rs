use tokenizer::tokenizer::{Tokenizer};

use proptest::prelude::*;
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000_000,
        .. ProptestConfig::default()
    })]


    #[test]
    fn tokens_iter_fully_consumes_str(s in ".+") {
        let mut tokens = Tokenizer::new(&s);
        assert_eq!(tokens.is_consumed(), false);

        while let Some(_token) = tokens.next() { }

        assert_eq!(tokens.is_consumed(), true);
    }
}

