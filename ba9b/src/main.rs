use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

fn prefix_trie_matching(text: &str, trie: &Trie) -> Option<String> {
    let mut symbol_iter = text.chars();
    let mut symbol = match symbol_iter.next() {
        Some(s) => s,
        None => return None
    };
    let mut v = 0; // root node
    
    loop {
        // If we've reached a leaf node, return the pattern
        if trie.edges.iter().all(|(&(start, _), _)| start != v) {
            let mut pattern = String::new();
            let mut node = 0;
            let mut text_chars = text.chars();
            while node != v {
                let c = text_chars.next().unwrap();
                pattern.push(c);
                node = trie.next_node(node, c);
            }
            return Some(pattern);
        }
        // If there is an edge from v labeled with symbol
        else if trie.edges.contains_key(&(v, symbol)) {
            v = trie.next_node(v, symbol);
            symbol = match symbol_iter.next() {
                Some(s) => s,
                None => continue // Continue to check if we're at a leaf
            };
        }
        else {
            return None;
        }
    }
}

fn trie_matching(text: &str, trie: &Trie) -> Vec<usize> {
    let mut positions = Vec::new();
    for i in 0..text.len() {
        if let Some(_) = prefix_trie_matching(&text[i..], trie) {
            positions.push(i);
        }
    }
    positions
}

fn main() -> Result<(), io::Error> {
    let file = File::open("../../../data/rosalind_ba9b.txt")?;
    let mut reader = BufReader::new(file);
    let mut text = String::new();
    reader.read_line(&mut text)?;
    text = text.trim().to_string();
    let mut patterns = Vec::with_capacity(100);
    
    for line in reader.lines() {
        patterns.push(line?);
    }
    
    let trie = trieconstruction(patterns);
    let positions = trie_matching(&text, &trie);

    for pos in positions {
        print!("{} ", pos);
    }
    println!();
    Ok(())
}
