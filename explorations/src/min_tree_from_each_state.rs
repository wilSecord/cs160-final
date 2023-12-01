use std::{collections::HashMap, io::Write};

use explorations::{
    artist_states, load_all_relations,
    shortest_path::dijkstras_algorithm_shortest_path_to_all,
    UnitedStatesState, add_states_to_graph, ad_hoc_shortest_path_file_format::FORMAT_NOTE,
};
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

    print!("{}", FORMAT_NOTE);

    for state in states {
        eprintln!("# Calculating shortness tree for {}", inverse_state_map[&state].name());
        let short_tree = dijkstras_algorithm_shortest_path_to_all(&graph, state);


        eprintln!("# Got shortness tree for {}", inverse_state_map[&state].name());
        
        let mut nv_vec = short_tree.into_iter().collect::<Vec<_>>();

        let mut stdout = std::io::stdout().lock();

        write!(stdout, "{}[", inverse_state_map[&state].name()).unwrap();

        stdout.write_all(&(nv_vec.len() as u32).to_le_bytes()[..]).unwrap();

        nv_vec.sort_by_cached_key(|x| graph.node_weight(x.0).unwrap());
        for (node, prev) in nv_vec {
            let node = graph.node_weight(node).unwrap();
            let prev = graph.node_weight(prev).unwrap();

            write!(stdout, "{node},{prev},").unwrap();

            //stdout.write_all(&(*node as u32).to_le_bytes()[..]).unwrap();
            //stdout.write_all(&(*prev as u32).to_le_bytes()[..]).unwrap();
        }
    }
}
