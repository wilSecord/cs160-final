use std::{collections::HashMap, io::Write};

use explorations::{
    ad_hoc_shortest_path_file_format::FORMAT_NOTE, add_states_to_graph, artist_states,
    load_all_relations, shortest_path::dijkstras_algorithm_shortest_path_to_all, UnitedStatesState,
};

const OUT_FILE: &str = "../results/state_trees.txt";

fn main() {
    let graph = load_all_relations().unwrap();

    eprintln!(
        "Loaded graph: {} nodes, {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    //Load states, removing the Northern Mariana Islands and the Minor Outlying Islands,
    //since we know from previous explorations that they both have no collaborations.
    let state_map = artist_states("../data/artist_states.csv")
        .unwrap()
        .into_iter()
        .filter(|(_, state)| {
            *state != UnitedStatesState::NorthernMarianaIslands
                && *state != UnitedStatesState::UnitedStatesMinorOutlyingIslands
        })
        .collect();

    let (graph, state_signifier_nodes) = add_states_to_graph(graph, &state_map, false);

    eprintln!("Loaded states and inserted into graph");

    let inverse_state_map = state_signifier_nodes
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    let states = state_signifier_nodes.values().cloned().collect::<Vec<_>>();

    let mut output = std::fs::File::create(OUT_FILE).unwrap();

    output.write_all(FORMAT_NOTE.as_bytes()).unwrap();

    for state in states {
        eprintln!(
            "# Calculating shortness tree for {}",
            inverse_state_map[&state].name()
        );
        let short_tree = dijkstras_algorithm_shortest_path_to_all(&graph, state);

        eprintln!(
            "# Got shortness tree for {}",
            inverse_state_map[&state].name()
        );
        
        output.write_all(inverse_state_map[&state].name().as_bytes()).unwrap();
        output.write_all(&[b'[']).unwrap();

        write_usize_as_u32_le(&mut output, short_tree.len() + 1);

        let mut indexed_vec = Vec::with_capacity(short_tree.len() + 1);

        indexed_vec.push((0,0));

        for (k, v) in short_tree.iter() {
            write_usize_as_u32_le(&mut output, graph.node_weight(k).unwrap().clone());
            write_usize_as_u32_le(&mut output, (*v) + 1);
        }
    }
}

fn write_usize_as_u32_le(output: &mut std::fs::File, val: usize) {
    output.write_all(&usize_as_u32_le_bytes(val)[..]).unwrap();
}

fn usize_as_u32_le_bytes<'a>(val: usize) -> [u8; 4] {
    let val_as_u32 = TryInto::<u32>::try_into(val).unwrap();

    val_as_u32.to_le_bytes()
}