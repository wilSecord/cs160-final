\o
\pset format csv 

\o data/collaborations_recording.csv
SELECT
    l_artist_recording1.entity0 AS artist1,
    'rc' AS collaboration_type,
    min(l_artist_recording1.entity1) AS collaboration,
    l_artist_recording2.entity0 AS artist2
FROM
    l_artist_recording AS l_artist_recording1
    JOIN l_artist_recording AS l_artist_recording2 ON l_artist_recording2.entity1 = l_artist_recording1.entity1
WHERE
    -- remove links in reverse & self-links
    l_artist_recording1.entity0 < l_artist_recording2.entity0
GROUP BY
    artist1,
    artist2;

\o data/collaborations_work.csv
SELECT
    l_artist_work1.entity0 AS artist1,
    'wk' AS collaboration_type,
    min(l_artist_work1.entity1) AS collaboration,
    l_artist_work2.entity0 AS artist2
FROM
    l_artist_work AS l_artist_work1
    JOIN l_artist_work AS l_artist_work2 ON l_artist_work1.entity1 = l_artist_work2.entity1
WHERE
    -- remove links in reverse & self-links
    l_artist_work1.entity0 < l_artist_work2.entity0
GROUP BY
    artist1,
    artist2;