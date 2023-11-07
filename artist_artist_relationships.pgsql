\o
\pset format csv 

\o data/artist_artist_relations.csv

SELECT
    laa.entity0 AS artist1,
    'aa' AS collaboration_type,
    link_type.id AS collaboration,
    laa.entity1 AS artist2
FROM
    l_artist_artist as laa
    JOIN link ON link.id = laa.link
    JOIN link_type ON link.link_type = link_type.id;

\o data/artist_relation_type_descriptions.csv 

SELECT DISTINCT 
    id, 
    name 
FROM 
    link_type
WHERE 
    entity_type0 = 'artist' AND entity_type1 = 'artist';
    
\o