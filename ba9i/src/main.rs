use std::error::Error;
type Matrix = Vec<Vec<char>>;

fn sort_matrix(matrix: Matrix) -> Matrix {
    let first_column = matrix.iter().map(|row| row[0]).collect::<Vec<char>>();
    let mut sorted_matrix = matrix.clone();
    sorted_matrix.sort_by_key(|row| row[0]);
    sorted_matrix
}

fn last_column(matrix: Matrix) -> Vec<char> {
    matrix.iter().map(|row| row[matrix.len() - 1]).collect()
}

fn burrows_wheeler_transform(text: &str) -> String {
    let n = text.len();
    let mut rotations: Vec<String> = Vec::with_capacity(n);
    
    // Generate all rotations
    for i in 0..n {
        let mut rotation = text[i..].to_string();
        rotation.push_str(&text[0..i]);
        rotations.push(rotation);
    }
    
    // Sort rotations
    rotations.sort();
    
    // Extract last character from each rotation
    rotations.iter()
        .map(|s| s.chars().last().unwrap())
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "ACGCTGGCCGCGGCAATGTAGGAATATCGTGCGCGGCAACCACGGCAAAGCGTCAATCACCACCTGATCGTCCCCCATAATTGAGAGGGTTGTTTTCTATTTTTCACAACGATACCGGTCTTTAGCATTTCGCTCGAGAGAAAAGTAGCAGGTTTTAACAGCGGGCGTTTGACGCGGCCGACTCGTCTCTGTACCGATGAGACCCACGGGGTAATCTCTCGACGCAGGGCCGAGAGAGCTCCTAGACGTCGTCGGTCACGGCGGTATCTTTATTGTCACACCTCTACAAGGTATACGCGTGCTATGATCGGTTCCTCGGCTAGCTTACCGGCCTAACTGCTCTGCGAGCTGCGCCAAGATAAATGCTAGTCTGATTCCACGAAATGGAGCGGTACGGTCTGGCAGGGGTATTCTTCCCAATTAGGGAGATTAGATAACGCCTATGGTCGAGTGGGCAGACTATAGGGGTTCTTAAAACATAACTACTATTAGGCCTCGCTTGCATAGCCGTGCAGTCTCGACTTCGAACGAGGACTATAATAGCGGAGTATGAGTTCCATCTTGCCTGGCATAGTCGCTACCCTGTCAATACTTGAAAAGACATGTCCACGGGCTAAAATCCCAACAGGACGATCATGTGCATCTTCCTCGCTAAAAAACGACAGAAGGGGGTGCGCAGATTCTTCCGTCCGATGATGTGATACGCCCCACTTCTTGTTCGACCGATCTCTGGTCCTTTTATAGCACCTCCGGAGGTATGGGGTAACCTCCGTTTGGTTTTCCAACTGCATTTTCGACGAACTGGTGGTAGTTCCTACCCTCCCCCCGAGGGTTGGCGTCCATTCACCAATCGATGCAGCGCTTAGTGTACAATACTAATACTTGGTCATATTT$";
    let bwt = burrows_wheeler_transform(input);
    println!("{}", bwt);
    Ok(())
}