
pub use enumset::{enum_set, EnumSet, EnumSetType};

#[derive(Debug, EnumSetType)]
pub enum Dialect {
    Nunavut,
    Nattilik,
    Aivilik,
    Nunavik,
}

pub type DialectSet = EnumSet<Dialect>;


#[derive(Debug, Clone)]
pub struct SyllabicUnit {
    pub dialects: DialectSet,
    pub consonant: Option<&'static str>,
    pub vowel: Option<&'static str>,
    pub original: SyllabicUnitRepresentation,
}

impl SyllabicUnit {

    /// Perhaps join to existing string?
    fn normalized_string(&self) -> String {
        self.consonant.unwrap_or_default().to_owned() 
        + self.vowel.unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub enum SyllabicUnitRepresentation {
    Latin(LatinSyllabicUnit),
    Syllabic(SyllabicSyllabicUnit)
}

#[derive(Debug, Clone)]
pub struct LatinSyllabicUnit {
    pub consonant: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct SyllabicSyllabicUnit {
    pub representation: &'static str,
    pub ai_representation: AiRepresentation
}

#[derive(Debug, Clone)]
pub enum AiRepresentation {
    Split,
    Classic,
    Ring,
    NotApplicable
}

pub type SyllabicUnitMap = phf::Map<&'static str, &'static SyllabicUnit>;