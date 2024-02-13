
"""
Table adapted from Wikipedia article: "Inuktitut syllabics".
See: https://en.wikipedia.org/wiki/Inuktitut_syllabics
"""

"""
TODO:
Decide the strategy: should all series be included in the base table, given
the series is not equivocal to another (ie, different versions of the
H-series), or should the base table truly be the base, and the special
characters be in their own groups?

Ie. The "łii → ᖡ" conversion only applies to certain dialects technically,
but the conversion should be included (in both directions) for any dialect,
because neither the latin consonant nor the syllabic symbol are ambiguous.
The characters may be used in names, for example, and there's no point
leaving them out.

With the "cascading hashtables" idea, the "ł" series should be applied on
top of the base table, casting its shadow onto it. But the base table could
easily have this character, and that would allow it to exist in all dialects.
Another idea is to always have the base table not be the true root, and have
additional dialect tables trailing it. This seems wasteful.

Indeed, the idea of having multiple, cascading hashtables may be wasteful.
I'd like to consider having one table per dialect, and perhaps even two, one
with the ai column using the restored syllabics like ᐁ, and one version with
the ᐊᐃ. Having the cascading effect is tempting in this case, but I'd be
forced to create a ᐊᐃ or ᐁ for every version of the table, which just adds
work, confusion, and CPU cycles.

Really, the differentiation in dialects comes down to, as I understand it, the
H series (plural).

ᕼᐊ → Nunavut
𑪴 → Natsilik
ᓴ → Eastern Nunavut (but S in Western Nunavut)
ᕹ → Nunavik

I'd like to look at webscrapes and see which of these characters are commonly
used.

In any case, the cascading hashtable, if necessary, is simple and elegant.
I have my doubts that it's needed, however.

Perhaps the "patching" can just be user preferences? They don't have to be
perfect hashing, just a regular hashtable. For example, latin-izing the ᖓ
series as "ng" vs "ŋ".

TODO: Figure out what "ᐂ" is and add it, probably. 

===

Have tables of symbols that are converted from, but not too. For example,
𑪶 is preferred to ᓯ̵, the latter should be converted from, but not to. Likewise,
ł is preferred to ɫ in latin text. 

Number of bytes for keys? Just count the number of [backslash]x per line.

Reverse shadowing: have the basic table contain all unambiguous Inuktitut
mappings, then just have a hole for the ambiguous mappings (ie. H-series),
and place the appropriate table pointer behind. Then the happy path maps
to the most used operations (non-h series). Of course, in this case, the
non-syllabic stuff still goes through the two tables per char.

Or, instead of returning a simple Option<str> from the tables, I could
also return a "h-series" variant, then use the current dialect to
deal with that. Same with the h-s series. I'm matching anyways on the
Some/None case, it'd just be another case or two.
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