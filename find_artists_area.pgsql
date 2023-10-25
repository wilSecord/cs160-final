\o

BEGIN;

CREATE OR REPLACE FUNCTION find_us_state_of_area (area_id int) 
RETURNS INT
AS 
$$
DECLARE
    parent_rel RECORD;
    parent_id INT;
BEGIN
    for parent_rel in select entity0 from l_area_area where area_id=entity1
    loop 
        parent_id = parent_rel.entity0;

        if '222'=parent_id then 
            -- if the parent is the U.S. (222) then yield the area itself, which is defined to be a state
            return area_id; 
        else
            -- otherwise, recurse upwards
            return find_us_state_of_area(parent_id);
        end if;
    end loop;
    
    -- if we didn't return earlier (i.e. if there was no parent ID), 
    -- return 0. This is the negative base case.
    return 0;
end;
$$ LANGUAGE plpgsql;

COMMIT;

\pset format csv 
\o data/artist_states.csv

SELECT 
    artist.id as artist_id, 
    state_id 
FROM artist,
    LATERAL (SELECT find_us_state_of_area(artist.begin_area) AS state_id) l
WHERE 
    l.state_id <> 0
ORDER BY artist.id;