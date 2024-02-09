
use converter;

use proptest::prelude::*;


const RE: &str = r"(:?ai|i|ii|u|uu|a|aa|p|pai|pi|pii|pu|puu|pa|paa|t|tai|ti|tii|tu|tuu|ta|taa|k|kai|ki|kii|ku|kuu|ka|kaa|g|gai|gi|gii|gu|guu|ga|gaa|m|mai|mi|mii|mu|muu|ma|maa|n|nai|ni|nii|nu|nuu|na|naa|s|sai|si|sii|su|suu|sa|saa|l|lai|li|lii|lu|luu|la|laa|j|jai|ji|jii|ju|juu|ja|jaa|jj|jjai|jji|jjii|jju|jjuu|jja|jjaa|v|vai|vi|vii|vu|vuu|va|vaa|r|rai|ri|rii|ru|ruu|ra|raa|q|qai|qi|qii|qu|quu|qa|qaa|qq|qqai|qqi|qqii|qqu|qquu|qqa|qqaa|ng|ngai|ngi|ngii|ngu|nguu|nga|ngaa|nng|nngi|nngii|nngu|nnguu|nnga|nngaa|ł|łi|łii|łu|łuu|ła|łaa|b|h|ʼ| )*";

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000,
        .. ProptestConfig::default()
    })]


    #[test]
    // fn doesnt_crash(s in r"[\w[a-z] ]*") {
    fn doesnt_crash(s in RE) {
        let maps = [&converter::BASE];
        let string = String::new();
        // println!("{}", s);
        converter::f(&maps[..], &s.as_bytes(), string);
        // println!("{}\n{}\n", s, converter::f(&maps[..], &s.as_bytes(), string));
    }
}

#[test]
fn test_main() {
    let maps = [&converter::BASE, &converter::WESTERN];
    let string = String::new();
    println!("{:?}", converter::f(&maps[..], "inuktitut".as_bytes(), string));
}