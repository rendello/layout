
use std::cmp;

use phf::phf_map;


type PMap = phf::Map<&'static [u8], &'static str>;

const MAX_KEY_LENGTH: usize = 5;


pub static BASE: PMap = phf_map! {
    b"ai" => "ᐁ",
    b"i" => "ᐃ",
    b"ii" => "ᐄ",
    b"u" => "ᐅ",
    b"uu" => "ᐆ",
    b"a" => "ᐊ",
    b"aa" => "ᐋ",
    b"p" => "ᑉ",
    b"pai" => "ᐯ",
    b"pi" => "ᐱ",
    b"pii" => "ᐲ",
    b"pu" => "ᐳ",
    b"puu" => "ᐴ",
    b"pa" => "ᐸ",
    b"paa" => "ᐹ",
    b"t" => "ᑦ",
    b"tai" => "ᑌ",
    b"ti" => "ᑎ",
    b"tii" => "ᑏ",
    b"tu" => "ᑐ",
    b"tuu" => "ᑑ",
    b"ta" => "ᑕ",
    b"taa" => "ᑖ",
    b"k" => "ᒃ",
    b"kai" => "ᑫ",
    b"ki" => "ᑭ",
    b"kii" => "ᑮ",
    b"ku" => "ᑯ",
    b"kuu" => "ᑰ",
    b"ka" => "ᑲ",
    b"kaa" => "ᑳ",
    b"g" => "ᒡ",
    b"gai" => "ᒉ",
    b"gi" => "ᒋ",
    b"gii" => "ᒌ",
    b"gu" => "ᒍ",
    b"guu" => "ᒎ",
    b"ga" => "ᒐ",
    b"gaa" => "ᒑ",
    b"m" => "ᒻ",
    b"mai" => "ᒣ",
    b"mi" => "ᒥ",
    b"mii" => "ᒦ",
    b"mu" => "ᒧ",
    b"muu" => "ᒨ",
    b"ma" => "ᒪ",
    b"maa" => "ᒫ",
    b"n" => "ᓐ",
    b"nai" => "ᓀ",
    b"ni" => "ᓂ",
    b"nii" => "ᓃ",
    b"nu" => "ᓄ",
    b"nuu" => "ᓅ",
    b"na" => "ᓇ",
    b"naa" => "ᓈ",
    b"s" => "ᔅ",
    b"sai" => "ᓭ",
    b"si" => "ᓯ",
    b"sii" => "ᓰ",
    b"su" => "ᓱ",
    b"suu" => "ᓲ",
    b"sa" => "ᓴ",
    b"saa" => "ᓵ",
    b"l" => "ᓪ",
    b"lai" => "ᓓ",
    b"li" => "ᓕ",
    b"lii" => "ᓖ",
    b"lu" => "ᓗ",
    b"luu" => "ᓘ",
    b"la" => "ᓚ",
    b"laa" => "ᓛ",
    b"j" => "ᔾ",
    b"jai" => "ᔦ",
    b"ji" => "ᔨ",
    b"jii" => "ᔩ",
    b"ju" => "ᔪ",
    b"juu" => "ᔫ",
    b"ja" => "ᔭ",
    b"jaa" => "ᔮ",
    b"jj" => "ᑦᔾ",
    b"jjai" => "ᑦᔦ",
    b"jji" => "ᑦᔨ",
    b"jjii" => "ᑦᔩ",
    b"jju" => "ᑦᔪ",
    b"jjuu" => "ᑦᔫ",
    b"jja" => "ᑦᔭ",
    b"jjaa" => "ᑦᔮ",
    b"v" => "ᕝ",
    b"vai" => "ᕓ",
    b"vi" => "ᕕ",
    b"vii" => "ᕖ",
    b"vu" => "ᕗ",
    b"vuu" => "ᕘ",
    b"va" => "ᕙ",
    b"vaa" => "ᕚ",
    b"r" => "ᕐ",
    b"rai" => "ᕂ",
    b"ri" => "ᕆ",
    b"rii" => "ᕇ",
    b"ru" => "ᕈ",
    b"ruu" => "ᕉ",
    b"ra" => "ᕋ",
    b"raa" => "ᕌ",
    b"q" => "ᖅ",
    b"qai" => "ᙯ",
    b"qi" => "ᕿ",
    b"qii" => "ᖀ",
    b"qu" => "ᖁ",
    b"quu" => "ᖂ",
    b"qa" => "ᖃ",
    b"qaa" => "ᖄ",
    b"qq" => "ᖅᒃ",
    b"qqai" => "ᖅᑫ",
    b"qqi" => "ᖅᑭ",
    b"qqii" => "ᖅᑮ",
    b"qqu" => "ᖅᑯ",
    b"qquu" => "ᖅᑰ",
    b"qqa" => "ᖅᑲ",
    b"qqaa" => "ᖅᑳ",
    b"ng" => "ᖕ",
    b"ngai" => "ᙰ",
    b"ngi" => "ᖏ",
    b"ngii" => "ᖐ",
    b"ngu" => "ᖑ",
    b"nguu" => "ᖒ",
    b"nga" => "ᖓ",
    b"ngaa" => "ᖔ",
    b"nng" => "ᖖ",
    b"nngi" => "ᙱ",
    b"nngii" => "ᙲ",
    b"nngu" => "ᙳ",
    b"nnguu" => "ᙴ",
    b"nnga" => "ᙵ",
    b"nngaa" => "ᙶ",
    b"\xc5\x82" => "ᖦ",    /* ł */
    b"\xc5\x82i" => "ᖠ",   /* łi */
    b"\xc5\x82ii" => "ᖡ",  /* łii */
    b"\xc5\x82u" => "ᖢ",   /* łu */
    b"\xc5\x82uu" => "ᖣ",  /* łuu */
    b"\xc5\x82a" => "ᖤ",   /* ła */
    b"\xc5\x82aa" => "ᖥ",  /* łaa */
    b"b" => "ᖯ",
    b"h" => "ᕼ",
    b"\xca\xbc" => "ᑊ",     /* ʼ */
};

pub static WESTERN: PMap = phf_map! {
    b"ri" => "ᖨ",
    b"r" => "ᖮ"
};


fn get(maps: &[&PMap], key: &[u8]) -> Option<&'static str> {
    for map in maps {
        if let Some(v) = map.get(key) {
            return Some(v)
        }
    }
    None
}

pub fn f(maps: &[&PMap], text: &[u8], mut buffer: String) -> String {
    if text.is_empty() { return buffer }

    let max_key_length = cmp::min(MAX_KEY_LENGTH, text.len());
    for length in (1..(max_key_length+1)).rev() {
        if let Some(v) = get(maps, &text[..length]) {
            buffer.push_str(&v);
            return f(maps, &text[length..], buffer);
        }
    }
    let jump = next_jump(text[0]);
    buffer.push_str(std::str::from_utf8(&text[..jump]).unwrap());
    return f(maps, &text[jump..], buffer);
}


fn next_jump(byte: u8) -> usize {
    if byte & 0b1000_0000 == 0 {
        1
    } else if 0b1110_0000 & byte == 0b1100_0000 {
        2
    } else if 0b1111_0000 & byte == 0b1110_0000 {
        3
    } else if 0b1111_1000 & byte == 0b1111_0000 {
        4
    } else {
        unreachable!();
    }
}