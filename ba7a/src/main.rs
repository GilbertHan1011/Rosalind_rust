use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

struct Edge {
    to: usize,
    weight: i32
}

fn parse_edge(line: &str) -> Option<(usize, Edge)> {
    let parts: Vec<&str> = line.split("->").collect();
    if parts.len() != 2 {
        return None;
    }
    let from = parts[0].parse::<usize>().ok()?;
    let to_weight: Vec<&str> = parts[1].split(":").collect();
    if to_weight.len() != 2{
        return None;
    }
    let to = to_weight[0].parse::<usize>().ok()?;
    let weight = to_weight[1].parse::<i32>().ok()?;
    Some((from, Edge { to, weight }))
}


fn build_graph(edges: Vec<(usize, Edge)>) -> HashMap<usize, Vec<Edge>>{
    let mut graph = HashMap::new();
    for (from, edge) in edges{
        graph.entry(from).or_insert(vec![]).push(edge);
    }
    graph
}

fn find_distance(graph: &HashMap<usize, Vec<Edge>>, 
    start: usize,
    end: usize,
    visited: &mut Vec<bool>
) -> Option<i32>{
    if start == end{
        return Some(0);
    }
    if visited[start]{
        return None;
    }
    if !graph.contains_key(&start){
        return None;
    }
    visited[start] = true;

    if let Some(edges) = graph.get(&start) {
        for edge in edges {
            if !visited[edge.to]{
                if let Some(distance) = find_distance(graph, edge.to, end, visited){
                    return Some(edge.weight + distance);
                }
            }
        }
    }
    visited[start] = false;
    None
}

fn create_distance_matrix(n : usize) -> Vec<Vec<i32>>{
    vec![vec![0; n]; n]
}

fn print_distance_matrix(matrix: &Vec<Vec<i32>>){
    for row in matrix{
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                print!("   ");
            }
            print!("{}", val);
        }
        println!();
    }
}

fn fill_distance_matrix(length: usize, dist_matrix: &mut Vec<Vec<i32>>, graph: &HashMap<usize, Vec<Edge>>){
    let mut visited = vec![false; length];
    for i in 0..dist_matrix.len(){
        for j in 0..dist_matrix.len(){
            if i != j{
                visited.fill(false);
                dist_matrix[i][j] = find_distance(graph, i, j, &mut visited).unwrap_or(-1);
            }
        }
    }
}

fn main() -> io::Result<()>{
    let file = File::open("../../../data/rosalind_ba7a.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    // First line contains n
    let n = lines.next().unwrap()?.parse::<usize>().unwrap();
    
    let mut edges = vec![];
    let mut nodes = std::collections::HashSet::new();
    
    for line in lines {
        let line = line?;
        if let Some((from, edge)) = parse_edge(&line){
            nodes.insert(from);
            nodes.insert(edge.to);
            edges.push((from, edge));
        }
    }

    let total_nodes = nodes.len();
    let graph = build_graph(edges);

    let mut dist_matrix = create_distance_matrix(n);
    fill_distance_matrix(total_nodes, &mut dist_matrix, &graph);
    print_distance_matrix(&dist_matrix);

    Ok(())
}
