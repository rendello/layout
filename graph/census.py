
import sqlite3


def as_unicode(l):
	return [s.encode("utf-8") for s in l]


def connect():
	connection = sqlite3.connect("census/db.sqlite3")
	connection.text_factory = bytes
	cursor = connection.cursor()
	return (connection, cursor)


def municipalities_with_inuktut_native_speakers(cursor):
	cur.execute("""SELECT GEO_NAME FROM canada_census_2021
				   WHERE CHARACTERISTIC_ID = "451"
				   AND GEO_LEVEL = "Census subdivision"
				   AND C1_COUNT_TOTAL > 0;""")

	return [t[0] for t in cur.fetchall()]


def municipality_data(cursor, municipality_name):
	d = {}

	for c_id in [1, 451, 453, 454, 455, 456]:
		cur.execute("""SELECT C1_COUNT_TOTAL FROM canada_census_2021
			           WHERE GEO_NAME = (?)
			           AND CHARACTERISTIC_ID = (?)""", [municipality_name, c_id])
	print(cur.fetchall())
	return d


if __name__ == "__main__":
	con, cur = connect()

	cur.execute("""SELECT C1_COUNT_TOTAL FROM canada_census_2021
				   WHERE CHARACTERISTIC_ID""")
	con.close()


# SELECT GEO_NAME, C1_COUNT_TOTAL from canada_census_2021 WHERE GEO_LEVEL = "Census subdivision" AND CHARACTERISTIC_ID = 452 AND C1_COUNT_TOTAL > 0;