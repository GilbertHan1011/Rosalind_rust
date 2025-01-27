use std::collections::{HashMap, VecDeque};

pub struct Edge {
    pub to: usize,
    pub weight: i32,
}

pub type Tree = Vec<HashMap<usize, i32>>;  // Changed from previous HashMap structure
pub type Matrix = Vec<Vec<i32>>;

pub fn calculate_limb_length(matrix: &Matrix, n: usize, j: usize) -> (i32, usize, usize) {
    let mut limb_length = i32::MAX;
    let mut curr_i;
    let mut curr_k = 0;
    
    // Use j-1 or j+1 as initial i value
    curr_i = if j > 0 { j - 1 } else { j + 1 };
    
    for k in 0..n {
        if curr_i != k && k != j {
            let curr_length = (matrix[curr_i][j] + matrix[j][k] - matrix[curr_i][k]) / 2;
            if curr_length < limb_length {
                limb_length = curr_length;
                curr_k = k;
            }
        }
    }
    (limb_length, curr_i, curr_k)
}

fn add_node(adj: &mut Tree, j: usize, limb_length: i32, i: usize, k: usize, x: i32) {
    let l = adj.len();
    let mut dist = vec![i32::MAX; l];
    let mut parent = vec![usize::MAX; l];
    let mut q = VecDeque::new();
    
    dist[i] = 0;
    q.push_back(i);
    
    while let Some(curr_node) = q.pop_front() {
        for (&node, &weight) in &adj[curr_node] {
            if dist[node] == i32::MAX {
                dist[node] = dist[curr_node] + weight;
                parent[node] = curr_node;
                q.push_back(node);
                
                if node == k {
                    let mut prev_node = node;
                    let mut curr_node = node;
                    
                    while dist[prev_node] > x {
                        curr_node = prev_node;
                        prev_node = parent[curr_node];
                    }
                    
                    if dist[prev_node] == x {
                        // Add leaf directly to existing node
                        adj[prev_node].insert(j, limb_length);
                        adj[j].insert(prev_node, limb_length);
                    } else {
                        // Create new internal node
                        adj.push(HashMap::new());
                        let new_node = adj.len() - 1;
                        
                        // Add new edges
                        adj[j].insert(new_node, limb_length);
                        adj[new_node].insert(j, limb_length);
                        
                        // Remove old edge
                        adj[prev_node].remove(&curr_node);
                        adj[curr_node].remove(&prev_node);
                        
                        // Add new edges
                        adj[prev_node].insert(new_node, x - dist[prev_node]);
                        adj[new_node].insert(prev_node, x - dist[prev_node]);
                        adj[curr_node].insert(new_node, dist[curr_node] - x);
                        adj[new_node].insert(curr_node, dist[curr_node] - x);
                    }
                    return;
                }
            }
        }
    }
}

pub fn additive_phylogeny(matrix: &Matrix, n: usize) -> Tree {
    let mut adj = vec![HashMap::new(); n];
    
    // Base case with 2 nodes
    adj[0].insert(1, matrix[0][1]);
    adj[1].insert(0, matrix[1][0]);
    
    // Add remaining nodes
    for j in 2..n {
        let (limb_length, i, k) = calculate_limb_length(matrix, j + 1, j);
        let x = matrix[i][j] - limb_length;
        add_node(&mut adj, j, limb_length, i, k, x);
    }
    
    adj
}

pub fn print_tree(adj: &Tree) {
    for (i, edges) in adj.iter().enumerate() {
        for (&to, &weight) in edges {
            println!("{}->{}:{}", i, to, weight);
        }
    }
}