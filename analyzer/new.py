
import csv
from dataclasses import dataclass

from typing import Union, List, Literal, Dict, TextIO

AiRepresentation = Literal["ring", "classic", "split", "not_applicable"]

@dataclass
class LatinSyllabicUnit:
	consonant: str

@dataclass
class SyllabicSyllabicUnit:
	syllabic: str
	ai_representation: AiRepresentation

@dataclass
class SyllabicUnit:
	dialects: List[str]
	consonant: str
	vowel: str
	original: Union[LatinSyllabicUnit, SyllabicSyllabicUnit]


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
			self.series_list = list(csv.DictReader(file, delimiter="\t", restval=""))

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
			for vowel_name, vowel_value in VOWEL_NAMES_AND_VALUES:
				latin = series["consonant"] + vowel_value
				syllabic = series[vowel_name]

				if syllabic != "":
					syllabic_units.append(SyllabicUnit(latin, syllabic))

		return syllabic_units

	def to_syllabic_units(self) -> List[SyllabicUnit]:
		syllabic_units = []
		for series in self.series_list:
			syllabic_units += self.series_to_syllabic_units(series)
		return syllabic_units

	def series_to_syllabic_units(self, series: Dict) -> List[SyllabicUnit]:
		syllabic_units = []

		vowel_columns_and_values = [
			("ring", "ai"),
			("ai", "ai"),
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
			if vowel_column == "ring":
				ai = "ring"
			elif vowel_column == "ai":
				ai = "classic"
			else:
				ai = "not_applicable"
			# Add implicit ai split

			dialects = series["dialect"]
			if dialects == []:
				dialects = self.all_dialects()

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



# def format_syllabic_entry(su: SyllabicUnit) -> str:

# 	series_dialects = su.dialect if su.dialect != "" else

# 	["Dialect::{dialect}" for dialect in ]
# 	dialects_str = enum_set!(Dialect::Nunavut | Dialect::Quebec)

# 	return (
# 		"{0} => SyllabicUnit \{"
# 		"dialects: "
# 		"\}"
# 	)


if __name__ == "__main__":
	series_data = SeriesData("table.tsv")

	# for s in series_data.series_list:
	# 	print(s)

	for su in series_data.to_syllabic_units():
		print(su)



	# for s in series_list_to_syllabic_units(series_list):
	# 	# print(l)

	# 	print(f'''"{s.syllabic}" => "{s.latin}"''')
	# print(all_dialects(series_list))
