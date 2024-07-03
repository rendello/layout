// [!WARNING]

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
// [!TABLE]
};