from dataclasses import dataclass

# TODO: Change KEY_LENGTHS to static from const?

DIALECTS = [
    {
        "name": "Base",
        "reciprocal": [
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
            "ř		ᖨ	ᖩ	ᖪ	ᖫ	ᖬ	ᖭ	ᖮ",
            "š		𑪶	𑪷	𑪸	𑪹	𑪺	𑪻	",
            "b								ᖯ",
            "h								ᕼ",
            "ʼ								ᑊ",
        ],
        "to_lat": [
            "sh		ᓯ̵	ᓰ̵	ᓱ̵	ᓲ̵	ᓴ̵	ᓵ̵	ᔅ̵",
            "h		ᓯˋ	ᓰˋ	ˎᓱ	ˎᓲ	ᓴˏ	ᓵˏ	ᔅ̷",
        ],
        "to_syl": [
            "ɫ		ᖠ	ᖡ	ᖢ	ᖣ	ᖤ	ᖥ	ᖦ",
            "&		ᖠ	ᖡ	ᖢ	ᖣ	ᖤ	ᖥ	ᖦ",
        ]
    },
    {
        "name": "Nunavik",
        "reciprocal": [
            "h	ᕴ	ᕵ	ᕶ	ᕷ	ᕸ	ᕹ	ᕺ	ᕻ",
        ]
    }
]

VOWELS = ["ai", "i", "ii", "u", "uu", "a", "aa", ""]

@dataclass
class Pair:
    latin: str
    latin_escaped: str
    latin_length: int
    syllabic: str
    syllabic_escaped: str
    syllabic_length: int


def extract_series(table):
    COLUMN_IDS = ["consonant"] + VOWELS 

    series = []
    for row in table:
        columns = row.split("\t")
        if len(columns) != len(COLUMN_IDS):
            raise Exception("Header length != row length. Use tabs, not spaces.\n"+str(cols))

        series.append({key: columns[i] for i, key in enumerate(COLUMN_IDS)})
    return series


def utf8_escape(s):
    escaped = s.encode("utf8")
    return len(escaped), repr(escaped)[2:][:-1]


def pairs(series):
    pairs = []
    for vowel in VOWELS:
        latin = series["consonant"] + vowel
        syllabic = series[vowel]

        latin_length, latin_escaped = utf8_escape(latin)
        syllabic_length, syllabic_escaped = utf8_escape(syllabic)

        if syllabic_length > 0:
            pairs.append(Pair(
                latin, latin_escaped, latin_length,
                syllabic, syllabic_escaped, syllabic_length
            ))

    return pairs


def to_lat_line(pair):
    return f'''    b"{pair.syllabic_escaped}" => "{pair.latin}",\t/* {pair.syllabic} */\n'''


def to_syl_line(pair):
    line = f'''    b"{pair.latin_escaped}" => "{pair.syllabic}",'''
    if pair.latin_escaped != pair.latin:
        line += f"\t/* {pair.latin} */"
    return line + "\n"


def line(direction, pair):
    match direction:
        case "to_lat":
            return to_lat_line(pair)
        case "to_syl":
            return to_syl_line(pair)


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


def generate_key_lengths(direction, key_lengths):
    lengths_s = sorted(list(key_lengths), reverse=True)
    count = len(key_lengths)
    return f'''pub const KEY_LENGTHS_{direction.upper()}: [usize; {count}] = {lengths_s};\n'''


def do(direction):
    buffer = ""
    key_lengths = set()

    for dialect in DIALECTS:
        buffer += "\npub static " + (dialect["name"] + "_" + direction).upper() + ": PMap = phf_map! {\n"

        for table, comment in [
            ("reciprocal", ""),
            (direction, "\n    /* UNIDIRECTIONAL NORMALIZATIONS */\n")
        ]:
            if dialect.get(table):
                buffer += comment
                for series in extract_series(dialect[table]):
                    for pair in pairs(series):
                        buffer += line(direction, pair)

                        match direction:
                            case "to_syl":
                                key_lengths.add(pair.latin_length)
                            case "to_lat":
                                key_lengths.add(pair.syllabic_length)
        buffer += "};\n"

    buffer = realign_comments(buffer)
    key_lengths_str = generate_key_lengths(direction, key_lengths)
    return buffer, key_lengths_str




'''
pub struct Dialect {
    maps: &'static [&'static PMap],
    key_lengths: &'static [usize],
}

static NUNAVUT: [&'static PMap; 2]  = [&BASE_TO_SYL, &IQALUIT_TO_SYL];
static NUNAVIK: [&'static PMap; 2]  = [&BASE_TO_SYL, &NUNAVIK_TO_SYL];
static NETSILIK: [&'static PMap; 2] = [&BASE_TO_SYL, &NETSILIK_TO_SYL];

let d = Dialect {
    maps: &M,
    key_lengths: &maps::KEY_LENGTHS_TO_SYL,
};
'''


def generate():
    header = (
        '''/* =========================================== */\n'''
        '''/* AUTOGENERATED: See `../scripts/generate.py` */\n'''
        '''/* =========================================== */\n\n'''
        '''use phf::phf_map;\n\n'''
        '''type PMap = phf::Map<&'static [u8], &'static str>;\n\n'''
    )

    maps_to_syl, key_lengths_to_syl = do("to_syl")
    maps_to_lat, key_lengths_to_lat = do("to_lat")

    buffer = "".join([
        header,
        key_lengths_to_syl,
        key_lengths_to_lat,
        maps_to_syl,
        maps_to_lat
    ]).strip()

    return buffer

print(generate())