
def Syllabic(t):
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
    Syllabic(row) for row in (
    	("ᐁ","ai","ᐃ","ᐄ","i","ᐅ","ᐆ","u","ᐊ","ᐋ","a","","",""),
    	("ᐯ","pai","ᐱ","ᐲ","pi","ᐳ","ᐴ","pu","ᐸ","ᐹ","pa","ᑉ","p","p"),
    	("ᑌ","tai","ᑎ","ᑏ","ti","ᑐ","ᑑ","tu","ᑕ","ᑖ","ta","ᑦ","t","t"),
    	("ᑫ","kai","ᑭ","ᑮ","ki","ᑯ","ᑰ","ku","ᑲ","ᑳ","ka","ᒃ","k","k"),
    	("ᕴ","hai","ᕵ","ᕶ","hi","ᕷ","ᕸ","hu","ᕹ","ᕺ","ha","ᕻ","h","h"),
    	("ᒉ","gai","ᒋ","ᒌ","gi","ᒍ","ᒎ","gu","ᒐ","ᒑ","ga","ᒡ","g","ɡ"),
    	("ᒣ","mai","ᒥ","ᒦ","mi","ᒧ","ᒨ","mu","ᒪ","ᒫ","ma","ᒻ","m","m"),
    	("ᓀ","nai","ᓂ","ᓃ","ni","ᓄ","ᓅ","nu","ᓇ","ᓈ","na","ᓐ","n","n"),
    	("ᓭ","sai","ᓯ","ᓰ","si","ᓱ","ᓲ","su","ᓴ","ᓵ","sa","ᔅ","s","s"),
    	("","","𑪶","𑪷","ši","𑪸","𑪹","šu","𑪺","𑪻","ša","","š","ʂ"),
    	("","","𑪰","𑪱","hi","𑪲","𑪳","hu","𑪴","𑪵","ha","","h","h"),
    	("ᓓ","lai","ᓕ","ᓖ","li","ᓗ","ᓘ","lu","ᓚ","ᓛ","la","ᓪ","l","l"),
    	("ᔦ","jai","ᔨ","ᔩ","ji","ᔪ","ᔫ","ju","ᔭ","ᔮ","ja","ᔾ","j","j"),
    	("ᑦᔦ","jjai","ᑦᔨ","ᑦᔩ","jji","ᑦᔪ","ᑦᔫ","jju","ᑦᔭ","ᑦᔮ","jja","ᑦᔾ","jj","jː"),
    	("","","ᖨ","ᖩ","ři","ᖪ","ᖫ","řu","ᖬ","ᖭ","řa","ᖮ","ř","ɟ"),
    	("ᕓ","vai","ᕕ","ᕖ","vi","ᕗ","ᕘ","vu","ᕙ","ᕚ","va","ᕝ","v","v"),
    	("ᕂ","rai","ᕆ","ᕇ","ri","ᕈ","ᕉ","ru","ᕋ","ᕌ","ra","ᕐ","r","ʁ"),
    	("ᙯ","qai","ᕿ","ᖀ","qi","ᖁ","ᖂ","qu","ᖃ","ᖄ","qa","ᖅ","q","q"),
    	("ᖅᑫ","qqai","ᖅᑭ","ᖅᑮ","qqi","ᖅᑯ","ᖅᑰ","qqu","ᖅᑲ","ᖅᑳ","qqa","ᖅᒃ","qq","qː"),
    	("ᙰ","ngai","ᖏ","ᖐ","ngi","ᖑ","ᖒ","ngu","ᖓ","ᖔ","nga","ᖕ","ng","ŋ"),
    	("","","ᙱ","ᙲ","nngi","ᙳ","ᙴ","nngu","ᙵ","ᙶ","nnga","ᖖ","nng","ŋː"),
    	("","","ᖠ","ᖡ","łi","ᖢ","ᖣ","łu","ᖤ","ᖥ","ła","ᖦ","ł","ɬ"),
    	("","","","","","","","","","","","ᖯ","b","b"),
    	("","","","","","","","","","","","ᕼ","h","h"),
    	("","","","","","","","","","","","ᑊ","ʼ","ʔ")
    )
]

def idents(table):
    syllabics_idents = []
    for syllabic in table:
        idents = []

        idents.append(syllabic["latin_ai"].upper())

        for key in ("latin_u", "latin_i", "latin_a"):
            s = syllabic[key].upper()
            idents.append(s)            
            if s != "":
                idents.append(s + s[-1])
            else:
                idents.append("")

        idents.append(syllabic["latin_final"].upper())

        syllabics_idents.append(idents)

    return unique_idents(syllabics_idents)


def unique_idents(idents):
    seen_idents = []
    new_idents = []
    for row in idents:
        new_row = []
        for ident in row:
            if ident != "" and ident in seen_idents:
                s = ident + "2"
                if s in seen_idents:
                    s = s[:-1] + "3"
            else:
                s = ident

            seen_idents.append(s)
            new_row.append(s)
        new_idents.append(new_row)
    return new_idents




text = ""
for ident in idents(table):
    line_text = ""
    for item in ident:
        if item == "ʼ":
            s = "STOP"
        elif item == "":
            s = ""
        else:
            s = item + ","

        line_text += s.ljust(7)
    text += line_text.rstrip() + "\n"

print(text)

# for item in table:
#     print(item)