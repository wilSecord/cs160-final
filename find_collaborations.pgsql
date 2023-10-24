SELECT DISTINCT
    l_artist_recording1.entity0 AS artist1,
    'recording' AS collaboration_type,
    l_artist_recording1.entity1 AS collaboration,
    l_artist_recording2.entity0 AS artist2
FROM
    artist
    JOIN l_artist_recording AS l_artist_recording1 ON l_artist_recording1.entity0 = artist.id
    JOIN l_artist_recording AS l_artist_recording2 ON l_artist_recording2.entity1 = l_artist_recording1.entity1
UNION
SELECT DISTINCT
    l_artist_work1.entity0 AS artist1,
    'work' AS collaboration_type,
    l_artist_work1.entity1 AS collaboration,
    l_artist_work2.entity0 AS artist2
FROM
    artist
    JOIN l_artist_work AS l_artist_work1 ON l_artist_work1.entity0 = artist.id
    JOIN l_artist_work AS l_artist_work2 ON l_artist_work2.entity1 = l_artist_work2.entity1;