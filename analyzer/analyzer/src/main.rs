#![allow(dead_code)]

use analyzer::tokenizer::Tokenizer;

pub fn main() {
    let s = r#"Inuktitut syllabics (Inuktitut: ᖃᓂᐅᔮᖅᐸᐃᑦ, romanized: qaniujaaqpait,[1] or ᑎᑎᕋᐅᓯᖅ ᓄᑖᖅ, titirausiq nutaaq) is an abugida-type writing system used in Canada by the Inuktitut-speaking Inuit of the territory of Nunavut and the Nunavik and Nunatsiavut regions of Quebec and Labrador, respectively. In 1976, the Language Commission of the Inuit Cultural Institute made it the co-official script for the Inuit languages, along with the Latin script.

The name qaniujaaqpait [qaniujaːqpaˈit] derives from the root qaniq, meaning "mouth". The alternative, Latin-based writing system is named qaliujaaqpait (ᖃᓕᐅᔮᖅᐸᐃᑦ), and it derives from qaliit, a word describing the markings or the grain in rocks. Titirausiq nutaaq [titiʁauˈsiq nuˈtaːq] meaning "new writing system" is to be seen in contrast to titirausiit nutaunngittut (ᑎᑎᕋᐅᓰᑦ ᓄᑕᐅᓐᖏᑦᑐᑦ), the "old syllabics" used before the reforms of 1976.[2] 
    "#;
    let tokens = Tokenizer::new(s);

    for t in tokens.collect::<Vec<_>>() {
        print!("{}", t.as_html());
    }
}