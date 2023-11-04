

l = [
	[["sugg" for i in range(0, 8)], "29px"],
	[["→", "→", "ᐊ", "A", "ᐸ", "ᑕ", "ᑲ", "←"]],
	[["⭯", "⭯", "ᒐ", "A", "ᒪ", "ᓇ", "ᓴ", "⎵"]],
	[["ABC", "ABC", "ᓚ",  "A", "ᔭ", "ᕙ", "ᕋ", "⮠"]],
	[["🌐", "🎙", "ᖃ", "A", "ᖓ", "ᑦᔭ", "ᖬ", "⮠"]],
	["/ .5fr  .5fr  1fr 1fr   1fr   1fr   1fr   1fr"]
]

def get_key_ids(layout):
	key_ids = {}
	key_index = 0

	for row in layout:
		for item in row:
			if isinstance(item, list):
				for key_name in item:
					if key_name not in key_ids:
						key_ids[key_name] = f"key_{key_index}"
						key_index += 1
	return key_ids


def layout_css(layout):
	s = ""
	key_ids = get_key_ids(layout)

	for row in layout:
		for item in row:
			if isinstance(item, str):
				s += f" {item}"
			else:
				s += '"' + "".join([key_ids[key_name].ljust(7) for key_name in item]).strip() + '"'
		s += "\n"
	return s.strip()


def layout_html(layout, css_class_map):
	key_ids = get_key_ids(layout)
	s = ""

	for k, v in key_ids.items():
		if k in css_class_map:
			css_classes = css_class_map[k]
		else:
			css_classes = ["button"]

		s += f"""<div style="grid-area: {v};" class="{" ".join([c for c in css_classes]).strip()}">{k}</div>\n"""

	return s.strip()


css_class_map = {
	"sugg": []
}


print(layout_css(l))
print(layout_html(l, css_class_map))