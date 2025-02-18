use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Node {
    children: HashMap<u8, usize>, // char -> edge index
}

#[derive(Debug)]
struct Edge {
    start: usize,
    end: usize,
    dest: usize,
}

struct SuffixTree {
    text: Vec<u8>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl SuffixTree {
    fn new(text: &str) -> Self {
        let text = text.as_bytes().to_vec();
        let root = Node { children: HashMap::new() };
        SuffixTree {
            text,
            nodes: vec![root],
            edges: Vec::new(),
        }
    }

    fn build(&mut self) {
        for i in 0..self.text.len() {
            self.add_suffix(i);
        }
    }

    fn add_suffix(&mut self, start: usize) {
        let mut curr_node = 0;
        let mut pos = start;

        while pos < self.text.len() {
            let curr_char = self.text[pos];
            
            if let Some(&edge_idx) = self.nodes[curr_node].children.get(&curr_char) {
                let edge = &self.edges[edge_idx];
                let mut matched = 0;
                let mut edge_pos = edge.start;

                while edge_pos < edge.end && pos < self.text.len() {
                    if self.text[edge_pos] != self.text[pos] {
                        // Split edge
                        self.split_edge(edge_idx, matched, pos);
                        break;
                    }
                    matched += 1;
                    edge_pos += 1;
                    pos += 1;
                }

                if edge_pos == edge.end {
                    curr_node = edge.dest;
                    continue;
                }
                break;
            } else {
                // Create new edge
                let new_node = self.nodes.len();
                self.nodes.push(Node { children: HashMap::new() });
                
                let edge = Edge {
                    start: pos,
                    end: self.text.len(),
                    dest: new_node,
                };
                
                let edge_idx = self.edges.len();
                self.edges.push(edge);
                self.nodes[curr_node].children.insert(curr_char, edge_idx);
                break;
            }
        }
    }

    fn split_edge(&mut self, edge_idx: usize, matched: usize, pos: usize) {
        let edge = self.edges[edge_idx];
        let split_node = self.nodes.len();
        self.nodes.push(Node { children: HashMap::new() });

        // Create new edges
        let new_edge = Edge {
            start: edge.start,
            end: edge.start + matched,
            dest: split_node,
        };

        let continuation = Edge {
            start: edge.start + matched,
            end: edge.end,
            dest: edge.dest,
        };

        let leaf = Edge {
            start: pos,
            end: self.text.len(),
            dest: self.nodes.len(),
        };

        // Update edges
        self.edges[edge_idx] = new_edge;
        let cont_idx = self.edges.len();
        self.edges.push(continuation);
        let leaf_idx = self.edges.len();
        self.edges.push(leaf);

        // Update nodes
        self.nodes.push(Node { children: HashMap::new() });
        self.nodes[split_node].children.insert(self.text[edge.start + matched], cont_idx);
        self.nodes[split_node].children.insert(self.text[pos], leaf_idx);
    }

    fn get_edge_labels(&self) -> Vec<String> {
        let mut labels = Vec::new();
        for edge in &self.edges {
            let label = String::from_utf8_lossy(&self.text[edge.start..edge.end]).into_owned();
            labels.push(label);
        }
        labels
    }
}

fn main() -> io::Result<()> {
    eprintln!("Starting program...");
    
    // Read input
    let input = std::fs::read_to_string("../../../data/rosalind_ba9c.txt")?;
    let text = input.trim().to_string() ;
    eprintln!("Input text: {}", text);

    // Build suffix tree
    let mut tree = SuffixTree::new(&text);
    tree.build();

    // Get edge labels
    let mut labels = tree.get_edge_labels();
    
    // Write output
    let mut output = File::create("rosalind_ba9c_output.txt")?;
    for label in labels {
        writeln!(output, "{}", label)?;
    }

    eprintln!("Program completed successfully");
    Ok(())
}
