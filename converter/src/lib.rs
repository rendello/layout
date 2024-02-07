
use std::cmp;

use phf::phf_map;


type PMap = phf::Map<&'static str, &'static str>;

const MAX_KEY_LENGTH: usize = 5;


pub static BASE: PMap = phf_map! {
"ai" => "ᐁ",
"i" => "ᐃ",
"ii" => "ᐄ",
"u" => "ᐅ",
"uu" => "ᐆ",
"a" => "ᐊ",
"aa" => "ᐋ",
"pai" => "ᐯ",
"pi" => "ᐱ",
"pii" => "ᐲ",
"pu" => "ᐳ",
"puu" => "ᐴ",
"pa" => "ᐸ",
"paa" => "ᐹ",
"p" => "ᑉ",
"tai" => "ᑌ",
"ti" => "ᑎ",
"tii" => "ᑏ",
"tu" => "ᑐ",
"tuu" => "ᑑ",
"ta" => "ᑕ",
"taa" => "ᑖ",
"t" => "ᑦ",
"kai" => "ᑫ",
"ki" => "ᑭ",
"kii" => "ᑮ",
"ku" => "ᑯ",
"kuu" => "ᑰ",
"ka" => "ᑲ",
"kaa" => "ᑳ",
"k" => "ᒃ",
"gai" => "ᒉ",
"gi" => "ᒋ",
"gii" => "ᒌ",
"gu" => "ᒍ",
"guu" => "ᒎ",
"ga" => "ᒐ",
"gaa" => "ᒑ",
"g" => "ᒡ",
"mai" => "ᒣ",
"mi" => "ᒥ",
"mii" => "ᒦ",
"mu" => "ᒧ",
"muu" => "ᒨ",
"ma" => "ᒪ",
"maa" => "ᒫ",
"m" => "ᒻ",
"nai" => "ᓀ",
"ni" => "ᓂ",
"nii" => "ᓃ",
"nu" => "ᓄ",
"nuu" => "ᓅ",
"na" => "ᓇ",
"naa" => "ᓈ",
"n" => "ᓐ",
"sai" => "ᓭ",
"si" => "ᓯ",
"sii" => "ᓰ",
"su" => "ᓱ",
"suu" => "ᓲ",
"sa" => "ᓴ",
"saa" => "ᓵ",
"s" => "ᔅ",
"lai" => "ᓓ",
"li" => "ᓕ",
"lii" => "ᓖ",
"lu" => "ᓗ",
"luu" => "ᓘ",
"la" => "ᓚ",
"laa" => "ᓛ",
"l" => "ᓪ",
"jai" => "ᔦ",
"ji" => "ᔨ",
"jii" => "ᔩ",
"ju" => "ᔪ",
"juu" => "ᔫ",
"ja" => "ᔭ",
"jaa" => "ᔮ",
"j" => "ᔾ",
"jjai" => "ᑦᔦ",
"jji" => "ᑦᔨ",
"jjii" => "ᑦᔩ",
"jju" => "ᑦᔪ",
"jjuu" => "ᑦᔫ",
"jja" => "ᑦᔭ",
"jjaa" => "ᑦᔮ",
"jj" => "ᑦᔾ",
"vai" => "ᕓ",
"vi" => "ᕕ",
"vii" => "ᕖ",
"vu" => "ᕗ",
"vuu" => "ᕘ",
"va" => "ᕙ",
"vaa" => "ᕚ",
"v" => "ᕝ",
"rai" => "ᕂ",
"ri" => "ᕆ",
"rii" => "ᕇ",
"ru" => "ᕈ",
"ruu" => "ᕉ",
"ra" => "ᕋ",
"raa" => "ᕌ",
"r" => "ᕐ",
"qai" => "ᙯ",
"qi" => "ᕿ",
"qii" => "ᖀ",
"qu" => "ᖁ",
"quu" => "ᖂ",
"qa" => "ᖃ",
"qaa" => "ᖄ",
"q" => "ᖅ",
"qqai" => "ᖅᑫ",
"qqi" => "ᖅᑭ",
"qqii" => "ᖅᑮ",
"qqu" => "ᖅᑯ",
"qquu" => "ᖅᑰ",
"qqa" => "ᖅᑲ",
"qqaa" => "ᖅᑳ",
"qq" => "ᖅᒃ",
"ngai" => "ᙰ",
"ngi" => "ᖏ",
"ngii" => "ᖐ",
"ngu" => "ᖑ",
"nguu" => "ᖒ",
"nga" => "ᖓ",
"ngaa" => "ᖔ",
"ng" => "ᖕ",
"nngi" => "ᙱ",
"nngii" => "ᙲ",
"nngu" => "ᙳ",
"nnguu" => "ᙴ",
"nnga" => "ᙵ",
"nngaa" => "ᙶ",
"nng" => "ᖖ",
"łi" => "ᖠ",
"łii" => "ᖡ",
"łu" => "ᖢ",
"łuu" => "ᖣ",
"ła" => "ᖤ",
"łaa" => "ᖥ",
"ł" => "ᖦ",
"b" => "ᖯ",
"h" => "ᕼ",
"ʼ" => "ᑊ",
};

static WESTERN: PMap = phf_map! {
    "ri" => "ᖨ",
    "r" => "ᖮ"
};


fn get(maps: &[&PMap], key: &str) -> Option<&'static str> {
    for map in maps {
        if let Some(v) = map.get(key) {
            return Some(v)
        }
    }
    None
}

pub fn f(maps: &[&PMap], text: &str, mut buffer: String) -> String {
    if text.is_empty() { return buffer }

    let max_key_length = cmp::min(MAX_KEY_LENGTH, text.len());
    for length in (1..(max_key_length+1)).rev() {
        if let Some(v) = get(maps, &text[..length]) {
            buffer.push_str(&v);
            return f(maps, &text[length..], buffer);
        }
    }
    buffer.push_str(&text[..1]);
    return f(maps, &text[1..], buffer);
}



fn main() {
    let maps = [&BASE, &WESTERN];
    let string = String::new();
    println!("{:?}", f(&maps[..], "inuktitut! iqaluit", string));
}
