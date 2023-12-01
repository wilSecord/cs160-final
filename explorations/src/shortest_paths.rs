use std::collections::{hash_map::RandomState, HashMap, HashSet};

use explorations::{
    artist_states, load_all_relations, load_artist_relations,
    shortest_path::dijkstras_algorithm_shortest_path_matrix, ArtistRelationshipGraph,
    UnitedStatesState, add_states_to_graph,
};
use petgraph::{
    algo::{connected_components, k_shortest_path, kosaraju_scc, tarjan_scc},
    prelude::{NodeIndex, Undirected},
};

fn main() {
    let graph = load_all_relations().unwrap();

    println!(
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

    let (graph, state_signifier_nodes) = add_states_to_graph(graph, &state_map, true);

    println!("Loaded states and inserted into graph");

    let inverse_state_map = state_signifier_nodes
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();
    let states = state_signifier_nodes.values().cloned().collect::<Vec<_>>();

    let shortest_paths = dijkstras_algorithm_shortest_path_matrix(&graph, &states);

    for ((start, end), path) in shortest_paths.iter() {
        let start_state = inverse_state_map[start];
        let end_state = inverse_state_map[end];

        println!(
            "Shortest path from {} to {} ({} nodes): {}",
            start_state.name(),
            end_state.name(),
            path.len(),
            path.iter()
                .filter_map(|x| graph.node_weight(*x).map(ToString::to_string))
                .fold(String::new(), |mut a, b| {
                    a.push_str(" <-> ");
                    a.push_str(&b);
                    a
                })
        )
    }
}

