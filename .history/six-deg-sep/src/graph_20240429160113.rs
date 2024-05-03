use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum BFSError {
    NodeNotFound,
}

pub struct Graph(HashMap<String, Vec<String>>, u32);

impl Graph {
    pub fn new(nodes: u32) -> Self {
        Graph(HashMap::new(), nodes)
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
    let mut distance = HashMap::new();

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

// takes in the file name (String slice) and the 
pub fn read_graph_from_file(filename: &str, nodes: u32) -> Result<Graph, std::io::Error> {
    
}