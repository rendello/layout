// =======================================================================
//! Automatically generated using `task generate-utf8-case-data`.
//!
//! Unicode characters that behave oddly when the case is changed, for use
//! with property tests.
// =======================================================================

pub const LOWERCASING_CONTRACTS: [&str; 22] = [
    "ẞ",	/* ß	(3->2), -1 bytes */
    "Ω",	/* ω	(3->2), -1 bytes */
    "Å",	/* å	(3->2), -1 bytes */
    "Ɫ",	/* ɫ	(3->2), -1 bytes */
    "Ɽ",	/* ɽ	(3->2), -1 bytes */
    "Ɑ",	/* ɑ	(3->2), -1 bytes */
    "Ɱ",	/* ɱ	(3->2), -1 bytes */
    "Ɐ",	/* ɐ	(3->2), -1 bytes */
    "Ɒ",	/* ɒ	(3->2), -1 bytes */
    "Ȿ",	/* ȿ	(3->2), -1 bytes */
    "Ɀ",	/* ɀ	(3->2), -1 bytes */
    "Ɥ",	/* ɥ	(3->2), -1 bytes */
    "Ɦ",	/* ɦ	(3->2), -1 bytes */
    "Ɜ",	/* ɜ	(3->2), -1 bytes */
    "Ɡ",	/* ɡ	(3->2), -1 bytes */
    "Ɬ",	/* ɬ	(3->2), -1 bytes */
    "Ɪ",	/* ɪ	(3->2), -1 bytes */
    "Ʞ",	/* ʞ	(3->2), -1 bytes */
    "Ʇ",	/* ʇ	(3->2), -1 bytes */
    "Ʝ",	/* ʝ	(3->2), -1 bytes */
    "Ʂ",	/* ʂ	(3->2), -1 bytes */
    "K",	/* k	(3->1), -2 bytes */
];

pub const LOWERCASING_EXPANDS: [&str; 2] = [
    "Ⱥ",	/* ⱥ	(2->3), +1 bytes */
    "Ⱦ",	/* ⱦ	(2->3), +1 bytes */
];

pub const LOWERCASING_EXPANDS_MULTI_CHAR: [&str; 1] = [
    "İ",	/* i̇	(2->3), +1 bytes, +1 chars */
];

pub const UPPERCASING_CONTRACTS: [&str; 13] = [
    "ı",	/* I	(2->1), -1 bytes */
    "ſ",	/* S	(2->1), -1 bytes */
    "ᲀ",	/* В	(3->2), -1 bytes */
    "ᲁ",	/* Д	(3->2), -1 bytes */
    "ᲂ",	/* О	(3->2), -1 bytes */
    "ᲃ",	/* С	(3->2), -1 bytes */
    "ᲄ",	/* Т	(3->2), -1 bytes */
    "ᲅ",	/* Т	(3->2), -1 bytes */
    "ᲆ",	/* Ъ	(3->2), -1 bytes */
    "ᲇ",	/* Ѣ	(3->2), -1 bytes */
    "ι",	/* Ι	(3->2), -1 bytes */
    "ⱥ",	/* Ⱥ	(3->2), -1 bytes */
    "ⱦ",	/* Ⱦ	(3->2), -1 bytes */
];

pub const UPPERCASING_CONTRACTS_MULTI_CHAR: [&str; 5] = [
    "ﬀ",	/* FF	(3->2), -1 bytes, +1 chars */
    "ﬁ",	/* FI	(3->2), -1 bytes, +1 chars */
    "ﬂ",	/* FL	(3->2), -1 bytes, +1 chars */
    "ﬅ",	/* ST	(3->2), -1 bytes, +1 chars */
    "ﬆ",	/* ST	(3->2), -1 bytes, +1 chars */
];

pub const UPPERCASING_EXPANDS: [&str; 18] = [
    "ȿ",	/* Ȿ	(2->3), +1 bytes */
    "ɀ",	/* Ɀ	(2->3), +1 bytes */
    "ɐ",	/* Ɐ	(2->3), +1 bytes */
    "ɑ",	/* Ɑ	(2->3), +1 bytes */
    "ɒ",	/* Ɒ	(2->3), +1 bytes */
    "ɜ",	/* Ɜ	(2->3), +1 bytes */
    "ɡ",	/* Ɡ	(2->3), +1 bytes */
    "ɥ",	/* Ɥ	(2->3), +1 bytes */
    "ɦ",	/* Ɦ	(2->3), +1 bytes */
    "ɪ",	/* Ɪ	(2->3), +1 bytes */
    "ɫ",	/* Ɫ	(2->3), +1 bytes */
    "ɬ",	/* Ɬ	(2->3), +1 bytes */
    "ɱ",	/* Ɱ	(2->3), +1 bytes */
    "ɽ",	/* Ɽ	(2->3), +1 bytes */
    "ʂ",	/* Ʂ	(2->3), +1 bytes */
    "ʇ",	/* Ʇ	(2->3), +1 bytes */
    "ʝ",	/* Ʝ	(2->3), +1 bytes */
    "ʞ",	/* Ʞ	(2->3), +1 bytes */
];

