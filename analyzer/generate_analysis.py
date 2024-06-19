

from typing import List

VOWELS = ["ai", "i", "ii", "u", "uu", "a", "aa"]
HEADER = ["consonant"] + VOWELS + ["final", "dialect"]


@dataclass
class Series:
	consonant: str,
	vowels: List[str]


def assert_eq(a, b):
	if a != b:
		print(f"Assertion failure:\nA: {a}\nB: {b}\n")
		raise AssertionError


if __name__ == "__main__":
	with open("table.tsv", 'r') as f:
		series_list = to_series(f.read())
	
	print(all_possible_latin_syllabic_units(series_list))


def extract_series(table):
    COLUMN_IDS = ["consonant"] + VOWELS 

    series = []
    for row in table:
        columns = row.split("\t")
        if len(columns) != len(COLUMN_IDS):
            raise Exception("Header length != row length. Use tabs, not spaces.\n"+str(cols))

        series.append({key: columns[i] for i, key in enumerate(COLUMN_IDS)})
    return series


def to_series(table):
	lines = [line.split("\t") for line in table.split("\n")]

	series_list = []







def all_possible_latin_syllabic_units(series) -> List[str]:
	[s.consonant + vowel for vowel in s.vowels for s in series if s.consonant is not None]