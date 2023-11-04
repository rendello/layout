text = """
| ᐁ | ai | ᐃ | ᐄ | i | ᐅ | ᐆ | u | ᐊ | ᐋ | a |  |  |  |
| ᐯ | pai | ᐱ | ᐲ | pi | ᐳ | ᐴ | pu | ᐸ | ᐹ | pa | ᑉ | p | / p / |
| ᑌ | tai | ᑎ | ᑏ | ti | ᑐ | ᑑ | tu | ᑕ | ᑖ | ta | ᑦ | t | / t / |
| ᑫ | kai | ᑭ | ᑮ | ki | ᑯ | ᑰ | ku | ᑲ | ᑳ | ka | ᒃ | k | / k / |
| ᕴ | hai | ᕵ | ᕶ | hi | ᕷ | ᕸ | hu | ᕹ | ᕺ | ha | ᕻ | h  | / h / |
| ᒉ | gai | ᒋ | ᒌ | gi | ᒍ | ᒎ | gu | ᒐ | ᒑ | ga | ᒡ | g | / ɡ / - / ɣ / |
| ᒣ | mai | ᒥ | ᒦ | mi | ᒧ | ᒨ | mu | ᒪ | ᒫ | ma | ᒻ | m | / m / |
| ᓀ | nai | ᓂ | ᓃ | ni | ᓄ | ᓅ | nu | ᓇ | ᓈ | na | ᓐ | n | / n / |
| ᓭ | sai | ᓯ | ᓰ | si/hi | ᓱ | ᓲ | su/hu | ᓴ | ᓵ | sa/ha | ᔅ | s/h  | / s / - / h / |
|  |  | 𑪶 | 𑪷 | ši | 𑪸 | 𑪹 | šu | 𑪺 | 𑪻 | ša |  | š  | / ʂ / |
|  |  | 𑪰 | 𑪱 | hi | 𑪲 | 𑪳 | hu | 𑪴 | 𑪵 | ha |  | h  | / h / |
| ᓓ | lai | ᓕ | ᓖ | li | ᓗ | ᓘ | lu | ᓚ | ᓛ | la | ᓪ | l | / l / |
| ᔦ | jai | ᔨ | ᔩ | ji | ᔪ | ᔫ | ju | ᔭ | ᔮ | ja | ᔾ | j | / j / |
| ᑦᔦ | jjai | ᑦᔨ | ᑦᔩ | jji | ᑦᔪ | ᑦᔫ | jju | ᑦᔭ | ᑦᔮ | jja | ᑦᔾ | jj | / j ː / |
|  |  | ᖨ | ᖩ | ři | ᖪ | ᖫ | řu | ᖬ | ᖭ | řa | ᖮ | ř  | / ɟ / |
| ᕓ | vai | ᕕ | ᕖ | vi | ᕗ | ᕘ | vu | ᕙ | ᕚ | va | ᕝ | v | / v / |
| ᕂ | rai | ᕆ | ᕇ | ri | ᕈ | ᕉ | ru | ᕋ | ᕌ | ra | ᕐ | r | / ʁ / |
| ᙯ | qai | ᕿ | ᖀ | qi | ᖁ | ᖂ | qu | ᖃ | ᖄ | qa | ᖅ | q | / q / |
| ᖅᑫ | qqai | ᖅᑭ | ᖅᑮ | qqi | ᖅᑯ | ᖅᑰ | qqu | ᖅᑲ | ᖅᑳ | qqa | ᖅᒃ | qq  | / q ː / |
| ᙰ | ngai | ᖏ | ᖐ | ngi | ᖑ | ᖒ | ngu | ᖓ | ᖔ | nga | ᖕ | ng  | / ŋ / |
|  |  | ᙱ | ᙲ | nngi | ᙳ | ᙴ | nngu | ᙵ | ᙶ | nnga | ᖖ | nng  | / ŋ ː / |
|  |  | ᖠ | ᖡ | łi | ᖢ | ᖣ | łu | ᖤ | ᖥ | ła | ᖦ | ł  | / ɬ / |
|  |  |  |  |  |  |  |  |  |  |  | ᖯ | b  | / b / |
|  |  |  |  |  |  |  |  |  |  |  | ᕼ | h  | / h / |
|  |  |  |  |  |  |  |  |  |  |  | ᑊ | ʼ | / ʔ / |
"""

import json

def n(s):
	isinstance(s, str)
	if s == "":
		return None
	else:
		return s

big_d = dict()

lines = text.strip().split("\n")
for line in lines:
	items = [s.strip() for s in line.split("|")]
	#print(items)

	t = [n(item) for item in [
		items[1],
		items[3],
		items[4],
		items[6],
		items[7],
		items[9],
		items[10],
		items[12],
		items[13]
	]]
	#print(t)

	d = {
		"final": t[7],
		"short": {
			"0": t[5],
			"1": t[1],
			"2": t[3],
			"3": t[0]
		},
		"long": {
			"0": t[6],
			"1": t[2],
			"2": t[4],
			"3": None
		}
	}
	big_d[t[8]] = d

print(json.dumps(big_d, ensure_ascii=False, indent=4))
for k, v in big_d.items():
	print(v.get("short").get("0"))