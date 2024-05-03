mod graph;
use crate::graph::*;
use rand::Rng;

fn main() {
    
}

fn run_file(filename: &str, nodes: u32, test_iterations: u32, track_nodes: bool, print_adj_list: bool) {
    let mut graph = read_graph_from_file(filename, nodes).expect("Error reading graph");
    let mut paths_total = 0;
}
