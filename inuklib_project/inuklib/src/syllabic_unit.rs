pub use enumset::{enum_set, EnumSet, EnumSetType};

#[derive(Debug, EnumSetType)]
pub enum Dialect {
    Nunavut,
    Nattilik,
    Aivilik,
    Nunavik,
}

pub type DialectSet = EnumSet<Dialect>;

#[derive(Debug, PartialEq)]
pub struct SUToken<'a> {
    pub syllabic_unit: &'a SyllabicUnit,
    pub text: &'a str
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyllabicUnit {
    pub dialects: DialectSet,
    pub consonant: Option<&'static str>,
    pub vowel: Option<&'static str>,
    pub original: SyllabicUnitRepresentation,
}

impl SyllabicUnit {
    pub fn normalized_string(&self) -> String {
        self.consonant.unwrap_or_default().to_owned() 
        + self.vowel.unwrap_or_default()
    }

    pub fn original_representation(&self) -> String {
        match &self.original {
            SyllabicUnitRepresentation::Syllabic(ssu) => ssu.representation.to_owned(),
            SyllabicUnitRepresentation::Latin(lsu) => lsu.representation.to_owned()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyllabicUnitRepresentation {
    Latin(LatinSyllabicUnit),
    Syllabic(SyllabicSyllabicUnit)
}

#[derive(Debug, Clone, PartialEq)]
pub struct LatinSyllabicUnit {
    pub representation: &'static str,
    pub consonant: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyllabicSyllabicUnit {
    pub representation: &'static str,
    pub ai_representation: AiRepresentation
}

#[derive(Debug, Clone, PartialEq)]
pub enum AiRepresentation {
    Split,
    Classic,
    Ring,
    NotApplicable
}

#[derive(Debug, Clone, PartialEq)]
pub enum Script {
    Latin,
    Syllabic
}

pub type SyllabicUnitMap = phf::Map<&'static str, &'static SyllabicUnit>;

pub struct SyllabicUnitLookup {
    pub map: &'static SyllabicUnitMap,
    pub key_lengths: &'static [usize],
    pub must_normalize: bool,
    pub script: Script
}