pub const UPPERCASING_EXPANDS_MULTI_CHAR: [&str; 89] = [
    "ΐ",	/* Ϊ́	(2->6), +4 bytes, +2 chars */
    "ΰ",	/* Ϋ́	(2->6), +4 bytes, +2 chars */
    "ὒ",	/* Υ̓̀	(3->6), +3 bytes, +2 chars */
    "ὔ",	/* Υ̓́	(3->6), +3 bytes, +2 chars */
    "ὖ",	/* Υ̓͂	(3->6), +3 bytes, +2 chars */
    "ᾷ",	/* Α͂Ι	(3->6), +3 bytes, +2 chars */
    "ῇ",	/* Η͂Ι	(3->6), +3 bytes, +2 chars */
    "ῒ",	/* Ϊ̀	(3->6), +3 bytes, +2 chars */
    "ΐ",	/* Ϊ́	(3->6), +3 bytes, +2 chars */
    "ῗ",	/* Ϊ͂	(3->6), +3 bytes, +2 chars */
    "ῢ",	/* Ϋ̀	(3->6), +3 bytes, +2 chars */
    "ΰ",	/* Ϋ́	(3->6), +3 bytes, +2 chars */
    "ῧ",	/* Ϋ͂	(3->6), +3 bytes, +2 chars */
    "ῷ",	/* Ω͂Ι	(3->6), +3 bytes, +2 chars */
    "և",	/* ԵՒ	(2->4), +2 bytes, +1 chars */
    "ᾀ",	/* ἈΙ	(3->5), +2 bytes, +1 chars */
    "ᾁ",	/* ἉΙ	(3->5), +2 bytes, +1 chars */
    "ᾂ",	/* ἊΙ	(3->5), +2 bytes, +1 chars */
    "ᾃ",	/* ἋΙ	(3->5), +2 bytes, +1 chars */
    "ᾄ",	/* ἌΙ	(3->5), +2 bytes, +1 chars */
    "ᾅ",	/* ἍΙ	(3->5), +2 bytes, +1 chars */
    "ᾆ",	/* ἎΙ	(3->5), +2 bytes, +1 chars */
    "ᾇ",	/* ἏΙ	(3->5), +2 bytes, +1 chars */
    "ᾈ",	/* ἈΙ	(3->5), +2 bytes, +1 chars */
    "ᾉ",	/* ἉΙ	(3->5), +2 bytes, +1 chars */
    "ᾊ",	/* ἊΙ	(3->5), +2 bytes, +1 chars */
    "ᾋ",	/* ἋΙ	(3->5), +2 bytes, +1 chars */
    "ᾌ",	/* ἌΙ	(3->5), +2 bytes, +1 chars */
    "ᾍ",	/* ἍΙ	(3->5), +2 bytes, +1 chars */
    "ᾎ",	/* ἎΙ	(3->5), +2 bytes, +1 chars */
    "ᾏ",	/* ἏΙ	(3->5), +2 bytes, +1 chars */
    "ᾐ",	/* ἨΙ	(3->5), +2 bytes, +1 chars */
    "ᾑ",	/* ἩΙ	(3->5), +2 bytes, +1 chars */
    "ᾒ",	/* ἪΙ	(3->5), +2 bytes, +1 chars */
    "ᾓ",	/* ἫΙ	(3->5), +2 bytes, +1 chars */
    "ᾔ",	/* ἬΙ	(3->5), +2 bytes, +1 chars */
    "ᾕ",	/* ἭΙ	(3->5), +2 bytes, +1 chars */
    "ᾖ",	/* ἮΙ	(3->5), +2 bytes, +1 chars */
    "ᾗ",	/* ἯΙ	(3->5), +2 bytes, +1 chars */
    "ᾘ",	/* ἨΙ	(3->5), +2 bytes, +1 chars */
    "ᾙ",	/* ἩΙ	(3->5), +2 bytes, +1 chars */
    "ᾚ",	/* ἪΙ	(3->5), +2 bytes, +1 chars */
    "ᾛ",	/* ἫΙ	(3->5), +2 bytes, +1 chars */
    "ᾜ",	/* ἬΙ	(3->5), +2 bytes, +1 chars */
    "ᾝ",	/* ἭΙ	(3->5), +2 bytes, +1 chars */
    "ᾞ",	/* ἮΙ	(3->5), +2 bytes, +1 chars */
    "ᾟ",	/* ἯΙ	(3->5), +2 bytes, +1 chars */
    "ᾠ",	/* ὨΙ	(3->5), +2 bytes, +1 chars */
    "ᾡ",	/* ὩΙ	(3->5), +2 bytes, +1 chars */
    "ᾢ",	/* ὪΙ	(3->5), +2 bytes, +1 chars */
    "ᾣ",	/* ὫΙ	(3->5), +2 bytes, +1 chars */
    "ᾤ",	/* ὬΙ	(3->5), +2 bytes, +1 chars */
    "ᾥ",	/* ὭΙ	(3->5), +2 bytes, +1 chars */
    "ᾦ",	/* ὮΙ	(3->5), +2 bytes, +1 chars */
    "ᾧ",	/* ὯΙ	(3->5), +2 bytes, +1 chars */
    "ᾨ",	/* ὨΙ	(3->5), +2 bytes, +1 chars */
    "ᾩ",	/* ὩΙ	(3->5), +2 bytes, +1 chars */
    "ᾪ",	/* ὪΙ	(3->5), +2 bytes, +1 chars */
    "ᾫ",	/* ὫΙ	(3->5), +2 bytes, +1 chars */
    "ᾬ",	/* ὬΙ	(3->5), +2 bytes, +1 chars */
    "ᾭ",	/* ὭΙ	(3->5), +2 bytes, +1 chars */
    "ᾮ",	/* ὮΙ	(3->5), +2 bytes, +1 chars */
    "ᾯ",	/* ὯΙ	(3->5), +2 bytes, +1 chars */
    "ᾲ",	/* ᾺΙ	(3->5), +2 bytes, +1 chars */
    "ῂ",	/* ῊΙ	(3->5), +2 bytes, +1 chars */
    "ῲ",	/* ῺΙ	(3->5), +2 bytes, +1 chars */
    "ŉ",	/* ʼN	(2->3), +1 bytes, +1 chars */
    "ǰ",	/* J̌	(2->3), +1 bytes, +1 chars */
    "ὐ",	/* Υ̓	(3->4), +1 bytes, +1 chars */
    "ᾳ",	/* ΑΙ	(3->4), +1 bytes, +1 chars */
    "ᾴ",	/* ΆΙ	(3->4), +1 bytes, +1 chars */
    "ᾶ",	/* Α͂	(3->4), +1 bytes, +1 chars */
    "ᾼ",	/* ΑΙ	(3->4), +1 bytes, +1 chars */
    "ῃ",	/* ΗΙ	(3->4), +1 bytes, +1 chars */
    "ῄ",	/* ΉΙ	(3->4), +1 bytes, +1 chars */
    "ῆ",	/* Η͂	(3->4), +1 bytes, +1 chars */
    "ῌ",	/* ΗΙ	(3->4), +1 bytes, +1 chars */
    "ῖ",	/* Ι͂	(3->4), +1 bytes, +1 chars */
    "ῤ",	/* Ρ̓	(3->4), +1 bytes, +1 chars */
    "ῦ",	/* Υ͂	(3->4), +1 bytes, +1 chars */
    "ῳ",	/* ΩΙ	(3->4), +1 bytes, +1 chars */
    "ῴ",	/* ΏΙ	(3->4), +1 bytes, +1 chars */
    "ῶ",	/* Ω͂	(3->4), +1 bytes, +1 chars */
    "ῼ",	/* ΩΙ	(3->4), +1 bytes, +1 chars */
    "ﬓ",	/* ՄՆ	(3->4), +1 bytes, +1 chars */
    "ﬔ",	/* ՄԵ	(3->4), +1 bytes, +1 chars */
    "ﬕ",	/* ՄԻ	(3->4), +1 bytes, +1 chars */
    "ﬖ",	/* ՎՆ	(3->4), +1 bytes, +1 chars */
    "ﬗ",	/* ՄԽ	(3->4), +1 bytes, +1 chars */
];