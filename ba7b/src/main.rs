use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn parse_matrix(lines: Vec<String>) -> Vec<Vec<i32>>{
    let mut matrix = vec![];
    for line in lines{
        let row: Vec<i32> = line.split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        matrix.push(row);
    }
    matrix
}

fn calculate_score(matrix: &Vec<Vec<i32>>, i: usize, j: usize, k: usize) -> i32{
    let score =  (matrix[i][j] + matrix[j][k] - matrix[i][k]) / 2;
    score
}


fn main() -> io::Result<()>{
    let file = File::open("../../../data/rosalind_ba7b.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n = lines.next().unwrap()?.parse::<usize>().unwrap();
    let j = lines.next().unwrap()?.parse::<usize>().unwrap();
    let matrix_lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
    let matrix = parse_matrix(matrix_lines);
    let mut scores_arr = vec![];
    for i in 0..n{
        for k in 0..n{
            if i != j && j != k && i != k{
                scores_arr.push(calculate_score(&matrix, i, j, k));
            }
        }
    }
    // find minimal score
    let min_score = scores_arr.iter().min().unwrap();
    println!("{}", min_score);
    
    Ok(())
}
