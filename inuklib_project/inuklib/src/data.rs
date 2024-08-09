// ====================================================================
//! Automatically generated data file; see `../../scripts/generate.py`.
//  Generated on on August 04 at 11:20 EST.
// ====================================================================

use phf_macros::{phf_map, phf_set};
use crate::syllabic_unit::Dialect::{Nunavut, Nattilik, Aivilik, Nunavik};
use crate::syllabic_unit::SyllabicUnitRepresentation::{Latin, Syllabic};
use crate::syllabic_unit::AiRepresentation::{Split, Classic, Ring, NotApplicable};
use crate::syllabic_unit::*;

pub static LATIN_MAP: SyllabicUnitMap = phf_map! {
    "ai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "ai",
            consonant: None
        })
    },
    "i" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "i",
            consonant: None
        })
    },
    "ii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "ii",
            consonant: None
        })
    },
    "u" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "u",
            consonant: None
        })
    },
    "uu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "uu",
            consonant: None
        })
    },
    "a" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "a",
            consonant: None
        })
    },
    "aa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "aa",
            consonant: None
        })
    },
    "pai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "pai",
            consonant: Some("p")
        })
    },
    "pi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "pi",
            consonant: Some("p")
        })
    },
    "pii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "pii",
            consonant: Some("p")
        })
    },
    "pu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "pu",
            consonant: Some("p")
        })
    },
    "puu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "puu",
            consonant: Some("p")
        })
    },
    "pa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "pa",
            consonant: Some("p")
        })
    },
    "paa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "paa",
            consonant: Some("p")
        })
    },
    "p" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "p",
            consonant: Some("p")
        })
    },
    "tai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "tai",
            consonant: Some("t")
        })
    },
    "ti" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ti",
            consonant: Some("t")
        })
    },
    "tii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "tii",
            consonant: Some("t")
        })
    },
    "tu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "tu",
            consonant: Some("t")
        })
    },
    "tuu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "tuu",
            consonant: Some("t")
        })
    },
    "ta" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ta",
            consonant: Some("t")
        })
    },
    "taa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "taa",
            consonant: Some("t")
        })
    },
    "t" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "t",
            consonant: Some("t")
        })
    },
    "kai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "kai",
            consonant: Some("k")
        })
    },
    "ki" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ki",
            consonant: Some("k")
        })
    },
    "kii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "kii",
            consonant: Some("k")
        })
    },
    "ku" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "ku",
            consonant: Some("k")
        })
    },
    "kuu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "kuu",
            consonant: Some("k")
        })
    },
    "ka" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ka",
            consonant: Some("k")
        })
    },
    "kaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "kaa",
            consonant: Some("k")
        })
    },
    "k" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "k",
            consonant: Some("k")
        })
    },
    "gai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "gai",
            consonant: Some("g")
        })
    },
    "gi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "gi",
            consonant: Some("g")
        })
    },
    "gii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "gii",
            consonant: Some("g")
        })
    },
    "gu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "gu",
            consonant: Some("g")
        })
    },
    "guu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "guu",
            consonant: Some("g")
        })
    },
    "ga" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ga",
            consonant: Some("g")
        })
    },
    "gaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "gaa",
            consonant: Some("g")
        })
    },
    "g" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "g",
            consonant: Some("g")
        })
    },
    "mai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "mai",
            consonant: Some("m")
        })
    },
    "mi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "mi",
            consonant: Some("m")
        })
    },
    "mii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "mii",
            consonant: Some("m")
        })
    },
    "mu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "mu",
            consonant: Some("m")
        })
    },
    "muu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "muu",
            consonant: Some("m")
        })
    },
    "ma" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ma",
            consonant: Some("m")
        })
    },
    "maa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "maa",
            consonant: Some("m")
        })
    },
    "m" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "m",
            consonant: Some("m")
        })
    },
    "nai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "nai",
            consonant: Some("n")
        })
    },
    "ni" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ni",
            consonant: Some("n")
        })
    },
    "nii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "nii",
            consonant: Some("n")
        })
    },
    "nu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "nu",
            consonant: Some("n")
        })
    },
    "nuu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "nuu",
            consonant: Some("n")
        })
    },
    "na" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "na",
            consonant: Some("n")
        })
    },
    "naa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "naa",
            consonant: Some("n")
        })
    },
    "n" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "n",
            consonant: Some("n")
        })
    },
    "sai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "sai",
            consonant: Some("s")
        })
    },
    "si" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "si",
            consonant: Some("s")
        })
    },
    "sii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "sii",
            consonant: Some("s")
        })
    },
    "su" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "su",
            consonant: Some("s")
        })
    },
    "suu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "suu",
            consonant: Some("s")
        })
    },
    "sa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "sa",
            consonant: Some("s")
        })
    },
    "saa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "saa",
            consonant: Some("s")
        })
    },
    "s" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "s",
            consonant: Some("s")
        })
    },
    "lai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "lai",
            consonant: Some("l")
        })
    },
    "li" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "li",
            consonant: Some("l")
        })
    },
    "lii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "lii",
            consonant: Some("l")
        })
    },
    "lu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "lu",
            consonant: Some("l")
        })
    },
    "luu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "luu",
            consonant: Some("l")
        })
    },
    "la" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "la",
            consonant: Some("l")
        })
    },
    "laa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "laa",
            consonant: Some("l")
        })
    },
    "l" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "l",
            consonant: Some("l")
        })
    },
    "jai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "jai",
            consonant: Some("j")
        })
    },
    "ji" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ji",
            consonant: Some("j")
        })
    },
    "jii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "jii",
            consonant: Some("j")
        })
    },
    "ju" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "ju",
            consonant: Some("j")
        })
    },
    "juu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "juu",
            consonant: Some("j")
        })
    },
    "ja" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ja",
            consonant: Some("j")
        })
    },
    "jaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "jaa",
            consonant: Some("j")
        })
    },
    "j" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "j",
            consonant: Some("j")
        })
    },
    "jjai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "jjai",
            consonant: Some("jj")
        })
    },
    "jji" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "jji",
            consonant: Some("jj")
        })
    },
    "jjii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "jjii",
            consonant: Some("jj")
        })
    },
    "jju" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "jju",
            consonant: Some("jj")
        })
    },
    "jjuu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "jjuu",
            consonant: Some("jj")
        })
    },
    "jja" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "jja",
            consonant: Some("jj")
        })
    },
    "jjaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "jjaa",
            consonant: Some("jj")
        })
    },
    "jj" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "jj",
            consonant: Some("jj")
        })
    },
    "vai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "vai",
            consonant: Some("v")
        })
    },
    "vi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "vi",
            consonant: Some("v")
        })
    },
    "vii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "vii",
            consonant: Some("v")
        })
    },
    "vu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "vu",
            consonant: Some("v")
        })
    },
    "vuu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "vuu",
            consonant: Some("v")
        })
    },
    "va" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "va",
            consonant: Some("v")
        })
    },
    "vaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "vaa",
            consonant: Some("v")
        })
    },
    "v" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "v",
            consonant: Some("v")
        })
    },
    "rai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "rai",
            consonant: Some("r")
        })
    },
    "ri" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ri",
            consonant: Some("r")
        })
    },
    "rii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "rii",
            consonant: Some("r")
        })
    },
    "ru" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "ru",
            consonant: Some("r")
        })
    },
    "ruu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "ruu",
            consonant: Some("r")
        })
    },
    "ra" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ra",
            consonant: Some("r")
        })
    },
    "raa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "raa",
            consonant: Some("r")
        })
    },
    "r" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "r",
            consonant: Some("r")
        })
    },
    "qai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "qai",
            consonant: Some("q")
        })
    },
    "qi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "qi",
            consonant: Some("q")
        })
    },
    "qii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "qii",
            consonant: Some("q")
        })
    },
    "qu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "qu",
            consonant: Some("q")
        })
    },
    "quu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "quu",
            consonant: Some("q")
        })
    },
    "qa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "qa",
            consonant: Some("q")
        })
    },
    "qaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "qaa",
            consonant: Some("q")
        })
    },
    "q" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "q",
            consonant: Some("q")
        })
    },
    "ngai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "ngai",
            consonant: Some("ng")
        })
    },
    "ngi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ngi",
            consonant: Some("ng")
        })
    },
    "ngii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "ngii",
            consonant: Some("ng")
        })
    },
    "ngu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "ngu",
            consonant: Some("ng")
        })
    },
    "nguu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "nguu",
            consonant: Some("ng")
        })
    },
    "nga" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "nga",
            consonant: Some("ng")
        })
    },
    "ngaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "ngaa",
            consonant: Some("ng")
        })
    },
    "ng" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "ng",
            consonant: Some("ng")
        })
    },
    "nngai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "nngai",
            consonant: Some("nng")
        })
    },
    "nngi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "nngi",
            consonant: Some("nng")
        })
    },
    "nngii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "nngii",
            consonant: Some("nng")
        })
    },
    "nngu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "nngu",
            consonant: Some("nng")
        })
    },
    "nnguu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "nnguu",
            consonant: Some("nng")
        })
    },
    "nnga" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "nnga",
            consonant: Some("nng")
        })
    },
    "nngaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "nngaa",
            consonant: Some("nng")
        })
    },
    "nng" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "nng",
            consonant: Some("nng")
        })
    },
    "ʼ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ʼ"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "ʼ",
            consonant: Some("ʼ")
        })
    },
    "qqai" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqai",
            consonant: Some("qq")
        })
    },
    "qqi" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqi",
            consonant: Some("qq")
        })
    },
    "qqii" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqii",
            consonant: Some("qq")
        })
    },
    "qqu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqu",
            consonant: Some("qq")
        })
    },
    "qquu" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "qquu",
            consonant: Some("qq")
        })
    },
    "qqa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqa",
            consonant: Some("qq")
        })
    },
    "qqaa" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "qqaa",
            consonant: Some("qq")
        })
    },
    "qq" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "qq",
            consonant: Some("qq")
        })
    },
    "h" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("h"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "h",
            consonant: Some("h")
        })
    },
    "řai" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "řai",
            consonant: Some("ř")
        })
    },
    "ři" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ři",
            consonant: Some("ř")
        })
    },
    "řii" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "řii",
            consonant: Some("ř")
        })
    },
    "řu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "řu",
            consonant: Some("ř")
        })
    },
    "řuu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "řuu",
            consonant: Some("ř")
        })
    },
    "řa" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "řa",
            consonant: Some("ř")
        })
    },
    "řaa" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "řaa",
            consonant: Some("ř")
        })
    },
    "ř" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "ř",
            consonant: Some("ř")
        })
    },
    "łai" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "łai",
            consonant: Some("ł")
        })
    },
    "łi" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "łi",
            consonant: Some("ł")
        })
    },
    "łii" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "łii",
            consonant: Some("ł")
        })
    },
    "łu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "łu",
            consonant: Some("ł")
        })
    },
    "łuu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "łuu",
            consonant: Some("ł")
        })
    },
    "ła" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ła",
            consonant: Some("ł")
        })
    },
    "łaa" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "łaa",
            consonant: Some("ł")
        })
    },
    "ł" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "ł",
            consonant: Some("ł")
        })
    },
    "šai" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "šai",
            consonant: Some("š")
        })
    },
    "ši" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "ši",
            consonant: Some("š")
        })
    },
    "šii" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "šii",
            consonant: Some("š")
        })
    },
    "šu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "šu",
            consonant: Some("š")
        })
    },
    "šuu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "šuu",
            consonant: Some("š")
        })
    },
    "ša" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ša",
            consonant: Some("š")
        })
    },
    "šaa" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "šaa",
            consonant: Some("š")
        })
    },
    "chai" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "chai",
            consonant: Some("ch")
        })
    },
    "chi" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "chi",
            consonant: Some("ch")
        })
    },
    "chii" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "chii",
            consonant: Some("ch")
        })
    },
    "chu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "chu",
            consonant: Some("ch")
        })
    },
    "chuu" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "chuu",
            consonant: Some("ch")
        })
    },
    "cha" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "cha",
            consonant: Some("ch")
        })
    },
    "chaa" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "chaa",
            consonant: Some("ch")
        })
    },
    "ch" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "ch",
            consonant: Some("ch")
        })
    },
    "b" => &SyllabicUnit {
        dialects: enum_set!(Aivilik),
        consonant: Some("b"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "b",
            consonant: Some("b")
        })
    },
    "hai" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("ai"),
        original: Latin(LatinSyllabicUnit {
            representation: "hai",
            consonant: Some("h")
        })
    },
    "hi" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("i"),
        original: Latin(LatinSyllabicUnit {
            representation: "hi",
            consonant: Some("h")
        })
    },
    "hii" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("ii"),
        original: Latin(LatinSyllabicUnit {
            representation: "hii",
            consonant: Some("h")
        })
    },
    "hu" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("u"),
        original: Latin(LatinSyllabicUnit {
            representation: "hu",
            consonant: Some("h")
        })
    },
    "huu" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("uu"),
        original: Latin(LatinSyllabicUnit {
            representation: "huu",
            consonant: Some("h")
        })
    },
    "ha" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("a"),
        original: Latin(LatinSyllabicUnit {
            representation: "ha",
            consonant: Some("h")
        })
    },
    "haa" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("aa"),
        original: Latin(LatinSyllabicUnit {
            representation: "haa",
            consonant: Some("h")
        })
    },
    "š" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: None,
        original: Latin(LatinSyllabicUnit {
            representation: "š",
            consonant: Some("š")
        })
    }
};

