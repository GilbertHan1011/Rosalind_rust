use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Tree {
    edges: HashMap<i32, Vec<i32>>,
    colors: HashMap<i32, String>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            edges: HashMap::new(),
            colors: HashMap::new(),
        }
    }
}
fn parse_tree(lines: &[String]) -> Tree {
    let mut tree = Tree::new();
    let mut reading_edges = true;

    // First pass - parse edges and set all nodes to gray
    for line in lines {
        if line.trim() == "-" {
            reading_edges = false;
            continue;
        }

        if reading_edges {
            // Parse edges: "2 -> 0,1" format or "0 -> {}" format
            if let Some((node, connections)) = line.split_once("->") {
                let node = node.trim().parse::<i32>().unwrap();
                let connections_str = connections.trim();
                
                let connections: Vec<i32> = if connections_str == "{}" {
                    Vec::new()
                } else {
                    connections_str
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<i32>().unwrap())
                        .collect()
                };
                
                tree.edges.insert(node, connections);
                tree.colors.insert(node, "gray".to_string());
            }
        } else {
            // Parse colors: "0: red" format
            if let Some((node, color)) = line.split_once(':') {
                let node = node.trim().parse::<i32>().unwrap();
                let color = color.trim().to_string();
                tree.colors.insert(node, color);
            }
        }
    }
    tree
}
fn tree_coloring(tree: &mut Tree) {
    // while colored tree has ripe nodes
    let mut ripe_nodes = tree.edges.iter()
        .filter(|(&node, connections)| {
            // Node must be gray
            matches!(tree.colors.get(&node), Some(color) if color == "gray") &&
            // All children must not be gray
            connections.iter().all(|child| {
                matches!(tree.colors.get(child), Some(color) if color != "gray")
            })
        })
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();

    while !ripe_nodes.is_empty() {
        let node = ripe_nodes.pop().unwrap();
        let children_colors: Vec<&str> = tree.edges[&node]
            .iter()
            .map(|child| tree.colors[child].as_str())
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // if there exist differently colored children, color purple
        if children_colors.len() > 1 {
            tree.colors.insert(node, "purple".to_string());
        } else if !children_colors.is_empty() {
            let color = children_colors[0];
            tree.colors.insert(node, color.to_string());
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("../../../data/rosalind_ba9p.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    let mut tree = parse_tree(&lines);
    
    while tree.colors.values().any(|color| color == "gray") {
        tree_coloring(&mut tree);
    }

    // Sort nodes by number and print each on a new line
    let mut nodes: Vec<_> = tree.colors.iter().collect();
    nodes.sort_by_key(|&(k, _)| k);
    
    for (node, color) in nodes {
        println!("{}: {}", node, color);
    }

    Ok(())
}
