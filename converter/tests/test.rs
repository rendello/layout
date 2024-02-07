
use converter;

use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in r"[a-zA-Z ]") {
        let maps = [&converter::BASE];
        let string = String::new();
        println!("{}", s);
        converter::f(&maps[..], &s, string);
    }
}