pub static SYLLABIC_MAP: SyllabicUnitMap = phf_map! {
    "ᐂ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐂ",
            ai_representation: Ring
        })
    },
    "ᐁ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐁ",
            ai_representation: Classic
        })
    },
    "ᐊᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐊᐃ",
            ai_representation: Split
        })
    },
    "ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐃ",
            ai_representation: NotApplicable
        })
    },
    "ᐄ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐄ",
            ai_representation: NotApplicable
        })
    },
    "ᐅ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐅ",
            ai_representation: NotApplicable
        })
    },
    "ᐆ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐆ",
            ai_representation: NotApplicable
        })
    },
    "ᐊ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐊ",
            ai_representation: NotApplicable
        })
    },
    "ᐋ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: None,
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐋ",
            ai_representation: NotApplicable
        })
    },
    "ᐰ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐰ",
            ai_representation: Ring
        })
    },
    "ᐯ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐯ",
            ai_representation: Classic
        })
    },
    "ᐸᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐸᐃ",
            ai_representation: Split
        })
    },
    "ᐱ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐱ",
            ai_representation: NotApplicable
        })
    },
    "ᐲ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐲ",
            ai_representation: NotApplicable
        })
    },
    "ᐳ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐳ",
            ai_representation: NotApplicable
        })
    },
    "ᐴ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐴ",
            ai_representation: NotApplicable
        })
    },
    "ᐸ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐸ",
            ai_representation: NotApplicable
        })
    },
    "ᐹ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᐹ",
            ai_representation: NotApplicable
        })
    },
    "ᑉ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("p"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑉ",
            ai_representation: NotApplicable
        })
    },
    "ᑍ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑍ",
            ai_representation: Ring
        })
    },
    "ᑌ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑌ",
            ai_representation: Classic
        })
    },
    "ᑕᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑕᐃ",
            ai_representation: Split
        })
    },
    "ᑎ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑎ",
            ai_representation: NotApplicable
        })
    },
    "ᑏ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑏ",
            ai_representation: NotApplicable
        })
    },
    "ᑐ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑐ",
            ai_representation: NotApplicable
        })
    },
    "ᑑ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑑ",
            ai_representation: NotApplicable
        })
    },
    "ᑕ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑕ",
            ai_representation: NotApplicable
        })
    },
    "ᑖ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑖ",
            ai_representation: NotApplicable
        })
    },
    "ᑦ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("t"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦ",
            ai_representation: NotApplicable
        })
    },
    "ᑬ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑬ",
            ai_representation: Ring
        })
    },
    "ᑫ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑫ",
            ai_representation: Classic
        })
    },
    "ᑲᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑲᐃ",
            ai_representation: Split
        })
    },
    "ᑭ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑭ",
            ai_representation: NotApplicable
        })
    },
    "ᑮ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑮ",
            ai_representation: NotApplicable
        })
    },
    "ᑯ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑯ",
            ai_representation: NotApplicable
        })
    },
    "ᑰ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑰ",
            ai_representation: NotApplicable
        })
    },
    "ᑲ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑲ",
            ai_representation: NotApplicable
        })
    },
    "ᑳ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑳ",
            ai_representation: NotApplicable
        })
    },
    "ᒃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("k"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒃ",
            ai_representation: NotApplicable
        })
    },
    "ᒊ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒊ",
            ai_representation: Ring
        })
    },
    "ᒉ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒉ",
            ai_representation: Classic
        })
    },
    "ᒐᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒐᐃ",
            ai_representation: Split
        })
    },
    "ᒋ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒋ",
            ai_representation: NotApplicable
        })
    },
    "ᒌ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒌ",
            ai_representation: NotApplicable
        })
    },
    "ᒍ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒍ",
            ai_representation: NotApplicable
        })
    },
    "ᒎ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒎ",
            ai_representation: NotApplicable
        })
    },
    "ᒐ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒐ",
            ai_representation: NotApplicable
        })
    },
    "ᒑ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒑ",
            ai_representation: NotApplicable
        })
    },
    "ᒡ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("g"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒡ",
            ai_representation: NotApplicable
        })
    },
    "ᒤ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒤ",
            ai_representation: Ring
        })
    },
    "ᒣ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒣ",
            ai_representation: Classic
        })
    },
    "ᒪᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒪᐃ",
            ai_representation: Split
        })
    },
    "ᒥ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒥ",
            ai_representation: NotApplicable
        })
    },
    "ᒦ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒦ",
            ai_representation: NotApplicable
        })
    },
    "ᒧ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒧ",
            ai_representation: NotApplicable
        })
    },
    "ᒨ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒨ",
            ai_representation: NotApplicable
        })
    },
    "ᒪ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒪ",
            ai_representation: NotApplicable
        })
    },
    "ᒫ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒫ",
            ai_representation: NotApplicable
        })
    },
    "ᒻ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("m"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᒻ",
            ai_representation: NotApplicable
        })
    },
    "ᓁ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓁ",
            ai_representation: Ring
        })
    },
    "ᓀ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓀ",
            ai_representation: Classic
        })
    },
    "ᓇᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓇᐃ",
            ai_representation: Split
        })
    },
    "ᓂ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓂ",
            ai_representation: NotApplicable
        })
    },
    "ᓃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓃ",
            ai_representation: NotApplicable
        })
    },
    "ᓄ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓄ",
            ai_representation: NotApplicable
        })
    },
    "ᓅ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓅ",
            ai_representation: NotApplicable
        })
    },
    "ᓇ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓇ",
            ai_representation: NotApplicable
        })
    },
    "ᓈ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓈ",
            ai_representation: NotApplicable
        })
    },
    "ᓐ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("n"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓐ",
            ai_representation: NotApplicable
        })
    },
    "ᓮ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓮ",
            ai_representation: Ring
        })
    },
    "ᓭ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓭ",
            ai_representation: Classic
        })
    },
    "ᓴᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴᐃ",
            ai_representation: Split
        })
    },
    "ᓯ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓯ",
            ai_representation: NotApplicable
        })
    },
    "ᓰ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓰ",
            ai_representation: NotApplicable
        })
    },
    "ᓱ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓱ",
            ai_representation: NotApplicable
        })
    },
    "ᓲ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓲ",
            ai_representation: NotApplicable
        })
    },
    "ᓴ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ",
            ai_representation: NotApplicable
        })
    },
    "ᓵ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓵ",
            ai_representation: NotApplicable
        })
    },
    "ᔅ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("s"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔅ",
            ai_representation: NotApplicable
        })
    },
    "ᓔ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓔ",
            ai_representation: Ring
        })
    },
    "ᓓ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓓ",
            ai_representation: Classic
        })
    },
    "ᓚᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓚᐃ",
            ai_representation: Split
        })
    },
    "ᓕ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓕ",
            ai_representation: NotApplicable
        })
    },
    "ᓖ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓖ",
            ai_representation: NotApplicable
        })
    },
    "ᓗ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓗ",
            ai_representation: NotApplicable
        })
    },
    "ᓘ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓘ",
            ai_representation: NotApplicable
        })
    },
    "ᓚ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓚ",
            ai_representation: NotApplicable
        })
    },
    "ᓛ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓛ",
            ai_representation: NotApplicable
        })
    },
    "ᓪ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("l"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓪ",
            ai_representation: NotApplicable
        })
    },
    "ᔧ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔧ",
            ai_representation: Ring
        })
    },
    "ᔦ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔦ",
            ai_representation: Classic
        })
    },
    "ᔭᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔭᐃ",
            ai_representation: Split
        })
    },
    "ᔨ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔨ",
            ai_representation: NotApplicable
        })
    },
    "ᔩ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔩ",
            ai_representation: NotApplicable
        })
    },
    "ᔪ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔪ",
            ai_representation: NotApplicable
        })
    },
    "ᔫ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔫ",
            ai_representation: NotApplicable
        })
    },
    "ᔭ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔭ",
            ai_representation: NotApplicable
        })
    },
    "ᔮ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔮ",
            ai_representation: NotApplicable
        })
    },
    "ᔾ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("j"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔾ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔧ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔧ",
            ai_representation: Ring
        })
    },
    "ᑦᔦ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔦ",
            ai_representation: Classic
        })
    },
    "ᑦᔭᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔭᐃ",
            ai_representation: Split
        })
    },
    "ᑦᔨ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔨ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔩ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔩ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔪ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔪ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔫ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔫ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔭ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔭ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔮ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔮ",
            ai_representation: NotApplicable
        })
    },
    "ᑦᔾ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("jj"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔾ",
            ai_representation: NotApplicable
        })
    },
    "ᕔ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕔ",
            ai_representation: Ring
        })
    },
    "ᕓ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕓ",
            ai_representation: Classic
        })
    },
    "ᕙᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕙᐃ",
            ai_representation: Split
        })
    },
    "ᕕ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕕ",
            ai_representation: NotApplicable
        })
    },
    "ᕖ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕖ",
            ai_representation: NotApplicable
        })
    },
    "ᕗ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕗ",
            ai_representation: NotApplicable
        })
    },
    "ᕘ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕘ",
            ai_representation: NotApplicable
        })
    },
    "ᕙ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕙ",
            ai_representation: NotApplicable
        })
    },
    "ᕚ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕚ",
            ai_representation: NotApplicable
        })
    },
    "ᕝ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("v"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕝ",
            ai_representation: NotApplicable
        })
    },
    "ᕅ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕅ",
            ai_representation: Ring
        })
    },
    "ᕂ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕂ",
            ai_representation: Classic
        })
    },
    "ᕋᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕋᐃ",
            ai_representation: Split
        })
    },
    "ᕆ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕆ",
            ai_representation: NotApplicable
        })
    },
    "ᕇ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕇ",
            ai_representation: NotApplicable
        })
    },
    "ᕈ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕈ",
            ai_representation: NotApplicable
        })
    },
    "ᕉ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕉ",
            ai_representation: NotApplicable
        })
    },
    "ᕋ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕋ",
            ai_representation: NotApplicable
        })
    },
    "ᕌ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕌ",
            ai_representation: NotApplicable
        })
    },
    "ᕐ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("r"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕐ",
            ai_representation: NotApplicable
        })
    },
    "ᕾ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕾ",
            ai_representation: Ring
        })
    },
    "ᙯ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙯ",
            ai_representation: Classic
        })
    },
    "ᖃᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖃᐃ",
            ai_representation: Split
        })
    },
    "ᕿ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕿ",
            ai_representation: NotApplicable
        })
    },
    "ᖀ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖀ",
            ai_representation: NotApplicable
        })
    },
    "ᖁ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖁ",
            ai_representation: NotApplicable
        })
    },
    "ᖂ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖂ",
            ai_representation: NotApplicable
        })
    },
    "ᖃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖃ",
            ai_representation: NotApplicable
        })
    },
    "ᖄ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖄ",
            ai_representation: NotApplicable
        })
    },
    "ᖅ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("q"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅ",
            ai_representation: NotApplicable
        })
    },
    "ᖎ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖎ",
            ai_representation: Ring
        })
    },
    "ᙰ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙰ",
            ai_representation: Classic
        })
    },
    "ᖓᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖓᐃ",
            ai_representation: Split
        })
    },
    "ᖏ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖏ",
            ai_representation: NotApplicable
        })
    },
    "ᖐ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖐ",
            ai_representation: NotApplicable
        })
    },
    "ᖑ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖑ",
            ai_representation: NotApplicable
        })
    },
    "ᖒ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖒ",
            ai_representation: NotApplicable
        })
    },
    "ᖓ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖓ",
            ai_representation: NotApplicable
        })
    },
    "ᖔ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖔ",
            ai_representation: NotApplicable
        })
    },
    "ᖕ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ng"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖕ",
            ai_representation: NotApplicable
        })
    },
    "ᙵᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙵᐃ",
            ai_representation: Split
        })
    },
    "ᙱ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙱ",
            ai_representation: NotApplicable
        })
    },
    "ᙲ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙲ",
            ai_representation: NotApplicable
        })
    },
    "ᙳ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙳ",
            ai_representation: NotApplicable
        })
    },
    "ᙴ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙴ",
            ai_representation: NotApplicable
        })
    },
    "ᙵ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙵ",
            ai_representation: NotApplicable
        })
    },
    "ᙶ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᙶ",
            ai_representation: NotApplicable
        })
    },
    "ᖖ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("nng"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖖ",
            ai_representation: NotApplicable
        })
    },
    "ᑊ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut | Nattilik | Aivilik | Nunavik),
        consonant: Some("ʼ"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᑊ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑫ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑫ",
            ai_representation: Classic
        })
    },
    "ᖅᑲᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑲᐃ",
            ai_representation: Split
        })
    },
    "ᖅᑭ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑭ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑮ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑮ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑯ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑯ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑰ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑰ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑲ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑲ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᑳ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑳ",
            ai_representation: NotApplicable
        })
    },
    "ᖅᒃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("qq"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᒃ",
            ai_representation: NotApplicable
        })
    },
    "ᕼ" => &SyllabicUnit {
        dialects: enum_set!(Nunavut),
        consonant: Some("h"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕼ",
            ai_representation: NotApplicable
        })
    },
    "ᖬᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖬᐃ",
            ai_representation: Split
        })
    },
    "ᖨ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖨ",
            ai_representation: NotApplicable
        })
    },
    "ᖩ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖩ",
            ai_representation: NotApplicable
        })
    },
    "ᖪ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖪ",
            ai_representation: NotApplicable
        })
    },
    "ᖫ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖫ",
            ai_representation: NotApplicable
        })
    },
    "ᖬ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖬ",
            ai_representation: NotApplicable
        })
    },
    "ᖭ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖭ",
            ai_representation: NotApplicable
        })
    },
    "ᖮ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ř"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖮ",
            ai_representation: NotApplicable
        })
    },
    "ᖤᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖤᐃ",
            ai_representation: Split
        })
    },
    "ᖠ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖠ",
            ai_representation: NotApplicable
        })
    },
    "ᖡ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖡ",
            ai_representation: NotApplicable
        })
    },
    "ᖢ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖢ",
            ai_representation: NotApplicable
        })
    },
    "ᖣ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖣ",
            ai_representation: NotApplicable
        })
    },
    "ᖤ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖤ",
            ai_representation: NotApplicable
        })
    },
    "ᖥ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖥ",
            ai_representation: NotApplicable
        })
    },
    "ᖦ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ł"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖦ",
            ai_representation: NotApplicable
        })
    },
    "𑪺ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪺ᐃ",
            ai_representation: Split
        })
    },
    "𑪶" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪶",
            ai_representation: NotApplicable
        })
    },
    "𑪷" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪷",
            ai_representation: NotApplicable
        })
    },
    "𑪸" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪸",
            ai_representation: NotApplicable
        })
    },
    "𑪹" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪹",
            ai_representation: NotApplicable
        })
    },
    "𑪺" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪺",
            ai_representation: NotApplicable
        })
    },
    "𑪻" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "𑪻",
            ai_representation: NotApplicable
        })
    },
    "ᕦᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕦᐃ",
            ai_representation: Split
        })
    },
    "ᕠ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕠ",
            ai_representation: NotApplicable
        })
    },
    "ᕢ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕢ",
            ai_representation: NotApplicable
        })
    },
    "ᕤ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕤ",
            ai_representation: NotApplicable
        })
    },
    "ᕥ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕥ",
            ai_representation: NotApplicable
        })
    },
    "ᕦ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕦ",
            ai_representation: NotApplicable
        })
    },
    "ᕧ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕧ",
            ai_representation: NotApplicable
        })
    },
    "ᕪ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("ch"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕪ",
            ai_representation: NotApplicable
        })
    },
    "ᖯ" => &SyllabicUnit {
        dialects: enum_set!(Aivilik),
        consonant: Some("b"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᖯ",
            ai_representation: NotApplicable
        })
    },
    "ᕴ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕴ",
            ai_representation: Classic
        })
    },
    "ᕹᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕹᐃ",
            ai_representation: Split
        })
    },
    "ᕵ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕵ",
            ai_representation: NotApplicable
        })
    },
    "ᕶ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕶ",
            ai_representation: NotApplicable
        })
    },
    "ᕷ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕷ",
            ai_representation: NotApplicable
        })
    },
    "ᕸ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕸ",
            ai_representation: NotApplicable
        })
    },
    "ᕹ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕹ",
            ai_representation: NotApplicable
        })
    },
    "ᕺ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕺ",
            ai_representation: NotApplicable
        })
    },
    "ᕻ" => &SyllabicUnit {
        dialects: enum_set!(Nunavik),
        consonant: Some("h"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᕻ",
            ai_representation: NotApplicable
        })
    },
    "ᓴ̵ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ai"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ̵ᐃ",
            ai_representation: Split
        })
    },
    "ᓯ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("i"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓯ̵",
            ai_representation: NotApplicable
        })
    },
    "ᓰ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("ii"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓰ̵",
            ai_representation: NotApplicable
        })
    },
    "ᓱ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("u"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓱ̵",
            ai_representation: NotApplicable
        })
    },
    "ᓲ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("uu"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓲ̵",
            ai_representation: NotApplicable
        })
    },
    "ᓴ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("a"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ̵",
            ai_representation: NotApplicable
        })
    },
    "ᓵ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: Some("aa"),
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᓵ̵",
            ai_representation: NotApplicable
        })
    },
    "ᔅ̵" => &SyllabicUnit {
        dialects: enum_set!(Nattilik),
        consonant: Some("š"),
        vowel: None,
        original: Syllabic(SyllabicSyllabicUnit {
            representation: "ᔅ̵",
            ai_representation: NotApplicable
        })
    }
};

