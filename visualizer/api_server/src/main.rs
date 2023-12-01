use std::{collections::HashMap, sync::Arc};

mod prefix_trie;

use explorations::UnitedStatesState;
use warp::{reply::json, Filter};

#[tokio::main]
async fn main() {
    // let shortest_paths =
    //     explorations::ad_hoc_shortest_path_file_format::read("../../results/state_trees.txt")
    //         .unwrap()
    //         .into_iter()
    //         .collect::<Vec<_>>();
    let shortest_paths = Vec::<(UnitedStatesState, HashMap<u32, u32>)>::new();
    let names_o = Arc::new(explorations::artist_names("../../data/artist_names.csv").unwrap());

    let cors = warp::cors().allow_any_origin();

    let names = names_o.clone();

    let artist_search = warp::path!("api" / "artists" / "search" / String)
        .map(move |name: String| {
            let name = urlencoding::decode(&name).expect("UTF-8");
            eprintln!("{name}");
            json(
                &names
                    .clone()
                    .iter()
                    .filter_map(|(k, v)| {
                        if starts_ignore_case_ignore_whitespace(v, &name) {
                            Some((k, v))
                        } else {
                            None
                        }
                    })
                    .take(10)
                    .collect::<Vec<_>>(),
            )
        })
        .with(cors.clone());

    let names = names_o.clone();

    let get_name = warp::path!("api" / "artists" / usize / "name")
        .map(move |id| json(&names.get(&id).cloned().unwrap_or_default()))
        .with(cors.clone());

    let names = names_o.clone();

    let get_artist_paths = warp::path!("api" / "artists" / u32 / "paths")
        .map(move |id| {
            json(
                &shortest_paths
                    .iter()
                    .map(|(k, v)| {
                        let mut node: u32 = id;
                        let mut path = Vec::new();
                        for _ in 0..256 {
                            path.push(names.get(&(node as usize)).cloned().unwrap_or_default());
                            let Some(next) = v.get(&node) else { break };
                            node = *next;
                        }
                        (k.name().split(':').nth(2).unwrap(), path)
                    })
                    .collect::<HashMap<_, _>>(),
            )
        })
        .with(cors);

    warp::serve(artist_search.or(get_artist_paths).or(get_name))
        .run(([127, 0, 0, 1], 3020))
        .await;
}

fn starts_ignore_case_ignore_whitespace(string: &str, prefix: &str) -> bool {
    let mut pchrs = prefix.chars();
    for (ch, pch) in string.chars().zip(&mut pchrs) {
        if !pch.eq_ignore_ascii_case(&ch) {
            return false;
        }
    }
    //if we've exhausted the prefix, true!
    if pchrs.next().is_none() {
        return true;
    } else {
        return false;
    }
}
