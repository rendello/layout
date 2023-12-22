


# for file_name in file_names:
# 	with open(f"census/{file_name}", encoding='latin-1') as f:
# 		for l in f:
# 			ls = l.split("\t")
# 			if ls[3] == "Census subdivision":

# 				col8 = ls[8]
# 				if col8 == "451" and try_int(ls[11]) > 0:
# 					print(f"\n{ls[11]} -- {ls[4]}")
# 					#print(l.strip())
# 				elif try_int(col8) > 452 and try_int(col8) < 457 and try_int(ls[11]) > 0:
# 					print(f"  {ls[11]} -- {lang(col8)}")

CENSUS_YEAR = 0
DGUID = 1
ALT_GEO_CODE = 2
GEO_LEVEL = 3
GEO_NAME = 4
TNR_SF = 5
TNR_LF = 6
DATA_QUALITY_FLAG = 7
CHARACTERISTIC_ID = 8
CHARACTERISTIC_NAME = 9
CHARACTERISTIC_NOTE = 10
C1_COUNT_TOTAL = 11
C1_COUNT_TOTAL_SYMBOL = 12
C2_COUNT_MEN = 13
C2_COUNT_MEN_SYMBOL = 14
C3_COUNT_WOMEN = 15
C3_COUNT_WOMEN_SYMBOL = 16
C10_RATE_TOTAL = 17
C10_RATE_TOTAL_SYMBOL = 18
C11_RATE_MEN = 19
C11_RATE_MEN_SYMBOL = 20
C12_RATE_WOMEN = 21
C12_RATE_WOMEN_SYMBOL = 22


def municipalities_with_inuktut_native_speakers(file_name):
	municipalities = []

	with open(f"census/{file_name}", encoding='latin-1') as file:
		for line in file:
			sections = line.split("\t")
			if sections[GEO_LEVEL] == "Census subdivision" and sections[CHARACTERISTIC_ID] == "451":
				try:
					if int(sections[C1_COUNT_TOTAL]) > 0:
						municipalities.append(sections[GEO_NAME])
				except ValueError:
					pass

	return municipalities


def municipality_data(file_name, municipality_names):
	d = {m: dict() for m in municipality_names}

	with open(f"census/{file_name}", encoding='latin-1') as file:
		for line in file:
			sections = line.split("\t")
			if sections[GEO_NAME] in municipality_names:
				match sections[CHARACTERISTIC_ID]:
					case "1":
						d[sections[GEO_NAME]]["population"] = int(sections[C1_COUNT_TOTAL])
					case "451":
						d[sections[GEO_NAME]]["inuktut"] = int(sections[C1_COUNT_TOTAL])
					case "453":
						d[sections[GEO_NAME]]["Inuinnaqtun"] = int(sections[C1_COUNT_TOTAL])
					case "454":
						d[sections[GEO_NAME]]["Inuvialuktun"] = int(sections[C1_COUNT_TOTAL])
					case "455":
						d[sections[GEO_NAME]]["Inuktitut"] = int(sections[C1_COUNT_TOTAL])
					case "456":
						d[sections[GEO_NAME]]["Inuktut (nie)"] = int(sections[C1_COUNT_TOTAL])
	return d




file_names = [
	"98-401-X2021006_English_TAB_data_Atlantic.TAB",
	"98-401-X2021006_English_TAB_data_BritishColumbia.TAB",
	"98-401-X2021006_English_TAB_data_Ontario.TAB",
	"98-401-X2021006_English_TAB_data_Prairies.TAB",
	"98-401-X2021006_English_TAB_data_Quebec.TAB",
	"98-401-X2021006_English_TAB_data_Territories.TAB"
]

d = {}
for file_name in reversed(file_names): 
	ms = municipalities_with_inuktut_native_speakers(file_name)
	d.update(municipality_data(file_name, ms))
print(d)
