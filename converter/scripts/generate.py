from pprint import pprint

def Series(t):
    d = {}
    for i, key in enumerate((
        "ai", "latin_ai",
        "i", "ii", "latin_i",
        "u", "uu", "latin_u",
        "a", "aa", "latin_a",
        "final", "latin_final",
        "ipa"
    )):
        d[key] = t[i]
    return d

table = [
    Series(row) for row in [
        ("ᐁ","ai","ᐃ","ᐄ","i","ᐅ","ᐆ","u","ᐊ","ᐋ","a",None,None,None),
        ("ᐯ","pai","ᐱ","ᐲ","pi","ᐳ","ᐴ","pu","ᐸ","ᐹ","pa","ᑉ","p","p"),
        ("ᑌ","tai","ᑎ","ᑏ","ti","ᑐ","ᑑ","tu","ᑕ","ᑖ","ta","ᑦ","t","t"),
        ("ᑫ","kai","ᑭ","ᑮ","ki","ᑯ","ᑰ","ku","ᑲ","ᑳ","ka","ᒃ","k","k"),
        ("ᕴ","hai","ᕵ","ᕶ","hi","ᕷ","ᕸ","hu","ᕹ","ᕺ","ha","ᕻ","h","h"),
        ("ᒉ","gai","ᒋ","ᒌ","gi","ᒍ","ᒎ","gu","ᒐ","ᒑ","ga","ᒡ","g","ɡ"),
        ("ᒣ","mai","ᒥ","ᒦ","mi","ᒧ","ᒨ","mu","ᒪ","ᒫ","ma","ᒻ","m","m"),
        ("ᓀ","nai","ᓂ","ᓃ","ni","ᓄ","ᓅ","nu","ᓇ","ᓈ","na","ᓐ","n","n"),
        ("ᓭ","sai","ᓯ","ᓰ","si","ᓱ","ᓲ","su","ᓴ","ᓵ","sa","ᔅ","s","s"),
        (None,None,"𑪶","𑪷","ši","𑪸","𑪹","šu","𑪺","𑪻","ša",None,"š","ʂ"),
        (None,None,"𑪰","𑪱","hi","𑪲","𑪳","hu","𑪴","𑪵","ha",None,"h","h"),
        ("ᓓ","lai","ᓕ","ᓖ","li","ᓗ","ᓘ","lu","ᓚ","ᓛ","la","ᓪ","l","l"),
        ("ᔦ","jai","ᔨ","ᔩ","ji","ᔪ","ᔫ","ju","ᔭ","ᔮ","ja","ᔾ","j","j"),
        ("ᑦᔦ","jjai","ᑦᔨ","ᑦᔩ","jji","ᑦᔪ","ᑦᔫ","jju","ᑦᔭ","ᑦᔮ","jja","ᑦᔾ","jj","jː"),
        (None,None,"ᖨ","ᖩ","ři","ᖪ","ᖫ","řu","ᖬ","ᖭ","řa","ᖮ","ř","ɟ"),
        ("ᕓ","vai","ᕕ","ᕖ","vi","ᕗ","ᕘ","vu","ᕙ","ᕚ","va","ᕝ","v","v"),
        ("ᕂ","rai","ᕆ","ᕇ","ri","ᕈ","ᕉ","ru","ᕋ","ᕌ","ra","ᕐ","r","ʁ"),
        ("ᙯ","qai","ᕿ","ᖀ","qi","ᖁ","ᖂ","qu","ᖃ","ᖄ","qa","ᖅ","q","q"),
        ("ᖅᑫ","qqai","ᖅᑭ","ᖅᑮ","qqi","ᖅᑯ","ᖅᑰ","qqu","ᖅᑲ","ᖅᑳ","qqa","ᖅᒃ","qq","qː"),
        ("ᙰ","ngai","ᖏ","ᖐ","ngi","ᖑ","ᖒ","ngu","ᖓ","ᖔ","nga","ᖕ","ng","ŋ"),
        (None,None,"ᙱ","ᙲ","nngi","ᙳ","ᙴ","nngu","ᙵ","ᙶ","nnga","ᖖ","nng","ŋː"),
        (None,None,"ᖠ","ᖡ","łi","ᖢ","ᖣ","łu","ᖤ","ᖥ","ła","ᖦ","ł","ɬ"),
        (None,None,None,None,None,None,None,None,None,None,None,"ᖯ","b","b"),
        (None,None,None,None,None,None,None,None,None,None,None,"ᕼ","h","h"),
        (None,None,None,None,None,None,None,None,None,None,None,"ᑊ","ʼ","ʔ")
    ]
]


def lengthen(s):
    if s != None:
        return s + s[-1]


def add_longs(series_table):
    new_series_table = []

    for series in series_table:
        new_series = series.copy()
        for key in ["latin_i", "latin_u", "latin_a"]:
            new_series[lengthen(key)] = lengthen(series[key])

        new_series_table.append(new_series)
    return new_series_table



def series_idents_add(series_table):
    previously_seen_idents = set()

    new_series_table = []
    for series in series_table:
        new_series = series.copy()

        for key in [
            "latin_ai",
            "latin_i", "latin_ii",
            "latin_u", "latin_uu",
            "latin_a", "latin_aa",
            "latin_final"
        ]:
            if series[key] != None:
                base_s = series[key].upper()
                s = base_s
                if base_s == "ʼ":
                    s = "STOP"
                for i in range(2, 10):
                    if s in previously_seen_idents:
                        s = base_s + str(i)
                    else:
                        break
                previously_seen_idents.add(s)
                new_series[key+"_ident"] = s
            else:
                new_series[key+"_ident"] = None

        new_series_table.append(new_series)
    return new_series_table


def enum(series_table):
    s = ""
    for series in series_table:
        line_s = ""
        for key in ["ai", "u", "uu", "i", "ii", "a", "aa", "final"]:
            res = series["latin_"+key+"_ident"]
            if res == None:
                line_s += " "*7
            else:
                line_s += (res +",").ljust(7)
        s += (" "*4)+line_s.rstrip()+"\n"
    s = s[:-2] # remove last comma and newline
    return f"enum Idents {{\n{s}\n}}"




print(enum(series_idents_add(add_longs(table))))
