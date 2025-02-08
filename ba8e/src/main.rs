use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap,HashSet};

#[derive(Clone, Debug)]
struct Cluster {
    id: usize,
    nodes: Vec<usize>,
}

impl Cluster {
    fn new(id: usize, nodes: Vec<usize>) -> Self {
        Cluster { id, nodes }
    }

    fn compute_distance_with_cluster(&self, other: &Cluster, matrix: &Matrix) -> f64 {
        let mut distance_sum = 0.0;
        for &i in &self.nodes {
            for &j in &other.nodes {
                distance_sum += matrix[i-1][j-1] as f64; // Fix index by subtracting 1
            }
        }
        distance_sum / (self.nodes.len() * other.nodes.len()) as f64
    }
}

type Tree = HashMap<usize, Vec<(usize, f64)>>;
type Matrix = Vec<Vec<f64>>;

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

fn hierarchical_clustering(matrix: &Matrix, n: usize) -> Vec<Vec<usize>> {
    let mut cluster_list: Vec<Cluster> = (0..n)
        .map(|id| Cluster::new(id, vec![id + 1]))
        .collect();
    let mut clusters: HashSet<usize> = (0..n).collect();
    let mut current_id = n;
    let mut result = Vec::new();

    while clusters.len() > 1 {
        let (c1_idx, c2_idx) = find_closest_clusters(&cluster_list, &clusters, matrix);
        let c1 = cluster_list[c1_idx].clone();
        let c2 = cluster_list[c2_idx].clone();

        let mut new_nodes = c1.nodes.clone();
        new_nodes.extend(c2.nodes.iter());
        new_nodes.sort();
        let new_cluster = Cluster::new(current_id, new_nodes.clone());

        clusters.remove(&c1_idx);
        clusters.remove(&c2_idx);
        clusters.insert(current_id);
        cluster_list.push(new_cluster);
        result.push(new_nodes);
        current_id += 1;
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../../data/rosalind_ba8e.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first_line = lines.next().ok_or("Missing first line")??;
    let n: usize = first_line.parse()?;
    
    let mut dist_mat: Matrix = Vec::new();
    for line in lines {
        let parts: Vec<f64> = line?
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<f64>, _>>()?;
        dist_mat.push(parts);
    }

    let clusters = hierarchical_clustering(&dist_mat, n);
    for cluster in clusters {
        println!("{}", cluster.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "));
    }

    Ok(())
}