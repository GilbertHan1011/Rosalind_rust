use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn distance(point1:&Vec<f64>, point2:&Vec<f64>) -> f64 {
    point1.iter().zip(point2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
}

fn distortion(points:&Vec<Vec<f64>>, centers:&Vec<Vec<f64>>) -> f64 {
    let mut total_squared_dist = 0.0;
    let n = points.len();
    for point in points {
        let mut min_dist = f64::MAX;
        for center in centers {
            let dist = distance(point, center);
            if dist < min_dist {
                min_dist = dist;
            }
        }
        total_squared_dist += min_dist.powi(2);
    }
    total_squared_dist / n as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../../data/rosalind_ba8b.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut parts: Vec<usize> = lines.next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let k = parts[0];
    let m = parts[1];

    let mut centers: Vec<Vec<f64>> = Vec::new();
    for _ in 0..k {
        let line = lines.next().unwrap()?;
        let parts: Vec<f64> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        centers.push(parts);
    }
    // skip the next line
    lines.next();

    let mut points: Vec<Vec<f64>> = Vec::new();
    for line in lines {
        let parts: Vec<f64> = line?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        points.push(parts);
    }

    let distortion_val = distortion(&points, &centers);
    println!("{:.3}", distortion_val);

    Ok(())
}
