import csv
from datetime import datetime
from dataclasses import dataclass
from zoneinfo import ZoneInfo

from typing import Union, List, Literal, Dict


VOWEL_COLUMNS_AND_VALUES = [
	("ai_ring", "ai"),
	("ai_classic", "ai"),
	("ai_split", "ai"),
	("i", "i"),
	("long_i", "ii"),
	("u", "u"),
	("long_u", "uu"),
	("a", "a"),
	("long_a", "aa"),
	("final", "")
]

AiRepresentation = Literal["Ring", "Classic", "Split", "NotApplicable"]


def string_to_rust_optional(s: str) -> str:
	if s == "":
		return "None"
	else:
		return f'''Some("{s}")'''


@dataclass
class LatinSyllabicUnit:
	representation: str
	consonant: str

@dataclass
class SyllabicSyllabicUnit:
	representation: str
	ai_representation: AiRepresentation

@dataclass
class SyllabicUnit:
	dialects: List[str]
	consonant: str
	vowel: str
	original: Union[LatinSyllabicUnit, SyllabicSyllabicUnit]

	def dialect_string(self) -> str:
		return " | ".join([f"Dialect::{d}" for d in self.dialects])

	def to_latin_entry(self) -> str:
		return (
			f'''    "{self.original.representation}" => &SyllabicUnit {{\n'''
			f'''        dialects: enum_set!({self.dialect_string()}),\n'''
			f'''        consonant: {string_to_rust_optional(self.consonant)},\n'''
			f'''        vowel: {string_to_rust_optional(self.vowel)},\n'''
			f'''        original: SyllabicUnitRepresentation::Latin(LatinSyllabicUnit {{\n'''
			f'''            representation: "{self.original.representation}",\n'''
			f'''            consonant: {string_to_rust_optional(self.original.consonant)}\n'''
			f'''        }})\n'''
			f'''    }}'''
		)

	def to_syllabic_entry(self) -> str:
		return (
			f'''    "{self.original.representation}" => &SyllabicUnit {{\n'''
			f'''        dialects: enum_set!({self.dialect_string()}),\n'''
			f'''        consonant: {string_to_rust_optional(self.consonant)},\n'''
			f'''        vowel: {string_to_rust_optional(self.vowel)},\n'''
			f'''        original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {{\n'''
			f'''            representation: "{self.original.representation}",\n'''
			f'''            ai_representation: AiRepresentation::{self.original.ai_representation}\n'''
			f'''        }})\n'''
			f'''    }}'''
		)


class SeriesData:
	""" A representation of the syllabic TSV table.

	The table holds a lot of intrensic information which will be used
	as the basis for almost all analysis and conversion data in the
	generated Rust code.

	Its methods generate the `SyllabicUnit`s for the analysis and
	transliteration tool.
	"""

	series_list: List[Dict]

	def __init__(self, table_tsv):
		with open(table_tsv) as file:
			series_list = list(csv.DictReader(file, delimiter="\t", restval=""))

		for series in series_list:
			if series["a"] == "":
				series["ai_split"] = ""
			else:
				series["ai_split"] = series["a"] + "ᐃ" # In place.

		self.series_list = series_list

	def all_dialects(self) -> List[str]:
		dialects = []
		for series in self.series_list:
			d = series["dialect"]
			if d != "" and d not in dialects:
				dialects.append(d)
		return dialects

	def to_latin_syllabic_units(self) -> List[SyllabicUnit]:
		unfiltered_syllabic_units = []
		for series in self.series_list:
			unfiltered_syllabic_units += self.series_to_latin_syllabic_units(series)

		syllabic_units = []
		seen_representations = []
		for syllabic_unit in unfiltered_syllabic_units:
			representation = syllabic_unit.original.representation
			if representation not in seen_representations:
				seen_representations.append(representation)
				syllabic_units.append(syllabic_unit)

		return syllabic_units

	def series_to_latin_syllabic_units(self, series: Dict) -> List[SyllabicUnit]:
		syllabic_units = []

		for vowel_column, vowel in VOWEL_COLUMNS_AND_VALUES:
			consonant = series["consonant"]

			latin_representation = consonant + vowel
			syllabic_representation = series[vowel_column]

			dialect = series["dialect"]
			if dialect == "":
				dialects = self.all_dialects()
			else:
				dialects = [dialect]

			if syllabic_representation != "":
				su = SyllabicUnit(
					dialects,
					consonant,
					vowel,
					LatinSyllabicUnit(
						latin_representation,
						consonant
					)
				)

				syllabic_units.append(su)
		return syllabic_units

	def to_syllabic_syllabic_units(self) -> List[SyllabicUnit]:
		syllabic_units = []
		for series in self.series_list:
			syllabic_units += self.series_to_syllabic_syllabic_units(series)
		return syllabic_units

	def series_to_syllabic_syllabic_units(self, series: Dict) -> List[SyllabicUnit]:
		syllabic_units = []

		for vowel_column, vowel in VOWEL_COLUMNS_AND_VALUES:
			consonant = series["consonant"]

			syllabic_representation = series[vowel_column]

			ai: AiRepresentation
			if vowel_column == "ai_ring":
				ai = "Ring"
			elif vowel_column == "ai_classic":
				ai = "Classic"
			elif vowel_column == "ai_split":
				ai = "Split"
			else:
				ai = "NotApplicable"

			dialect = series["dialect"]
			if dialect == "":
				dialects = self.all_dialects()
			else:
				dialects = [dialect]

			if syllabic_representation != "":
				su = SyllabicUnit(
					dialects,
					consonant,
					vowel,
					SyllabicSyllabicUnit(
						syllabic_representation,
						ai
					)
				)

				syllabic_units.append(su)
		return syllabic_units

	def all_valid_inuktitut_latin_letters(self) -> List[str]:
		alphabet = set("abcdefghijklmnopqrstuvwxyz")
		letters = set("aiu")
		for series in self.series_list:
			for letter in series["consonant"]:
				letters.add(letter)

		alphabet -= letters
		return sorted(list(alphabet))


def generate():
	date = datetime.now(ZoneInfo("Canada/Eastern")).astimezone().strftime("on %B %d at %H:%M EST")

	series_data = SeriesData("table.tsv")
	syllabic_entries = ",\n".join([su.to_syllabic_entry() for su in series_data.to_syllabic_syllabic_units()])
	latin_entries = ",\n".join([su.to_latin_entry() for su in series_data.to_latin_syllabic_units()])

	return (
		f'''// ==============================================\n'''
		f'''// AUTOGENERATED {date}.\n'''
		f'''// See `../../generate_maps.py`.\n'''
		f'''// ==============================================\n\n'''
		f'''use phf_macros::phf_map;\n'''
		f'''use crate::syllabic_unit::*;\n\n'''
		f'''pub static LATIN_MAP: SyllabicUnitMap = phf_map! {{\n'''
		f'''{latin_entries}\n'''
		f'''}};\n\n'''
		f'''pub static SYLLABIC_MAP: SyllabicUnitMap = phf_map! {{\n'''
		f'''{syllabic_entries}\n'''
		f'''}};'''
	)


def update_data_file():
	generated = generate()

	if input("Overwrite `data.rs`? [Y/n]: ").upper() != "Y":
		print("Aborting.")
		return

	with open("analyzer/src/data.rs", "w") as file:
		file.write(generated)
		print("`data.rs` updated")


if __name__ == "__main__":
	update_data_file()