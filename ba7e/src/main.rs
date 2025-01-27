use std::fs::File;
use std::io::{self, BufReader, BufRead};
use ba7e::*;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../../data/rosalind_ba7e.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let matrix: Matrix = lines.map(|line| line.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect()).collect();
    let labels: Vec<usize> = (0..n).collect();
    let tree = neighbor_joining(matrix, n, labels);
    print_tree(&tree);
    Ok(())
}