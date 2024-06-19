
import csv
from dataclasses import dataclass

from typing import List, Dict, TextIO

VOWEL_NAMES_AND_VALUES = [
	("ai", "ai"),
	("i", "i"),
	("long_i", "ii"),
	("u", "u"),
	("long_u", "uu"),
	("a", "a"),
	("long_a", "aa"),
	("final", "")
]

@dataclass
class SyllabicUnit:
	latin: str
	syllabic: str


def tsv_to_series_list(file: TextIO) -> List[Dict]:
    return list(csv.DictReader(file, delimiter="\t", restval=""))


def series_list_to_syllabic_units(series_list: List[Dict]) -> List[SyllabicUnit]:
	syllabic_units = []
	for series in series_list:
		for vowel_name, vowel_value in VOWEL_NAMES_AND_VALUES:
			latin = series["consonant"] + vowel_value
			syllabic = series[vowel_name]

			if syllabic != "":
				syllabic_units.append(SyllabicUnit(latin, syllabic))

	return syllabic_units



if __name__ == "__main__":

	with open("table.tsv") as csv_file:
		series_list = tsv_to_series_list(csv_file)

	for l in series_list_to_syllabic_units(series_list):
		print(l)