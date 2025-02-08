use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Trie {
    nodes: Vec<Node>,
    edges: HashMap<(usize, char), usize>,
}

struct Node {
    id: usize,
    label: String,
}

impl Trie {
    fn next_node(&self, current_node: usize, symbol: char) -> usize {
        *self.edges.get(&(current_node, symbol)).unwrap_or(&usize::MAX)
    }
    fn add_edge(&mut self, start: usize, end: usize, symbol: char) {
        self.edges.insert((start, symbol), end);
    }
    fn add_node(&mut self, label: String) {
        self.nodes.push(Node {
            id: self.nodes.len(),
            label,
        });
    }
}

fn trieconstruction(patterns: Vec<String>) -> Trie {
    let mut trie = Trie {
        nodes: Vec::with_capacity(1000),
        edges: HashMap::with_capacity(1000),
    };
    
    trie.add_node("ROOT".to_string());

    for pattern in patterns {
        let mut currentNode = 0;
        for currentSymbol in pattern.chars() {
            let nextNode = trie.next_node(currentNode, currentSymbol);
            if nextNode == usize::MAX {
                trie.add_node(currentSymbol.to_string());
                let newNode = trie.nodes.len() - 1;
                trie.add_edge(currentNode, newNode, currentSymbol);
                currentNode = newNode;
            } else {
                currentNode = nextNode;
            }
        }
    }
    trie
}

fn write_trie_to_file(trie: &Trie) -> Result<(), Box<dyn Error>> {
    let mut output = String::with_capacity(1000);
    for (&(start, symbol), &end) in &trie.edges {
        output.push_str(&format!("{}->{}:{}\n", start, end, symbol));
    }
    
    std::fs::write("trie_output.txt", output)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../../data/rosalind_ba9a.txt")?;
    let reader = BufReader::new(file);
    let mut patterns = Vec::with_capacity(100);
    
    for line in reader.lines() {
        patterns.push(line?);
    }
    
    let trie = trieconstruction(patterns);
    write_trie_to_file(&trie)?;
    Ok(())
}