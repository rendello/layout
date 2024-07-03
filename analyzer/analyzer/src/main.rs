// !!! GENERATED FILE, DO NOT EDIT !!!
// See `template.rs` and `new.py`
//
// July 03 00:09

#![allow(dead_code)]


use phf_macros::phf_map;
use enumset::enum_set;


use enumset::{EnumSet, EnumSetType};

#[derive(Debug, EnumSetType)]
enum Dialect {
	Nunavut,
	Nattilik,
	Aivilik,
	Nunavik,
}

type DialectSet = EnumSet<Dialect>;


#[derive(Debug, Clone)]
pub struct SyllabicUnit {
	dialects: DialectSet,
	consonant: Option<&'static str>,
	vowel: Option<&'static str>,
	original: SyllabicUnitRepresentation,
}

impl SyllabicUnit {

	/// Perhaps join to existing string?
	fn normalized_string(&self) -> String {
		self.consonant.unwrap_or_default().to_owned() 
		+ self.vowel.unwrap_or_default()
	}
}

#[derive(Debug, Clone)]
enum SyllabicUnitRepresentation {
	Latin(LatinSyllabicUnit),
	Syllabic(SyllabicSyllabicUnit)
}

#[derive(Debug, Clone)]
struct LatinSyllabicUnit {
	consonant: Option<&'static str>,
}

#[derive(Debug, Clone)]
struct SyllabicSyllabicUnit {
	representation: &'static str,
	ai_representation: AiRepresentation
}

#[derive(Debug, Clone)]
enum AiRepresentation {
	Split,
	Classic,
	Ring,
	NotApplicable
}


pub fn main() {
	// println!("{:?}", MAP.get("ᕦᐃ").expect(""));
	// println!("{:?}", MAP.get("ᕦᐃ").expect("").normalized_string());
	println!("{:?}", pop_syllabic_unit("ᕦᐃᐃHello world!", &MAP));
}


fn bind_key_lengths(key_lengths: &[usize], max: usize) -> &[usize] {
    for (i, &length) in key_lengths.iter().enumerate() {
        if length < max { return &key_lengths[i..]; }
    };
    &key_lengths[key_lengths.len()..]
}

fn pop_syllabic_unit<'a>(text: &'a str, map: &Map) -> Option<(SyllabicUnit, &'a str)> {
	const MAX: usize = 16;
	let mut character_indices_array: [usize; MAX] = [0; MAX];
	let character_indices = {
		let mut array_index = 0;
		for (character_index, _f) in text.char_indices() {
			if character_index > MAX || array_index > MAX { break; }
			character_indices_array[array_index] = character_index;
			array_index += 1;
		}
		&character_indices_array[0..array_index]
	};

	for &character_index in character_indices.iter().rev() {
		if let Some(&su) = map.get(&text[0..character_index]) {
			return Some((su.clone(), &text[character_index..]))
		}
	}
	None
}


// ============================================================================



