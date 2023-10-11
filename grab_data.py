import psycopg2
import json


def grab_data(cur):
    cur.execute("SELECT name, area FROM artist WHERE area BETWEEN 262 AND 311")
    artist_area = cur.fetchall()

    artist_area_name = []
    for item in artist_area:
        cur.execute("SELECT name FROM area WHERE id = %s", (item[1],))
        area_name = cur.fetchone()
        artist_area_name.append((item[0], area_name))
    with open('../data.json', 'w+') as f:
        json.dump(artist_area_name, f)


with open('../data.txt', 'r') as f:
    data = [item.rstrip() for item in f.readlines()]


conn = psycopg2.connect(
        database=data[0],
        host=data[1],
        port=data[2],
        user=data[3],
        password=data[4])

cur = conn.cursor()
grab_data(cur)
