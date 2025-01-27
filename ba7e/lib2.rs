use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Tree {
    pub vertices: Vec<usize>,
    pub edges: HashMap<usize, Vec<(usize, f64)>>,
}

impl Tree {
    pub fn new(n: usize) -> Self {
        Tree {
            vertices: (0..n).collect(),
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize, length: f64) {
        if !self.vertices.contains(&node1) {
            self.vertices.push(node1);
        }
        if !self.vertices.contains(&node2) {
            self.vertices.push(node2);
        }
        
        self.edges.entry(node1)
            .or_insert_with(Vec::new)
            .push((node2, length));
        self.edges.entry(node2)
            .or_insert_with(Vec::new)
            .push((node1, length));
    }
}

pub type Matrix = Vec<Vec<f64>>;

fn compute_total_distance(matrix: &Matrix, i: usize) -> f64 {
    matrix[i].iter().sum()
}

fn find_closest_clusters(d_prime: &Matrix, n: usize) -> (usize, usize) {
    let mut min_distance = f64::MAX;
    let mut closest_pair = (0, 0);
    
    for i in 0..n {
        for j in (i+1)..n {
            if d_prime[i][j] < min_distance {
                min_distance = d_prime[i][j];
                closest_pair = (i, j);
            }
        }
    }
    closest_pair
}

pub fn print_tree(tree: &Tree) {
    let mut vertices = tree.vertices.clone();
    vertices.sort_unstable();
    
    for &node in &vertices {
        if let Some(edges) = tree.edges.get(&node) {
            let mut sorted_edges = edges.clone();
            sorted_edges.sort_by(|a, b| a.0.cmp(&b.0));
            
            for &(dest, weight) in &sorted_edges {
                println!("{}->{}:{:.3}", node, dest, weight);
            }
        }
    }
}

fn total_distance(d: &[Vec<f64>], i: usize) -> f64 {
    d[i].iter().sum()
}

fn nj_matrix(d: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    let len = d.len();
    let mut ndj = vec![vec![0.0; len]; len];
    
    for i in 0..len {
        for j in (i + 1)..len {
            ndj[i][j] = (n as f64 - 2.0) * d[i][j] - total_distance(d, i) - total_distance(d, j);
            ndj[j][i] = ndj[i][j];
        }
    }
    ndj
}

fn find_min_ndj(ndj: &[Vec<f64>]) -> (usize, usize) {
    let mut min_val = f64::MAX;
    let mut min_i = 0;
    let mut min_j = 0;
    
    for i in 0..ndj.len() {
        for j in (i + 1)..ndj.len() {
            if ndj[i][j] < min_val {
                min_val = ndj[i][j];
                min_i = i;
                min_j = j;
            }
        }
    }
    (min_i, min_j)
}

fn add_new_row(d: &mut Vec<Vec<f64>>, i: usize, j: usize) {
    let len = d.len();
    let mut new_row = Vec::with_capacity(len + 1);
    
    for k in 0..len {
        new_row.push(0.5 * (d[k][i] + d[k][j] - d[i][j]));
    }
    new_row.push(0.0);
    
    for x in 0..len {
        d[x].push(new_row[x]);
    }
    d.push(new_row);
}

fn remove_row(d: &[Vec<f64>], x: usize) -> Vec<Vec<f64>> {
    let mut new_matrix = Vec::new();
    for i in 0..d.len() {
        if i == x {
            continue;
        }
        let mut row = Vec::new();
        for j in 0..d.len() {
            if j == x {
                continue;
            }
            row.push(d[i][j]);
        }
        new_matrix.push(row);
    }
    new_matrix
}

pub fn neighbor_joining(mut d: Vec<Vec<f64>>, n: usize, mut nodes: Vec<usize>) -> Tree {
    if n == 2 {
        let mut tree = Tree::new(2);
        tree.add_edge(nodes[0], nodes[1], d[0][1]);
        return tree;
    }

    let ndj_matrix = nj_matrix(&d, n);
    let (i, j) = find_min_ndj(&ndj_matrix);
    
    let delta = (total_distance(&d, i) - total_distance(&d, j)) / (n as f64 - 2.0);
    let limb_i = (d[i][j] + delta) / 2.0;
    let limb_j = (d[i][j] - delta) / 2.0;
    
    add_new_row(&mut d, i, j);
    let m = nodes.iter().max().unwrap() + 1;
    nodes.push(m);
    
    d = remove_row(&d, j.max(i));
    d = remove_row(&d, j.min(i));
    
    let node_i = nodes[i];
    let node_j = nodes[j];
    nodes.remove(i.max(j));
    nodes.remove(i.min(j));
    
    let mut tree = neighbor_joining(d, n - 1, nodes);
    tree.add_edge(node_i, m, limb_i);
    tree.add_edge(node_j, m, limb_j);
    
    tree
}





/*
NeighborJoining(D,n)
 if n = 2
  T ← tree consisting of a single edge of length D1,2
  return T
 D' ← neighbor-joining matrix constructed from the distance matrix D
 find elements i and j such that D'i,j is a minimum non-diagonal element of D'
 Δ ← (TotalDistanceD(i) - TotalDistanceD(j)) /(n - 2)
 limbLengthi ← (1/2)(Di,j + Δ)
 limbLengthj ← (1/2)(Di,j - Δ)
 add a new row/column m to D so that Dk,m = Dm,k = (1/2)(Dk,i + Dk,j - Di,j) for any k
 remove rows i and j from D
 remove columns i and j from D
 T ← NeighborJoining(D, n - 1)
 add two new limbs (connecting node m with leaves i and j) to the tree T
 assign length limbLengthi to Limb(i)
 assign length limbLengthj to Limb(j)
 return T
 */