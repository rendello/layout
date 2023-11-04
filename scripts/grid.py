

l = []

for i in range(1, 25):
	l.append("key" + str(i))


syllabics = [
	"→",
	"ᐊ",
	"ᐸ",
	"ᑕ",
	"ᑲ",
	"←",
	"⭯",
	"ᒐ",
	"ᒪ",
	"ᓇ",
	"ᓴ",
	"⎵",
	"ABC",
	"ᓚ",
	"ᔭ",
	"ᕙ",
	"ᕋ",
	"🌐",
	"🎙",
	"ᖃ",
	"ᖓ",
	"ᑦᔭ",
	"ᖬ",
	"⮠",
	"ᖅᑲ",
	"ᙵ",
	"ᖤ",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	"",
	""
]

print('''<div style="grid-area: sugg;">Suggestions</div>''')
for i, item in enumerate(l):
	print(f'''<div class="button" style="grid-area: {item};">{syllabics[i]}</div>''')
