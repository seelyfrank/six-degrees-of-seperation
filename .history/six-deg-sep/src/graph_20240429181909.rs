use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]

#[derive(PartialEq)]
pub enum BFSError {
    NodeNotFound,
}

pub struct Graph(HashMap<String, Vec<String>>);

impl Graph {
    pub fn new(nodes: u32) -> Self {
        Graph(HashMap::new())
    }

    pub fn add_edge(&mut self, from_node: String, to_node: String) {
        self.0.entry(from_node).or_insert_with(Vec::new).push(to_node);
    }

    pub fn get_map(&mut self) -> &mut HashMap<String, Vec<String>> {
        &mut self.0
    }

    pub fn print_adjacency_list(&self) {
        for (node, neighbors) in &self.0 {
            println!("{}: {:?}", node, neighbors);
        }
    }
}

pub fn bfs_shortest(graph: &HashMap<String, Vec<String>>, start_node: &str, target_node: &str) -> Result<usize, BFSError> {
    
    // used to keep track of nodes and distance
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distance: HashMap<String, usize> = HashMap::new();

    // start by declaring first node as visited
    visited.insert(start_node.to_string());
    // push it into the queue
    queue.push_back(start_node.to_string());
    // set its distance to 0 (itself)
    distance.insert(start_node.to_string(), 0);

    // while wueue is not empty, iterate through each in queue
        // return if node is found, else, go to next neighbors
    while let Some(node) = queue.pop_front() {
        if node == target_node {
            return Ok(distance[&node]);
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                    distance.insert(neighbor.clone(), distance[&node] + 1);
                }
            }
        }
    }
    Err(BFSError::NodeNotFound)
}

// takes in the file name (String slice) and the number of nodes
    // returns a result of either the graph or an error message
pub fn read_graph_from_file(filename: &str, nodes: u32) -> Result<Graph, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph: Graph = Graph::new(nodes);

    for line in reader.lines() {
        let line = line?; // unwrap if not empty or error
        let nodes: Vec<&str> = line.split_whitespace().collect();

        if nodes.len() != 2 {
            println!("Invalid line in the input file: {line}");
            continue; // skip
        }

        let from_node = nodes[0].to_string();
        let to_node = nodes[1].to_string();
        graph.add_edge(from_node, to_node);
    }
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new(5);
        assert_eq!(graph.0.len(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(9);
        // Replicates the graph in "datasets/directed_connected.txt"
        graph.add_edge("0".to_string(), "4".to_string());
        graph.add_edge("1".to_string(), "2".to_string());
        graph.add_edge("1".to_string(), "8".to_string());
        graph.add_edge("2".to_string(), "3".to_string());
        graph.add_edge("3".to_string(), "1".to_string());
        graph.add_edge("3".to_string(), "8".to_string());
        graph.add_edge("4".to_string(), "2".to_string());
        graph.add_edge("5".to_string(), "4".to_string());
        graph.add_edge("6".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "6".to_string());
        graph.add_edge("8".to_string(), "7".to_string());

        assert_eq!(graph.0.len(), 9);
        assert_eq!(graph.0.get("0"), Some(&vec!["4".to_string()]));
        assert_eq!(graph.0.get("7"), Some(&vec!["5".to_string(), "6".to_string()]));
        assert_eq!(graph.0.get("2"), Some(&vec!["3".to_string()]));
    }

    #[test]
    fn test_bfs_shortest_path() {
        let mut graph = Graph::new(5);
        // Draw out this graph again for report
        graph.add_edge("0".to_string(), "4".to_string());
        graph.add_edge("1".to_string(), "2".to_string());
        graph.add_edge("1".to_string(), "8".to_string());
        graph.add_edge("2".to_string(), "3".to_string());
        graph.add_edge("3".to_string(), "1".to_string());
        graph.add_edge("3".to_string(), "8".to_string());
        graph.add_edge("4".to_string(), "2".to_string());
        graph.add_edge("5".to_string(), "4".to_string());
        graph.add_edge("6".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "5".to_string());
        graph.add_edge("7".to_string(), "6".to_string());
        graph.add_edge("8".to_string(), "7".to_string());

        assert_eq!(bfs_shortest(&graph.0, "2", "6"), Ok(4));
        assert_eq!(bfs_shortest(&graph.0, "2", "2"), Ok(0));
        assert_eq!(bfs_shortest(&graph.0, "0", "2"), Ok(2));
        assert_eq!(bfs_shortest(&graph.0, "1", "4"), Ok(4));
    }

    #[test]
    fn test_read_graph_from_file() {
        let result = read_graph_from_file("datasets/directed_connected.txt", 5);
        assert!(result.is_ok());

        let graph = result.unwrap();
        assert_eq!(bfs_shortest(&graph.0, "2", "6"), Ok(4));
        assert_eq!(bfs_shortest(&graph.0, "2", "2"), Ok(0));
        assert_eq!(bfs_shortest(&graph.0, "0", "2"), Ok(2));
        assert_eq!(bfs_shortest(&graph.0, "1", "4"), Ok(4));
    }
}
