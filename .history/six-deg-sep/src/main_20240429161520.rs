mod graph;
use crate::graph::*;
use rand::Rng;

fn main() {
    
}

fn run_file(filename: &str, nodes: u32, test_iterations: u32, track_nodes: bool, print_adj_list: bool) {
    
    // tracking variables
    let mut graph = read_graph_from_file(filename, nodes).expect("Error reading graph");
    let mut paths_total: u32 = 0;
    let mut paths_found: u32 = 0;
    let mut longest_path: u32 = 0;

    for i in 1..=test_iterations {
        let mut rng = rand::thread_rng();

        let start_node = rng.gen_range(0..=nodes).to_string();
        let target_node = rng.gen_range(0..=nodes).to_string();

        match bfs_shortest(&graph.get_map(), &start_node, &target_node) {
            Ok(distance) => {

            }
            Err(_err) => 
        }
    }
}
