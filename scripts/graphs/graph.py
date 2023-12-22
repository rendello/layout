
import matplotlib.pyplot as plt
import numpy
import matplotlib

from inukdata import inukdata

municipalities = []
population = []
inuktut = []
inuinnaqtun = []
inuvialuktun = []
inuktitut = []
inuktut_nie = []

for municipality, data in inukdata.items():
	if data['inuktut'] / data['population'] >= 0.5:
		municipalities.append(municipality.split(",")[0])
		population.append(data['population'])
		inuktut.append(data['inuktut'])
		inuinnaqtun.append(data['Inuinnaqtun'])
		inuvialuktun.append(data['Inuvialuktun'])
		inuktitut.append(data['Inuktitut'])
		inuktut_nie.append(data['Inuktut (nie)'])

print(inuinnaqtun, inuvialuktun, inuktitut, inuktut_nie)



# x = range(len(population))

# bar_width = 0.8

# plt.figure()  # an empty figure with no Axes

# plt.bar(x, inuktut_nie, color='r', width=bar_width)
# # plt.bar(x, y2, bottom=y1, color='b')
# plt.xticks(x, municipalities, rotation=90)

# plt.tight_layout()
# plt.show()