type Map = phf::Map<&'static str, &'static SyllabicUnit>;
pub static MAP: Map = phf_map! {
    "ᐂ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐂ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᐁ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐁ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᐊᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐊᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐃ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐄ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐄ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐅ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐅ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐆ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐆ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐊ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐊ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐋ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: None,
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐋ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐰ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐰ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᐯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐯ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᐸᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐸᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᐱ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐱ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐲ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐲ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐳ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐳ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐴ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐴ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐸ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐸ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᐹ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᐹ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑉ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("p"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑉ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑍ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑍ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᑌ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑌ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᑕᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑕᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᑎ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑎ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑏ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑏ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑐ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑐ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑑ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑑ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑕ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑕ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑖ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑖ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("t"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑬ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑬ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᑫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑫ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᑲᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑲᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᑭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑭ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑮ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑯ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑰ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑰ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑲ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑲ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑳ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑳ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("k"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒃ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒊ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒊ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᒉ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒉ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᒐᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒐᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᒋ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒋ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒌ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒌ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒍ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒍ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒎ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒎ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒐ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒐ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒑ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒑ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒡ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("g"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒡ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒤ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒤ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᒣ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒣ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᒪᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒪᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᒥ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒥ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒦ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒧ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒧ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒨ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒨ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒫ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᒻ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("m"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᒻ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓁ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓁ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᓀ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓀ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᓇᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓇᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᓂ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓂ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓃ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓄ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓄ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓅ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓅ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓇ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓇ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓈ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓈ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓐ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("n"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓐ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓮ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᓭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓭ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᓴᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᓯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓯ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓰ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓰ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓱ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓱ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓲ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓲ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓴ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓵ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓵ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔅ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("s"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔅ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓔ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓔ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᓓ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓓ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᓚᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓚᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᓕ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓕ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓖ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓖ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓗ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓗ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓘ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓘ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓚ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓚ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓛ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓛ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("l"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔧ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔧ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᔦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔦ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᔭᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔭᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᔨ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔨ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔩ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔩ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔫ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔭ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔮ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔾ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("j"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔾ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔧ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔧ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᑦᔦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔦ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᑦᔭᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔭᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᑦᔨ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔨ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔩ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔩ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔫ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔭ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔮ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑦᔾ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("jj"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑦᔾ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕔ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕔ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᕓ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕓ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᕙᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕙᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᕕ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕕ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕖ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕖ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕗ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕗ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕘ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕘ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕙ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕙ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕚ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕚ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕝ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("v"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕝ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕅ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕅ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᕂ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕂ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᕋᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕋᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᕆ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕆ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕇ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕇ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕈ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕈ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕉ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕉ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕋ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕋ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕌ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕌ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕐ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("r"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕐ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕾ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕾ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᙯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙯ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᖃᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖃᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᕿ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕿ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖀ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖀ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖁ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖁ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖂ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖂ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖃ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖄ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖄ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("q"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖎ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖎ",
            ai_representation: AiRepresentation::Ring
        })
    },    "ᙰ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙰ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᖓᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖓᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᖏ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖏ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖐ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖐ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖑ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖑ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖒ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖒ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖓ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖓ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖔ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖔ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖕ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ng"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖕ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙵᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙵᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᙱ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙱ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙲ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙲ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙳ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙳ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙴ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙴ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙵ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙵ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᙶ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᙶ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖖ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("nng"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖖ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᑊ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut | Dialect::Nattilik | Dialect::Aivilik | Dialect::Nunavik),
        consonant: Some("ʼ"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᑊ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑫ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᖅᑲᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑲᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᖅᑭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑭ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑮ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑯ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑰ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑰ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑲ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑲ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᑳ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᑳ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖅᒃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("qq"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖅᒃ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕼ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavut),
        consonant: Some("h"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕼ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖬᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖬᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᖨ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖨ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖩ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖩ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖫ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖫ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖬ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖬ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖭ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖭ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖮ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ř"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖮ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖤᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖤᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᖠ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖠ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖡ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖡ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖢ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖢ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖣ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖣ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖤ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖤ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖥ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖥ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ł"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖦ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪺ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪺ᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "𑪶" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪶",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪷" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪷",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪸" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪸",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪹" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪹",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪺" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪺",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "𑪻" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "𑪻",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕦᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕦᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᕠ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕠ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕢ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕢ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕤ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕤ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕥ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕥ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕦ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕦ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕧ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕧ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕪ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("ch"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕪ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᖯ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Aivilik),
        consonant: Some("b"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᖯ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕴ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕴ",
            ai_representation: AiRepresentation::Classic
        })
    },    "ᕹᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕹᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᕵ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕵ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕶ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕶ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕷ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕷ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕸ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕸ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕹ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕹ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕺ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕺ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᕻ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nunavik),
        consonant: Some("h"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᕻ",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓴ̵ᐃ" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("ai"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ̵ᐃ",
            ai_representation: AiRepresentation::Split
        })
    },    "ᓯ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("i"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓯ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓰ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("ii"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓰ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓱ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("u"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓱ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓲ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("uu"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓲ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓴ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("a"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓴ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᓵ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: Some("aa"),
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᓵ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },    "ᔅ̵" => &SyllabicUnit {
        dialects: enum_set!(Dialect::Nattilik),
        consonant: Some("š"),
        vowel: None,
        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {
            representation: "ᔅ̵",
            ai_representation: AiRepresentation::NotApplicable
        })
    },
};