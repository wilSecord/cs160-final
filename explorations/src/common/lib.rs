use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Seek, Read},
    path::Path,
};

use petgraph::{prelude::NodeIndex, Graph, Directed};
pub use state_ids::UnitedStatesState;

mod state_ids;
pub mod shortest_path;
pub mod ad_hoc_shortest_path_file_format;

pub type ArtistRelationshipGraph = Graph<usize, (), Directed, u32>;

pub fn load_all_relations() -> Result<ArtistRelationshipGraph, Box<dyn Error>> {
    let mut graph = ArtistRelationshipGraph::default();
    load_artist_relations("../data/collaborations_work.csv", &mut graph)?;
    load_artist_relations("../data/collaborations_recording.csv", &mut graph)?;
    load_artist_relations("../data/artist_artist_relations.csv", &mut graph)?;
    
    Ok(graph)
}

pub fn load_artist_relations<P: AsRef<Path>>(
    path: P,
    graph: &mut ArtistRelationshipGraph
) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::open(path)?;


    //count newlines so that we can pre-reserve edges later
    let line_count = line_count(&mut file)?;
    file.rewind()?;

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    //ensure that the header is what we expect
    assert!(match lines.next() {
        Some(Ok(line)) => line == "artist1,collaboration_type,collaboration,artist2",
        _ => false,
    });

    //and consume the rest of the lines, adding them to the `graph` parameter
    let mut added_nodes: HashMap<usize, NodeIndex<u32>> = HashMap::new();
    graph.reserve_edges(line_count);

    for line in lines {
        let Ok(line) = line else { return  Err("Couldn't read line".into()); };
        let [artist_1, _, _, artist_2] = line.split(',').collect::<Vec<_>>()[..] else { return Err("Line doesn't contain 4 columns".into()) };

        let artist_1: usize = artist_1.parse()?;
        let artist_2: usize = artist_2.parse()?;

        let artist_1_key = if let Some(a1k) = added_nodes.get(&artist_1) {
            a1k.to_owned()
        } else {
            let a1k = graph.add_node(artist_1);
            added_nodes.insert(artist_1, a1k);
            a1k
        };

        let artist_2_key = if let Some(a2k) = added_nodes.get(&artist_2) {
            a2k.to_owned()
        } else {
            let a2k = graph.add_node(artist_2);
            added_nodes.insert(artist_2, a2k);
            a2k
        };

        graph.update_edge(artist_1_key, artist_2_key, ());
        graph.update_edge(artist_2_key, artist_1_key, ());
    }

    Ok(())
}

fn line_count(file: &mut std::fs::File) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for b in BufReader::new(file).bytes() {
        if let Ok(b'\n') = b { 
            count += 1;
        }
    }
    Ok(count)
}

pub fn add_states_to_graph(
    mut graph: ArtistRelationshipGraph,
    state_map: &HashMap<usize, UnitedStatesState>,
    make_bidirectional: bool,
) -> (
    ArtistRelationshipGraph,
    HashMap<UnitedStatesState, NodeIndex<u32>>,
) {
    let mut state_signifier_nodes = HashMap::new();

    for artistId in graph.node_indices() {
        let artist = graph.node_weight(artistId).unwrap();
        let Some(state) = state_map.get(artist) else {
            continue;
        };

        if let Some(stateId) = state_signifier_nodes.get(state) {
            graph.update_edge(*stateId, artistId, ());
        } else {
            let stateId = graph.add_node(0);

            graph.update_edge(stateId, artistId, ());

            if make_bidirectional {
                graph.update_edge(artistId, stateId, ());
            }

            state_signifier_nodes.insert(*state, stateId);
        }
    }

    (graph, state_signifier_nodes)
}

pub fn artist_names<P: AsRef<Path>>(path: P) -> Result<HashMap<usize, String>, Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open(path)?);

    let mut lines = reader.lines();

    //ensure that the header is what we expect
    assert!(match lines.next() {
        Some(Ok(line)) => line == "id,name",
        _ => false,
    });

    //and consume the rest! for each artist, record their state
    let mut map = HashMap::new();
    for line in lines {
        let Ok(line) = line else { return  Err("Couldn't read line".into()); };
        let [artist, name] = line.splitn(2, ',').collect::<Vec<_>>()[..] else { return Err("Line doesn't contain 2 columns".into()) };

        map.insert(artist.parse()?, name.to_string());
    }

    Ok(map)
}

pub fn artist_states<P: AsRef<Path>>(path: P) -> Result<HashMap<usize, UnitedStatesState>, Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open(path)?);

    let mut lines = reader.lines();

    //ensure that the header is what we expect
    assert!(match lines.next() {
        Some(Ok(line)) => line == "artist_id,state_id,area_gid",
        _ => false,
    });

    //and consume the rest! for each artist, record their state
    let mut map = HashMap::new();
    for line in lines {
        let Ok(line) = line else { return  Err("Couldn't read line".into()); };
        let [artist, state, _] = line.split(',').collect::<Vec<_>>()[..] else { return Err("Line doesn't contain 3 columns".into()) };

        map.insert(artist.parse()?, state.parse()?);
    }

    Ok(map)

}