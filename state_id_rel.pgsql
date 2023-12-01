\pset format csv
\o data/state_ids.csv

SELECT id, name FROM area WHERE id BETWEEN 261 AND 311 OR id IN (172, 88, 159, 4)
