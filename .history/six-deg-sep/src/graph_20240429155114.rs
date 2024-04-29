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

pub fn bfs_shortest(graph: &HashMap<String, Vec<String>>, start_node)