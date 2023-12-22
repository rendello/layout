
import sqlite3

"""
SELECT GEO_NAME, C1_COUNT_TOTAL from canada_census_2021
WHERE GEO_LEVEL = "Census subdivision"
AND CHARACTERISTIC_ID = 452
AND C1_COUNT_TOTAL > 0;
"""

connection = sqlite3.connect("census/db.sqlite3")
connection.text_factory = bytes
cur = connection.cursor()

for c_id in [1, 451, 453, 454, 455, 456]:
	cur.execute("""SELECT CHARACTERISTIC_NAME FROM canada_census_2021
	           WHERE CHARACTERISTIC_ID = (?) LIMIT 1""", [c_id])
	print(cur.fetchall())