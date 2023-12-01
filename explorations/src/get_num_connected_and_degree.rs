use std::{ops::{Add, Div, Mul}, collections::HashMap};

use explorations::{ad_hoc_shortest_path_file_format, shortest_path::NodePath};

pub fn main() -> () {
    let trees = ad_hoc_shortest_path_file_format::read_to_nodepaths("../results/state_trees.txt").unwrap();

    println!("state,num_connected,mean_connection_degree,max_connection_degree,weighted_sum_linear,weighted_sum_quadratic");

    for (k,v) in trees {
        eprintln!("Processing {k}");

        let lengths = v.keys().map(|k| {
            length_from_given_node_to_root(&v, *k)
        });
        let (mean,max,wsl, wsq) = stats(lengths);
        println!("{},{},{},{},{},{}", k, v.len(),mean,max,wsl,wsq);
    }
}

fn length_from_given_node_to_root(map: &HashMap<u32, NodePath<u32>>, node: u32) -> usize {
    map[&node].size()
}

fn stats(vals: impl Iterator<Item = usize>) -> (usize,usize,f64,f64) {
    let mut avg = 0;
    let mut max = 0;
    let mut weighted_linear = 0.;
    let mut weighted_sum_quad = 0.;
    
    for (i,x) in vals.enumerate() {

        if x == 0 {continue;}

        avg = (avg * i + x) / (i + 1);
        max = max.max(x);

        weighted_linear += 1. / (x as f64);
        weighted_sum_quad += 1. / (x as f64 * x as f64);
    }
    (avg,max, weighted_linear, weighted_sum_quad)
}