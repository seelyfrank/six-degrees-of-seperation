mod graph;
use crate::graph::*;
use rand::Rng;

fn main() {
    run_file("datasets/directed.txt", 8, 500, false, false);
    run_file("datasets/directed_connected.txt", 8, 500, true, false);
    run_file("datasets/amazon.txt")
}

// driver function
fn run_file(filename: &str, nodes: u32, test_iterations: u32, track_nodes: bool, print_adj_list: bool) -> (u32, f32, f32, u32) {
    
    println!("-----------------------------------------------------------");
    println!("FILENAME: {filename}");
    println!("-----------------------------------------------------------");

    // tracking variables
    let mut graph = read_graph_from_file(filename).expect("Error reading graph");
    let mut paths_total: u32 = 0;
    let mut paths_found: u32 = 0;
    let mut longest_path: u32 = 0;

    if track_nodes { println!("Iterations\n"); }
    for i in 1..=test_iterations {
        let mut rng = rand::thread_rng();

        let start_node = rng.gen_range(0..=nodes).to_string();
        let target_node = rng.gen_range(0..=nodes).to_string();

        match bfs_shortest(&graph.get_map(), &start_node, &target_node) {
            Ok(distance) => {
                if track_nodes { println!("Shortest path distance from {} to {} is {}", start_node, target_node, distance); }

                paths_total += distance as u32;
                paths_found += 1;

                if distance as u32 > longest_path { longest_path = distance as u32; }
            }
            Err(_err) => {
                if track_nodes { println!("No path found from node {} to {} - iteration {}", start_node, target_node, i); }
            }
        }
    }
    if track_nodes {println!("-----------------------------------------------------------");}

    let avg_len: f32 = paths_total as f32 / paths_found as f32;
    let connected: f32 = paths_found as f32 / test_iterations as f32 * 100.;

    

    println!("Total paths found: {paths_found}");
    println!("Percent node-pairs connected to one another: {connected}%");
    println!("Average path length: {avg_len}");
    println!("Longest path length: {longest_path}");
    println!("-----------------------------------------------------------");

    if print_adj_list { graph.print_adjacency_list(); }

    (paths_found, connected, avg_len, longest_path)

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let filename = "datasets/directed.txt"; 
        let nodes = 9;
        let test_iterations = 500;
        let track_nodes = false;
        let print_adj_list = false;

        run_file(filename, nodes, test_iterations, track_nodes, print_adj_list);
    }

    #[test]
    fn check_run_file_values() {
        let filename = "datasets/directed_connected.txt"; 
        let nodes = 8;
        let test_iterations = 500;
        let track_nodes = false;
        let print_adj_list = false;

        let results = run_file(filename, nodes, test_iterations, track_nodes, print_adj_list);
        let (paths_found, connected, _avg_len, longest_path) = results;

        assert_eq!(longest_path, 6);
        assert_eq!(paths_found, 500);
        assert_eq!(connected, 100.);
    }
}
