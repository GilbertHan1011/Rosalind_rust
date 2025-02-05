use std::io::{self, BufRead, BufReader};
use std::fs::File;


fn FarthestFirstTraversal(points: &Vec<Vec<f64>>, k: usize) -> Vec<Vec<f64>> {
    let mut centers: Vec<Vec<f64>> = Vec::new();
    let mut current_center = points[0].clone();
    centers.push(current_center);
    for _ in 1..k {
        let mut max_distance = 0.0;
        let mut max_distance_index = 0;
        for (i, point) in points.iter().enumerate() {
            let mut min_distance = f64::MAX;
            for center in &centers {
                let distance = distance(point, center);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
            if min_distance > max_distance {
                max_distance = min_distance;
                max_distance_index = i;
            }
        }
        current_center = points[max_distance_index].clone();
        centers.push(current_center);
    }
    centers
}

fn distance(point1: &Vec<f64>, point2: &Vec<f64>) -> f64 {
    point1.iter().zip(point2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
}
fn format_points(points: &Vec<Vec<f64>>) -> String {
    points.iter().map(|p| p.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n")
}

fn main() -> io::Result<()> {
    let file = File::open("../../../data/rosalind_ba7g.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let mut parts: Vec<usize> = lines.next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let k = parts[0];
    let m = parts[1];
    //println!("{} {}", k, m);

    let mut points: Vec<Vec<f64>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        let parts: Vec<f64> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        points.push(parts);
    }
    //println!("{:?}", points);

    let centers = FarthestFirstTraversal(&points, k);
    //println!("{}", format_points(&centers));
    let output = format_points(&centers);
    println!("{}", output);

    Ok(())

}
