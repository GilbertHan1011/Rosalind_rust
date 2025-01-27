use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Cluster {
    id: usize,
    age: f64,
    nodes: Vec<usize>,
}

impl Cluster {
    fn new(id: usize, age: f64, nodes: Vec<usize>) -> Self {
        Cluster { id, age, nodes }
    }

    fn compute_distance_with_cluster(&self, other: &Cluster, matrix: &Matrix) -> f64 {
        let mut distance_sum = 0.0;
        for &i in &self.nodes {
            for &j in &other.nodes {
                distance_sum += matrix[i][j] as f64;
            }
        }
        distance_sum / (self.nodes.len() * other.nodes.len()) as f64
    }
}

pub type Tree = HashMap<usize, Vec<(usize, f64)>>;
pub type Matrix = Vec<Vec<i32>>;

fn find_closest_clusters(cluster_list: &[Cluster], clusters: &HashSet<usize>, matrix: &Matrix) -> (usize, usize) {
    let mut min_distance = f64::MAX;
    let mut closest_pair = (0, 0);

    for &c1_id in clusters {
        for &c2_id in clusters {
            if c1_id != c2_id {
                let c1 = &cluster_list[c1_id];
                let c2 = &cluster_list[c2_id];
                let distance = c1.compute_distance_with_cluster(c2, matrix);
                if distance < min_distance {
                    min_distance = distance;
                    closest_pair = (c1_id, c2_id);
                }
            }
        }
    }
    closest_pair
}

fn connect_nodes(graph: &mut Tree, parent: &Cluster, child: &Cluster) {
    let distance = parent.age - child.age;
    graph.entry(parent.id)
        .or_insert_with(Vec::new)
        .push((child.id, distance));
    graph.entry(child.id)
        .or_insert_with(Vec::new)
        .push((parent.id, distance));
}

pub fn upgma(matrix: &Matrix, n: usize) -> Tree {
    let mut cluster_list: Vec<Cluster> = (0..n)
        .map(|id| Cluster::new(id, 0.0, vec![id]))
        .collect();
    
    let mut clusters: HashSet<usize> = (0..n).collect();
    let mut graph: Tree = HashMap::new();
    let mut current_id = n;

    while clusters.len() > 1 {
        // Find closest clusters
        let (c1_idx, c2_idx) = find_closest_clusters(&cluster_list, &clusters, matrix);
        let c1 = cluster_list[c1_idx].clone();
        let c2 = cluster_list[c2_idx].clone();

        // Calculate age
        let age = c1.compute_distance_with_cluster(&c2, matrix) / 2.0;

        // Create new cluster
        let mut new_nodes = c1.nodes.clone();
        new_nodes.extend(c2.nodes.iter());
        let new_cluster = Cluster::new(current_id, age, new_nodes);

        // Connect nodes
        connect_nodes(&mut graph, &new_cluster, &c1);
        connect_nodes(&mut graph, &new_cluster, &c2);

        // Update clusters
        clusters.remove(&c1.id);
        clusters.remove(&c2.id);
        clusters.insert(new_cluster.id);
        cluster_list.push(new_cluster);

        current_id += 1;
    }

    graph
}

pub fn print_tree(tree: &Tree) {
    let mut edges = Vec::new();
    for (&from, adj_list) in tree {
        for &(to, weight) in adj_list {
            edges.push((from, to, weight));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    
    for (from, to, weight) in edges {
        println!("{}->{}:{:.3}", from, to, weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgma() {
        let matrix = vec![
            vec![0, 20, 17, 11],
            vec![20, 0, 20, 13],
            vec![17, 20, 0, 10],
            vec![11, 13, 10, 0],
        ];
        let tree = upgma(&matrix, 4);
        
        // Verify some properties of the tree
        assert!(tree.len() > 4); // Should have internal nodes
        for i in 0..4 {
            assert!(tree.contains_key(&i)); // All leaves should be present
        }
    }
}