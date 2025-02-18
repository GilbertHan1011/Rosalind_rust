use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Node {
    children: HashMap<u8, usize>, // char -> edge index
}

#[derive(Debug, Clone)]
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

    fn is_leaf(&self, node_idx: usize) -> bool {
        self.nodes[node_idx].children.is_empty()
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
                let edge = self.edges[edge_idx].clone();
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
        let edge = self.edges[edge_idx].clone();
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

    fn get_path_label(&self, edge: &Edge) -> String {
        String::from_utf8_lossy(&self.text[edge.start..edge.end]).into_owned()
    }

    fn dfs(&self, node_idx: usize, current_string: String, longest_repeat: &mut String, max_length: &mut usize) {
        if self.is_leaf(node_idx) {
            return;
        }
        if self.nodes[node_idx].children.len() > 1 && current_string.len() > *max_length {
            *longest_repeat = current_string.clone();
            *max_length = current_string.len();
        }

        for (&_char, &edge_idx) in &self.nodes[node_idx].children {
            let edge = &self.edges[edge_idx];
            let edge_label = self.get_path_label(edge);
            let new_string = current_string.clone() + &edge_label;
            self.dfs(edge.dest, new_string, longest_repeat, max_length);
        }
    }

    fn find_longest_repeat(&self) -> String {
        let mut longest_repeat = String::new();
        let mut max_length = 0;
        self.dfs(0, String::new(), &mut longest_repeat, &mut max_length);
        longest_repeat
    }
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("../../../data/rosalind_ba9d.txt")?;
    let text = input.trim().to_string();
    let mut tree = SuffixTree::new(&(text + "$"));
    tree.build();
    let longest_repeat = tree.find_longest_repeat();
    println!("{}", longest_repeat);
    Ok(())
}
