
"""
Table adapted from Wikipedia article: "Inuktitut syllabics".
See: https://en.wikipedia.org/wiki/Inuktitut_syllabics
"""

from pprint import pprint
import re


table = [
    "	ᐁ	ᐃ	ᐄ	ᐅ	ᐆ	ᐊ	ᐋ	",
    "p	ᐯ	ᐱ	ᐲ	ᐳ	ᐴ	ᐸ	ᐹ	ᑉ",
    "t	ᑌ	ᑎ	ᑏ	ᑐ	ᑑ	ᑕ	ᑖ	ᑦ",
    "k	ᑫ	ᑭ	ᑮ	ᑯ	ᑰ	ᑲ	ᑳ	ᒃ",
    "h	ᕴ	ᕵ	ᕶ	ᕷ	ᕸ	ᕹ	ᕺ	ᕻ",
    "g	ᒉ	ᒋ	ᒌ	ᒍ	ᒎ	ᒐ	ᒑ	ᒡ",
    "m	ᒣ	ᒥ	ᒦ	ᒧ	ᒨ	ᒪ	ᒫ	ᒻ",
    "n	ᓀ	ᓂ	ᓃ	ᓄ	ᓅ	ᓇ	ᓈ	ᓐ",
    "s	ᓭ	ᓯ	ᓰ	ᓱ	ᓲ	ᓴ	ᓵ	ᔅ",
    "š		𑪶	𑪷	𑪸	𑪹	𑪺	𑪻	",
    "h		𑪰	𑪱	𑪲	𑪳	𑪴	𑪵	",
    "l	ᓓ	ᓕ	ᓖ	ᓗ	ᓘ	ᓚ	ᓛ	ᓪ",
    "j	ᔦ	ᔨ	ᔩ	ᔪ	ᔫ	ᔭ	ᔮ	ᔾ",
    "jj	ᑦᔦ	ᑦᔨ	ᑦᔩ	ᑦᔪ	ᑦᔫ	ᑦᔭ	ᑦᔮ	ᑦᔾ",
    "ř		ᖨ	ᖩ	ᖪ	ᖫ	ᖬ	ᖭ	ᖮ",
    "v	ᕓ	ᕕ	ᕖ	ᕗ	ᕘ	ᕙ	ᕚ	ᕝ",
    "r	ᕂ	ᕆ	ᕇ	ᕈ	ᕉ	ᕋ	ᕌ	ᕐ",
    "q	ᙯ	ᕿ	ᖀ	ᖁ	ᖂ	ᖃ	ᖄ	ᖅ",
    "qq	ᖅᑫ	ᖅᑭ	ᖅᑮ	ᖅᑯ	ᖅᑰ	ᖅᑲ	ᖅᑳ	ᖅᒃ",
    "ng	ᙰ	ᖏ	ᖐ	ᖑ	ᖒ	ᖓ	ᖔ	ᖕ",
    "nng		ᙱ	ᙲ	ᙳ	ᙴ	ᙵ	ᙶ	ᖖ",
    "ł		ᖠ	ᖡ	ᖢ	ᖣ	ᖤ	ᖥ	ᖦ",
    "b								ᖯ",
    "h								ᕼ",
    "ʼ								ᑊ",
]


def escape_empty(s):
    if s == "":
        return None
    else:
        return s


def extract_series(table):
    col_ids = ["latin", "ai", "i", "ii", "u", "uu", "a", "aa", "final"]

    series = []
    for row in table:
        cols = row.split("\t")
        series.append({key: escape_empty(cols[i]) for i, key in enumerate(col_ids)})
    return series


def to_match(series_table):
    vowels = ["ai", "i", "ii", "u", "uu", "a", "aa"]

    to_latin_str = str()
    to_syllabic_str = str()

    for series in series_table:
        latin = series["latin"]
        for vowel in vowels:
            match series[vowel]:
                case str(syllabic):
                    if latin == None:
                        enum_s = f'\n{vowel.upper()} => '
                        to_syllabic_str += enum_s + f'"{vowel}",'
                    else:
                        enum_s = f'\n({latin.upper()}, {vowel.upper()}) => '
                        to_syllabic_str += enum_s + f'"{latin}{vowel}",'
                    to_latin_str += enum_s + f'"{syllabic}",'

    return to_latin_str + "\n" + to_syllabic_str


series_table = extract_series(table)
print(to_match(series_table))
