use std::fs::File;
use std::io::{self, BufReader, BufRead};
use ba7d::*;

fn main() -> io::Result<()>{
    let file = File::open("../../../data/rosalind_ba7d.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    // Parse n
    let n: usize = lines.next()
    .expect("Missing first line")?
    .trim()
    .parse()
    .expect("First line should be a number");
    
    // Parse matrix
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        let line = lines.next().expect("Missing matrix row")?;
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        matrix[i] = numbers;
    }
    let tree = upgma(&mut matrix, n);
    print_tree(&tree);
    Ok(())
}