/// English words that contain only letters found in Inuktitut. To be used to
/// filter ambiguities. Automatically generated.
pub static ENGLISH_WORDS: phf::Set<&str> = phf_set!{
    "a",
    "aa",
    "aaa",
    "ab",
    "abc",
    "abraham",
    "abs",
    "abstract",
    "abstracts",
    "abu",
    "ac",
    "acc",
    "acm",
    "act",
    "acting",
    "activists",
    "acts",
    "actual",
    "ag",
    "again",
    "against",
    "aging",
    "agricultural",
    "ah",
    "ai",
    "aim",
    "aims",
    "air",
    "aj",
    "ak",
    "aka",
    "al",
    "ala",
    "alabama",
    "alan",
    "alarm",
    "alaska",
    "albania",
    "album",
    "albums",
    "ali",
    "alias",
    "align",
    "all",
    "allah",
    "allan",
    "alpha",
    "alt",
    "aluminium",
    "aluminum",
    "alumni",
    "am",
    "amp",
    "an",
    "ana",
    "animal",
    "animals",
    "ann",
    "anna",
    "annual",
    "ant",
    "antarctica",
    "anti",
    "antigua",
    "antivirus",
    "ap",
    "apart",
    "api",
    "apnic",
    "app",
    "apparatus",
    "applicant",
    "applicants",
    "appraisal",
    "apps",
    "apr",
    "april",
    "apt",
    "aqua",
    "aquarium",
    "aquatic",
    "ar",
    "arab",
    "arabia",
    "arabic",
    "arc",
    "arch",
    "arctic",
    "arg",
    "arising",
    "arkansas",
    "arm",
    "arms",
    "arrival",
    "arrivals",
    "art",
    "arthritis",
    "arthur",
    "artist",
    "artistic",
    "artists",
    "arts",
    "aruba",
    "as",
    "ascii",
    "ash",
    "asia",
    "asian",
    "asin",
    "ask",
    "asking",
    "asks",
    "asn",
    "asp",
    "assault",
    "assign",
    "assist",
    "assistant",
    "assists",
    "assuming",
    "asthma",
    "asus",
    "at",
    "ata",
    "ati",
    "atlanta",
    "atlantic",
    "atlas",
    "atm",
    "attach",
    "attack",
    "attacks",
    "attract",
    "au",
    "auburn",
    "aug",
    "august",
    "aus",
    "austin",
    "australia",
    "australian",
    "austria",
    "autumn",
    "av",
    "avatar",
    "avg",
    "avi",
    "b",
    "ba",
    "back",
    "backing",
    "backup",
    "bag",
    "bags",
    "bahamas",
    "bahrain",
    "baking",
    "bali",
    "ball",
    "ban",
    "banana",
    "bang",
    "bank",
    "banking",
    "banks",
    "baptist",
    "bar",
    "barbara",
    "bargain",
    "bargains",
    "barn",
    "bars",
    "basic",
    "basics",
    "basin",
    "basis",
    "bass",
    "bat",
    "batch",
    "bath",
    "baths",
    "batman",
    "bb",
    "bbc",
    "bbs",
    "bc",
    "bg",
    "bhutan",
    "bi",
    "bias",
    "biblical",
    "big",
    "bikini",
    "bill",
    "billing",
    "bills",
    "bin",
    "birmingham",
    "birth",
    "bit",
    "bits",
    "bk",
    "bl",
    "black",
    "blackjack",
    "blacks",
    "blah",
    "blair",
    "blank",
    "blast",
    "blink",
    "bm",
    "bp",
    "br",
    "bra",
    "brain",
    "branch",
    "bras",
    "brass",
    "brian",
    "brick",
    "bright",
    "brilliant",
    "bring",
    "bringing",
    "brings",
    "britain",
    "britannica",
    "british",
    "brush",
    "brutal",
    "bs",
    "bt",
    "buck",
    "bucks",
    "bug",
    "bugs",
    "built",
    "bulgaria",
    "bulgarian",
    "bulk",
    "bull",
    "bunch",
    "burn",
    "burning",
    "burns",
    "burst",
    "bus",
    "bush",
    "but",
    "butts",
    "c",
    "ca",
    "cab",
    "cabin",
    "cal",
    "calcium",
    "call",
    "calling",
    "calls",
    "calm",
    "calvin",
    "cam",
    "camp",
    "campaign",
    "campaigns",
    "camping",
    "camps",
    "campus",
    "cams",
    "can",
    "canal",
    "cant",
    "canvas",
    "cap",
    "capital",
    "caps",
    "captain",
    "car",
    "carb",
    "caring",
    "carl",
    "carnival",
    "cars",
    "cart",
    "cas",
    "casa",
    "cash",
    "cast",
    "casting",
    "casual",
    "cat",
    "catch",
    "cats",
    "caught",
    "causing",
    "cb",
    "cbs",
    "cc",
    "cg",
    "cgi",
    "ch",
    "chain",
    "chains",
    "chair",
    "chairman",
    "chairs",
    "chan",
    "changing",
    "char",
    "charging",
    "charm",
    "charming",
    "charms",
    "chart",
    "charts",
    "chassis",
    "chat",
    "chi",
    "chick",
    "chicks",
    "china",
    "chip",
    "chips",
    "chris",
    "christ",
    "christian",
    "christians",
    "christina",
    "christmas",
    "chuck",
    "church",
    "ci",
    "cia",
    "cialis",
    "cincinnati",
    "cingular",
    "cir",
    "circuit",
    "circuits",
    "circular",
    "circus",
    "civic",
    "civil",
    "civilian",
    "cj",
    "cl",
    "claim",
    "claims",
    "clan",
    "clara",
    "clark",
    "class",
    "classic",
    "classical",
    "classics",
    "click",
    "clicking",
    "clicks",
    "climb",
    "climbing",
    "clinic",
    "clinical",
    "clinics",
    "clip",
    "clips",
    "club",
    "clubs",
    "cm",
    "cms",
    "cn",
    "cnn",
    "cp",
    "cpu",
    "cr",
    "crack",
    "craig",
    "craps",
    "crash",
    "criminal",
    "crisis",
    "critical",
    "criticism",
    "critics",
    "crm",
    "crucial",
    "cs",
    "css",
    "cst",
    "ct",
    "ctrl",
    "cu",
    "cuba",
    "cubic",
    "cult",
    "cultural",
    "cup",
    "cups",
    "curriculum",
    "curtis",
    "cut",
    "cuts",
    "cutting",
    "cv",
    "cvs",
    "g",
    "ga",
    "gain",
    "gains",
    "gambling",
    "gaming",
    "gamma",
    "gang",
    "gap",
    "gaps",
    "garcia",
    "garlic",
    "garmin",
    "gas",
    "gb",
    "gba",
    "gbp",
    "gc",
    "gcc",
    "gg",
    "ghana",
    "gi",
    "giant",
    "giants",
    "gibraltar",
    "gig",
    "girl",
    "girls",
    "gis",
    "giving",
    "gl",
    "glass",
    "gm",
    "gmbh",
    "gmc",
    "gmt",
    "gnu",
    "gp",
    "gpl",
    "gps",
    "gr",
    "grab",
    "graham",
    "grain",
    "grammar",
    "grams",
    "grant",
    "grants",
    "graph",
    "graphic",
    "graphical",
    "graphics",
    "graphs",
    "gras",
    "grass",
    "gratis",
    "gratuit",
    "grill",
    "grip",
    "gs",
    "gsm",
    "gst",
    "gt",
    "gtk",
    "guam",
    "gui",
    "guitar",
    "guitars",
    "gun",
    "guns",
    "guru",
    "h",
    "ha",
    "habitat",
    "habits",
    "hack",
    "hair",
    "haiti",
    "hall",
    "ham",
    "hamburg",
    "hang",
    "hanging",
    "hans",
    "harm",
    "harris",
    "hart",
    "has",
    "hash",
    "hat",
    "hats",
    "having",
    "hb",
    "hc",
    "hh",
    "hi",
    "high",
    "highlight",
    "highlights",
    "highs",
    "hiking",
    "hill",
    "hills",
    "him",
    "hint",
    "hints",
    "hip",
    "hiring",
    "his",
    "hispanic",
    "hist",
    "hit",
    "hitachi",
    "hits",
    "hitting",
    "hiv",
    "hk",
    "hl",
    "hp",
    "hq",
    "hr",
    "hrs",
    "hs",
    "ht",
    "html",
    "http",
    "hu",
    "hub",
    "hugh",
    "hull",
    "human",
    "humanitarian",
    "humans",
    "hung",
    "hungarian",
    "hunt",
    "hunting",
    "hurt",
    "i",
    "ia",
    "ian",
    "ibm",
    "ic",
    "icq",
    "ict",
    "ii",
    "iii",
    "il",
    "ill",
    "im",
    "imaging",
    "img",
    "immigrants",
    "impact",
    "impacts",
    "in",
    "inc",
    "inch",
    "incl",
    "ing",
    "initial",
    "ink",
    "inn",
    "inns",
    "input",
    "inputs",
    "ins",
    "insight",
    "insights",
    "install",
    "installing",
    "instant",
    "insulin",
    "int",
    "intl",
    "ip",
    "ipaq",
    "ips",
    "ir",
    "ira",
    "iran",
    "iraq",
    "iraqi",
    "irc",
    "irish",
    "irs",
    "is",
    "isa",
    "isaac",
    "isbn",
    "islam",
    "islamic",
    "isp",
    "issn",
    "ist",
    "istanbul",
    "it",
    "italia",
    "italian",
    "italic",
    "its",
    "iv",
    "j",
    "ja",
    "jack",
    "jaguar",
    "jail",
    "jam",
    "jamaica",
    "jan",
    "japan",
    "jar",
    "java",
    "javascript",
    "jc",
    "jill",
    "jim",
    "jj",
    "jm",
    "jp",
    "jpg",
    "jr",
    "js",
    "juan",
    "jul",
    "julia",
    "julian",
    "jump",
    "jumping",
    "jun",
    "junk",
    "just",
    "justin",
    "jvc",
    "k",
    "ka",
    "kai",
    "kansas",
    "karl",
    "karma",
    "katrina",
    "kb",
    "kg",
    "kick",
    "kijiji",
    "kill",
    "killing",
    "kills",
    "kim",
    "king",
    "kings",
    "kirk",
    "kiss",
    "kissing",
    "kit",
    "kits",
    "km",
    "knight",
    "knights",
    "knit",
    "knitting",
    "ks",
    "kurt",
    "l",
    "la",
    "lab",
    "labs",
    "lack",
    "lamb",
    "lamp",
    "lamps",
    "lan",
    "lang",
    "lanka",
    "lap",
    "las",
    "last",
    "lasting",
    "lat",
    "latin",
    "latina",
    "latinas",
    "latvia",
    "laugh",
    "laughing",
    "launch",
    "laura",
    "lb",
    "lbs",
    "lc",
    "lg",
    "li",
    "lib",
    "librarian",
    "libs",
    "licking",
    "light",
    "lighting",
    "lightning",
    "lights",
    "lil",
    "limit",
    "limiting",
    "limits",
    "link",
    "linking",
    "links",
    "lip",
    "lips",
    "lisa",
    "list",
    "listing",
    "listings",
    "lists",
    "lit",
    "lithuania",
    "living",
    "ll",
    "llc",
    "llp",
    "lm",
    "ln",
    "lp",
    "ls",
    "lt",
    "lu",
    "lucas",
    "lucia",
    "luck",
    "luis",
    "lunch",
    "lung",
    "m",
    "ma",
    "mac",
    "mag",
    "magic",
    "magical",
    "mai",
    "mail",
    "mailing",
    "mailman",
    "mails",
    "main",
    "maintain",
    "maintaining",
    "maintains",
    "making",
    "mali",
    "mall",
    "malta",
    "man",
    "managing",
    "manga",
    "manhattan",
    "manual",
    "manuals",
    "map",
    "mapping",
    "maps",
    "mar",
    "marc",
    "march",
    "marcus",
    "margin",
    "maria",
    "mariah",
    "marijuana",
    "marina",
    "mark",
    "marking",
    "marks",
    "mars",
    "marsh",
    "marshall",
    "mart",
    "martha",
    "martial",
    "martin",
    "mas",
    "mask",
    "mass",
    "mat",
    "match",
    "matching",
    "math",
    "mating",
    "mats",
    "matt",
    "maui",
    "mauritius",
    "mb",
    "mba",
    "mc",
    "mg",
    "mh",
    "mi",
    "mia",
    "miami",
    "mic",
    "michigan",
    "might",
    "mil",
    "milan",
    "milk",
    "mill",
    "mills",
    "min",
    "mini",
    "minimal",
    "minimum",
    "mining",
    "mins",
    "mint",
    "minus",
    "misc",
    "miss",
    "missing",
    "mississippi",
    "mit",
    "mitsubishi",
    "mj",
    "ml",
    "mlb",
    "mls",
    "mm",
    "mn",
    "mp",
    "mpg",
    "mph",
    "mr",
    "mrna",
    "mrs",
    "ms",
    "msg",
    "msgstr",
    "msn",
    "mt",
    "mtv",
    "mu",
    "much",
    "mug",
    "multi",
    "mumbai",
    "munich",
    "municipal",
    "music",
    "musical",
    "musician",
    "musicians",
    "muslim",
    "muslims",
    "must",
    "mustang",
    "mutual",
    "mv",
    "n",
    "na",
    "nail",
    "nails",
    "nam",
    "namibia",
    "nasa",
    "nascar",
    "nat",
    "nathan",
    "natural",
    "naturals",
    "nav",
    "naval",
    "nb",
    "nba",
    "nbc",
    "nc",
    "ncaa",
    "ng",
    "nh",
    "nhl",
    "nhs",
    "ni",
    "niagara",
    "nicaragua",
    "nick",
    "night",
    "nights",
    "nil",
    "nirvana",
    "nissan",
    "nj",
    "nl",
    "nm",
    "nn",
    "np",
    "nr",
    "ns",
    "nt",
    "ntsc",
    "nu",
    "null",
    "nursing",
    "nut",
    "nuts",
    "nv",
    "p",
    "pa",
    "pac",
    "pack",
    "packaging",
    "packing",
    "packs",
    "pain",
    "paint",
    "paintball",
    "painting",
    "paintings",
    "pair",
    "pairs",
    "pakistan",
    "pal",
    "palm",
    "pam",
    "pan",
    "panama",
    "panic",
    "pants",
    "papua",
    "par",
    "para",
    "paragraph",
    "paragraphs",
    "paris",
    "parish",
    "park",
    "parking",
    "parks",
    "part",
    "partial",
    "participant",
    "participants",
    "participating",
    "particular",
    "parts",
    "pas",
    "pass",
    "passing",
    "past",
    "pasta",
    "pat",
    "patch",
    "path",
    "paths",
    "patricia",
    "patrick",
    "paul",
    "pb",
    "pc",
    "pci",
    "pcs",
    "pct",
    "pg",
    "pgp",
    "ph",
    "phi",
    "phil",
    "philip",
    "philips",
    "phillips",
    "php",
    "phpbb",
    "pi",
    "pic",
    "pick",
    "picking",
    "picks",
    "pickup",
    "picnic",
    "pics",
    "pig",
    "pill",
    "pills",
    "pin",
    "ping",
    "pink",
    "pins",
    "pit",
    "pitch",
    "pittsburgh",
    "pj",
    "pk",
    "pl",
    "placing",
    "plain",
    "plains",
    "plan",
    "planning",
    "plans",
    "plant",
    "plants",
    "plasma",
    "plastic",
    "plastics",
    "platinum",
    "plc",
    "plug",
    "plugin",
    "plugins",
    "plumbing",
    "plus",
    "pm",
    "pmc",
    "pn",
    "pp",
    "ppc",
    "ppm",
    "pr",
    "practical",
    "pricing",
    "principal",
    "print",
    "printing",
    "prints",
    "ps",
    "psi",
    "psp",
    "pst",
    "pt",
    "pts",
    "pub",
    "public",
    "publish",
    "publishing",
    "pubs",
    "pull",
    "pulling",
    "pump",
    "pumps",
    "punch",
    "punk",
    "pupils",
    "purchasing",
    "pursuant",
    "pursuit",
    "push",
    "pushing",
    "put",
    "puts",
    "putting",
    "pvc",
    "q",
    "qatar",
    "qc",
    "qt",
    "quantum",
    "qui",
    "quick",
    "quilt",
    "quit",
    "r",
    "ra",
    "rabbit",
    "racial",
    "racing",
    "rack",
    "racks",
    "rail",
    "rain",
    "raising",
    "ralph",
    "ram",
    "ran",
    "ranch",
    "ranging",
    "rank",
    "ranking",
    "rankings",
    "ranks",
    "rap",
    "rat",
    "rating",
    "ratings",
    "rats",
    "rb",
    "rc",
    "rca",
    "rg",
    "rh",
    "ri",
    "rica",
    "rich",
    "rick",
    "right",
    "rights",
    "rim",
    "ring",
    "rings",
    "rip",
    "rising",
    "risk",
    "risks",
    "rj",
    "rl",
    "rm",
    "rn",
    "rna",
    "rp",
    "rpg",
    "rpm",
    "rr",
    "rrp",
    "rs",
    "rss",
    "rt",
    "ru",
    "rug",
    "rugs",
    "ruling",
    "run",
    "running",
    "runs",
    "rural",
    "rush",
    "russia",
    "russian",
    "ruth",
    "rv",
    "s",
    "sa",
    "sail",
    "sailing",
    "saint",
    "saints",
    "salt",
    "sam",
    "samba",
    "sampling",
    "samsung",
    "san",
    "sans",
    "santa",
    "sap",
    "sara",
    "sarah",
    "sas",
    "sat",
    "satin",
    "saturn",
    "savannah",
    "saving",
    "savings",
    "sb",
    "sbjct",
    "sc",
    "scan",
    "scanning",
    "sci",
    "scratch",
    "script",
    "scripting",
    "scripts",
    "scsi",
    "scuba",
    "sg",
    "sh",
    "shakira",
    "shall",
    "shanghai",
    "sharing",
    "shark",
    "sharp",
    "ship",
    "shipping",
    "ships",
    "shirt",
    "shirts",
    "shut",
    "si",
    "sic",
    "sick",
    "sig",
    "sight",
    "sigma",
    "sign",
    "signal",
    "signals",
    "signing",
    "signs",
    "signup",
    "silk",
    "sim",
    "similar",
    "sims",
    "sin",
    "sing",
    "singh",
    "singing",
    "sink",
    "sip",
    "sir",
    "sit",
    "sitting",
    "sk",
    "skating",
    "ski",
    "skiing",
    "skill",
    "skills",
    "skin",
    "skins",
    "skip",
    "skirt",
    "skirts",
    "sku",
    "sl",
    "slight",
    "slim",
    "slip",
    "sm",
    "small",
    "smart",
    "smith",
    "sms",
    "smtp",
    "sn",
    "snap",
    "sp",
    "spa",
    "spain",
    "spam",
    "span",
    "spanish",
    "spank",
    "spanking",
    "sparc",
    "spas",
    "spatial",
    "spin",
    "spirit",
    "spirits",
    "spiritual",
    "split",
    "spring",
    "springs",
    "sprint",
    "sq",
    "sql",
    "sr",
    "src",
    "sri",
    "ss",
    "ssl",
    "st",
    "stack",
    "stamp",
    "stamps",
    "stan",
    "star",
    "starring",
    "stars",
    "start",
    "starting",
    "starts",
    "startup",
    "stat",
    "static",
    "stating",
    "statistical",
    "statistics",
    "stats",
    "status",
    "stick",
    "sticks",
    "still",
    "str",
    "straight",
    "strain",
    "strap",
    "strict",
    "striking",
    "string",
    "strings",
    "strip",
    "strips",
    "struck",
    "struct",
    "structural",
    "stuart",
    "stuck",
    "stunning",
    "su",
    "sub",
    "subaru",
    "submit",
    "submitting",
    "substantial",
    "suburban",
    "such",
    "sucking",
    "sugar",
    "suit",
    "suits",
    "sullivan",
    "sum",
    "summit",
    "sun",
    "sur",
    "surgical",
    "surplus",
    "surprising",
    "survival",
    "susan",
    "sv",
    "t",
    "ta",
    "tab",
    "tabs",
    "tactics",
    "tag",
    "tags",
    "tail",
    "taking",
    "talk",
    "talking",
    "talks",
    "tall",
    "tamil",
    "tampa",
    "tan",
    "tank",
    "tanks",
    "tap",
    "tar",
    "task",
    "tasks",
    "taught",
    "tb",
    "tba",
    "tc",
    "tcp",
    "tgp",
    "th",
    "thai",
    "than",
    "thank",
    "thanks",
    "thanksgiving",
    "that",
    "thats",
    "thick",
    "thin",
    "thing",
    "things",
    "think",
    "thinking",
    "thinks",
    "this",
    "thru",
    "thu",
    "thumb",
    "thumbnail",
    "thumbnails",
    "thumbs",
    "thus",
    "ti",
    "tight",
    "til",
    "till",
    "tim",
    "timing",
    "tin",
    "tip",
    "tips",
    "titanium",
    "titans",
    "tm",
    "tmp",
    "tn",
    "tp",
    "tr",
    "track",
    "trackback",
    "trackbacks",
    "tracking",
    "tracks",
    "tract",
    "trail",
    "trails",
    "train",
    "training",
    "trains",
    "trans",
    "transcript",
    "transcripts",
    "transit",
    "transmit",
    "trap",
    "trash",
    "trauma",
    "travis",
    "tri",
    "trial",
    "trials",
    "tribal",
    "tribunal",
    "trick",
    "tricks",
    "trim",
    "trip",
    "trips",
    "triumph",
    "trivia",
    "truck",
    "trucks",
    "trunk",
    "trust",
    "trusts",
    "truth",
    "ts",
    "tsunami",
    "tt",
    "tu",
    "tub",
    "tulsa",
    "tuning",
    "tunisia",
    "turkish",
    "turn",
    "turning",
    "turns",
    "tv",
    "tvs",
    "u",
    "uc",
    "uh",
    "ui",
    "uk",
    "ul",
    "ultra",
    "ultram",
    "um",
    "un",
    "una",
    "uni",
    "unit",
    "units",
    "univ",
    "until",
    "unusual",
    "up",
    "upc",
    "ups",
    "ur",
    "urban",
    "uri",
    "url",
    "urls",
    "us",
    "usa",
    "usb",
    "usc",
    "usgs",
    "using",
    "usps",
    "usr",
    "usual",
    "ut",
    "utah",
    "utc",
    "utils",
    "uv",
    "v",
    "va",
    "vacuum",
    "val",
    "valium",
    "van",
    "vanilla",
    "var",
    "vast",
    "vat",
    "vatican",
    "vault",
    "vb",
    "vc",
    "vcr",
    "vg",
    "vhs",
    "vi",
    "via",
    "vic",
    "victim",
    "victims",
    "vii",
    "viii",
    "viking",
    "villa",
    "villas",
    "vip",
    "viral",
    "virgin",
    "virginia",
    "virtual",
    "virus",
    "visa",
    "visit",
    "visiting",
    "visits",
    "vista",
    "visual",
    "vital",
    "vitamin",
    "vitamins",
    "vp",
    "vpn",
    "vs",
    "vt"
};