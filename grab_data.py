import psycopg2


def test(cur):
    cur.execute("SELECT * FROM gender")
    print(cur.fetchone())
with open('data.txt', 'r') as f:
    data = [item.rstrip() for item in f.readlines()]


conn = psycopg2.connect(
        database=data[0],
        host=data[1],
        port=data[2],
        user=data[3],
        password=data[4])

cur = conn.cursor()
