
"""
Table adapted from Wikipedia article: "Inuktitut syllabics".
See: https://en.wikipedia.org/wiki/Inuktitut_syllabics
"""

BASE_TABLE = [
    "	ᐁ	ᐃ	ᐄ	ᐅ	ᐆ	ᐊ	ᐋ	",
    "p	ᐯ	ᐱ	ᐲ	ᐳ	ᐴ	ᐸ	ᐹ	ᑉ",
    "t	ᑌ	ᑎ	ᑏ	ᑐ	ᑑ	ᑕ	ᑖ	ᑦ",
    "k	ᑫ	ᑭ	ᑮ	ᑯ	ᑰ	ᑲ	ᑳ	ᒃ",
    "g	ᒉ	ᒋ	ᒌ	ᒍ	ᒎ	ᒐ	ᒑ	ᒡ",
    "m	ᒣ	ᒥ	ᒦ	ᒧ	ᒨ	ᒪ	ᒫ	ᒻ",
    "n	ᓀ	ᓂ	ᓃ	ᓄ	ᓅ	ᓇ	ᓈ	ᓐ",
    "s	ᓭ	ᓯ	ᓰ	ᓱ	ᓲ	ᓴ	ᓵ	ᔅ",
    "l	ᓓ	ᓕ	ᓖ	ᓗ	ᓘ	ᓚ	ᓛ	ᓪ",
    "j	ᔦ	ᔨ	ᔩ	ᔪ	ᔫ	ᔭ	ᔮ	ᔾ",
    "jj	ᑦᔦ	ᑦᔨ	ᑦᔩ	ᑦᔪ	ᑦᔫ	ᑦᔭ	ᑦᔮ	ᑦᔾ",
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

NUNAVIK_TABLE = [
    "h	ᕴ	ᕵ	ᕶ	ᕷ	ᕸ	ᕹ	ᕺ	ᕻ",
]

NATSILIK_TABLE = [
    "š		𑪶	𑪷	𑪸	𑪹	𑪺	𑪻	",
    "h		𑪰	𑪱	𑪲	𑪳	𑪴	𑪵	",
    "ř		ᖨ	ᖩ	ᖪ	ᖫ	ᖬ	ᖭ	ᖮ",
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


def encode(s):
    return repr(s.encode("utf8"))[2:][:-1]


def to_syl_line(consonant, vowel, syllabic):
    line = f'''    b"{encode(consonant) or ""}{vowel or ""}" => "{syllabic}",'''
    if "\\x" in line:
        line += f'''\t/* {consonant}{vowel or ""} */\n'''
    else:
        line += "\n"
    return line


def to_lat_line(consonant, vowel, syllabic):
    line = f'''    b"{encode(syllabic)}" => "{consonant or ""}{vowel or ""}",'''
    if "\\x" in line:
        line += f'''\t/* {syllabic} */\n'''
    else:
        line += "\n"
    return line



def generate_hashtables(series_table, line_function):
    s = str()

    for series in series_table:
        consonant = series["latin"] or ""

        match series["final"]:
            case str(syllabic):
                s += line_function(consonant, None, syllabic) 

        for vowel in ["ai", "i", "ii", "u", "uu", "a", "aa"]:
            match series[vowel]:
                case str(syllabic):
                    s += line_function(consonant, vowel, syllabic) 
    return s


def to_syl(series_table):
    return generate_hashtables(series_table, to_syl_line)

def to_lat(series_table):
    return generate_hashtables(series_table, to_lat_line)


# TODO: Count max bytes for syllabics and latin letters for max key lengths.

dialects = [
    ("BASE", BASE_TABLE),
    ("NUNAVIK", NUNAVIK_TABLE),
    ("NATSILIK", NATSILIK_TABLE)
]


def realign_comments(s):
    buffer = ""
    max_left_length = 0

    for line in s.split("\n"):
        if "\\x" in line and "\t" in line:
            left, _ = line.split("\t")
            if len(left) > max_left_length:
                max_left_length = len(left)

    for line in s.split("\n"):
        if "\\x" in line and "\t" in line:
            left, right = line.split("\t")

            right_aligned = " " * ((max_left_length // 4 + 1) * 4 - len(left)) + right
            buffer += left + right_aligned + "\n"
        else:
            buffer += line + "\n"

    return buffer


def generate():
    buffer = (
        '''/* =========================================== */\n'''
        '''/* AUTOGENERATED: See `../scripts/generate.py` */\n'''
        '''/* =========================================== */\n\n'''
        '''use phf::phf_map;\n\n'''
        '''type PMap = phf::Map<&'static [u8], &'static str>;\n\n'''
    )

    for (suffix, function) in ("_TO_SYL", to_syl), ("_TO_LAT", to_lat):
        local_buffer = ""
        for (dialect, table) in dialects:
            series_table = extract_series(table)

            local_buffer += "pub static " + dialect + suffix + ": PMap = phf_map! {\n"
            local_buffer += function(series_table)
            local_buffer += "};\n\n"

        buffer += realign_comments(local_buffer)

    return buffer


print(realign_comments(generate()))