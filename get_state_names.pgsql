
\pset format csv 
\o data/state_names.csv

SELECT 
    area.id, 
    name 
FROM l_area_area
JOIN area ON l_area_area.entity1=area.id
WHERE l_area_area.entity0 = '222';