
import csv
from dataclasses import dataclass

from typing import Union, List, Literal, Dict


def string_to_rust_optional(s: str) -> str:
	if s == "":
		return "None"
	else:
		return f'''Some("{s}")'''


AiRepresentation = Literal["Ring", "Classic", "Split", "NotApplicable"]

@dataclass
class LatinSyllabicUnit:
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

	def to_entry(self) -> str:

		dialect_string = " | ".join([f"Dialect::{d}" for d in self.dialects])


		return (
			f'''"{self.original.representation}" => &SyllabicUnit {{\n'''
			f'''    dialects: enum_set!({dialect_string}),\n'''
			f'''    consonant: {string_to_rust_optional(self.consonant)},\n'''
			f'''    vowel: {string_to_rust_optional(self.vowel)},\n'''
			f'''    original: SyllabicUnitRepresentation::Syllabic(SyllabicSyllabicUnit {{\n'''
			f'''        representation: "{self.original.representation}",\n'''
			f'''        ai_representation: AiRepresentation::{self.original.ai_representation}\n'''
			f'''    }})\n'''
			f'''}},'''
		)


class SeriesData:
	""" A representation of the syllabic TSV table.

	The table holds a lot of intrensic information which will be used
	as the basis for almost all analysis and conversion data in the
	generated Rust code.

	Its methods generate the `SyllabicUnit`s for the analysis and
	transliteration tool, as well as the enums required for the
	code to work.
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

	def to_syllabic_units(self) -> List[SyllabicUnit]:
		syllabic_units = []
		for series in self.series_list:
			syllabic_units += self.series_to_syllabic_units(series)
		return syllabic_units

	def series_to_syllabic_units(self, series: Dict) -> List[SyllabicUnit]:
		syllabic_units = []

		vowel_columns_and_values = [
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

		for vowel_column, vowel in vowel_columns_and_values:
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
			# Add implicit ai split

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



if __name__ == "__main__":
	series_data = SeriesData("table.tsv")

	for su in series_data.to_syllabic_units():
		# print(su)
		print(su.to_entry())
