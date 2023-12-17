use std::{collections::HashMap, sync::Arc};

mod prefix_trie;

use explorations::{UnitedStatesState, shortest_path};
use prefix_trie::PrefixTrie;
use warp::{reply::json, Filter};

#[tokio::main]
async fn main() {
    let shortest_paths = explorations::ad_hoc_shortest_path_file_format::read("../../results/state_trees.txt").unwrap();

        for _ in shortest_paths.take(10) {

        }

        return ();

        let shortest_paths: Vec<_> = explorations::ad_hoc_shortest_path_file_format::read("../../results/state_trees.txt").unwrap().collect();

    let names_o = Arc::new(explorations::artist_names("../../data/artist_names.csv").unwrap());
    let name_search_tree = Arc::new(
        names_o
            .clone()
            .iter()
            .map(|(id, name)| {
                (
                    name.chars().filter_map(|mut x| {
                        if x.is_ascii_alphanumeric() {
                            x.make_ascii_lowercase();
                            Some(x)
                        } else {
                            None
                        }
                    }),
                    id.to_owned(),
                )
            })
            .collect::<PrefixTrie<_, _>>(),
    );

    let cors = warp::cors().allow_any_origin();

    let names = names_o.clone();

    let artist_search = warp::path!("api" / "artists" / "search" / String)
        .map(move |name: String| {
            let name = urlencoding::decode(&name).expect("UTF-8");
            eprintln!("{name}");
            json(
                &name_search_tree
                    .prefix_values(name.chars())
                    .take(10)
                    .map(|id| (id, names[id].clone()))
                    .collect::<Vec<_>>(),
            )
        })
        .with(cors.clone());

    let names = names_o.clone();

    let get_name = warp::path!("api" / "artists" / usize / "name")
        .map(move |id| json(&names.get(&id).cloned().unwrap_or_default()))
        .with(cors.clone());

    let names = names_o.clone();

    eprintln!("there are {} states", shortest_paths.len());

    let get_artist_paths = warp::path!("api" / "artists" / u32 / "paths")
        .map(move |id| {
            json(
                &shortest_paths
                    .iter()
                    .map(|(k, v)| {
                        let path = v.path_as_vec(&id);
                        (k.name().split(':').nth(2).unwrap(), path)
                    })
                    .collect::<HashMap<_, _>>(),
            )
        })
        .with(cors);

    eprintln!("Serving!");

    let static_files = warp::fs::dir("../../static");

    warp::serve(artist_search.or(get_artist_paths).or(get_name).or(static_files))
        .run(([127, 0, 0, 1], 3020))
        .await;
}
