use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Node {
    node_id: u32,
    timestamp: u64,

    // These 2 vectors represent the edges of the node in both direction
    // Storing the edge of the graph in the node was easier than having 2 Vec in Graph
    parents: Vec<u32>,
    children: Vec<u32>,
}

impl Node {
    pub fn new(id: u32, time: u64) -> Self {
        Node {
            node_id: id,
            timestamp: time,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_parent(&mut self, parent_id: u32) {
        if !self.parents.contains(&parent_id) {
            self.parents.push(parent_id);
        }
    }

    pub fn add_child(&mut self, child_id: u32) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }
}

struct Edge {
    p1: u32,
    p2: u32,
}

#[derive(Clone)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, id: u32, timestamp: u64) {
        self.nodes.push(Node::new(id, timestamp));
    }

    pub fn parse(&mut self, filename: String) {
        let file = File::open(filename);
        // let mut reader = BufReader::new(file);
        let mut reader = BufReader::new(file.unwrap());

        let mut first_line = String::new();
        // Read first line
        // This gives the number of noce which is not necessary in Rust to parse a file
        let _res = reader.read_line(&mut first_line);
        let _res = match _res {
            Ok(usize) => usize,
            Err(error) => panic!("Error reading first line: {:?}", error),
        };
        // Remove trailing new line before parsing
        let first_line = first_line.trim_end();
        let num_nodes: usize = first_line.parse().unwrap();

        // First node is a special case
        self.add_node(0, 0);

        // Store the vertices set in a temporary Vec
        let mut tmp_vec: Vec<Edge> = Vec::new();
        let mut index = 1;
        for l in reader.lines() {
            let line: String = l.unwrap();
            let vec = line.split(" ").collect::<Vec<&str>>();
            self.add_node(index, vec[2].parse().unwrap());

            // Apply a -1 offset so node_id and index position are the same
            let mut tmp_node_id_p1: u32 = vec[0].parse().unwrap();
            let mut tmp_node_id_p2: u32 = vec[1].parse().unwrap();
            tmp_node_id_p1 = tmp_node_id_p1 - 1;
            tmp_node_id_p2 = tmp_node_id_p2 - 1;

            tmp_vec.push(Edge {
                p1: tmp_node_id_p1,
                p2: tmp_node_id_p2,
            });

            index = index + 1;
        }

        // Construct the graph from the vertices set
        for (pos, p) in tmp_vec.iter().enumerate() {
            // -1 offset because nodes_id starts at 1 and vector index starts at 0
            self.nodes[(pos + 1) as usize].add_parent(p.p1);
            self.nodes[(pos + 1) as usize].add_parent(p.p2);
            self.nodes[(p.p1) as usize].add_child((pos as u32) + 1);
            self.nodes[(p.p2) as usize].add_child((pos as u32) + 1);
        }

        // Do not forget node 0
        // Sanity check
        assert_eq!(num_nodes + 1, self.nodes.len());
    }

    // Get number of leaf in the graph
    pub fn get_num_of_leaf(&self) -> u32 {
        let mut num_leaf = 0;
        for n in 1..self.nodes.len() {
            if self.nodes[n].children.len() == 0 {
                num_leaf = num_leaf + 1;
            }
        }
        num_leaf
    }

    // Unless we don't identical references on a given nodes there are always 2 references per node
    // This formulas only depends on the number of node
    pub fn get_avg_reference_per_node(&self) -> f32 {
        let num_ref: f32 = 2.0 * (self.nodes.len() as f32 - 1.0) / (self.nodes.len() as f32);
        num_ref
    }

    pub fn get_avg_depth(&mut self) -> f32 {
        let mut sum_depth: f32 = 0.0;
        // Use index as node_id to avoid borrowing self
        for n in 0..self.nodes.len() {
            sum_depth = sum_depth + (Graph::get_depth(&mut self.nodes, n as u32)) as f32;
        }
        sum_depth = sum_depth / (self.nodes.len() as f32);
        sum_depth
    }

    pub fn get_avg_tx_per_depth(&mut self) -> f32 {
        let mut avg_tx: Vec<f32> = Vec::new();
        let mut curr_depth = 0;

        for n in 1..self.nodes.len() {
            let d = Graph::get_depth(&mut self.nodes, n as u32) as usize;
            let n_tx = self.nodes[n].children.len() as f32;
            if d != curr_depth {
                curr_depth = d;
                avg_tx.push(n_tx);
            } else {
                avg_tx[d - 1] = avg_tx[d - 1] + n_tx;
            }
        }
        let mut sum: f32 = avg_tx.iter().sum();
        sum = sum / (avg_tx.len() as f32);
        sum
    }

    pub fn show(&self) {
        println!("parents ->nodeid ->children");
        for node in &self.nodes {
            let nspace = min(8 - (node.parents.len()) * 3, 6);
            let depth = Graph::get_depth(&mut self.nodes.clone(), node.node_id);
            println!(
                "{parents:?}{a:>w$}->{nodeid}({depth}){b:filler$}->{children:?}",
                parents = node.parents,
                a = "",
                w = nspace,
                nodeid = node.node_id,
                depth = depth,
                b = "",
                filler = 6,
                children = node.children
            );
        }
    }

    // This function is not efficient as it has to check all the way to the root node before being able to determine its depth
    fn get_depth(nodes: &mut [Node], node_id: u32) -> u32 {
        let parents = &mut nodes[node_id as usize].parents;
        let mut d_vec: Vec<u32> = Vec::new();
        if parents.len() != 0 {
            let par = parents.clone();
            for p in par {
                let d = Graph::get_depth(nodes, p) + 1;
                d_vec.push(d);
            }
        }

        // Keep min value
        let mut depth = 0;
        for d_val in d_vec {
            if depth == 0 {
                depth = d_val;
            } else if d_val < depth {
                depth = d_val;
            }
        }
        // return depth
        depth
    }
}
