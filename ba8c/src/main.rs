use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn distance(point1: &Vec<f64>, point2: &Vec<f64>) -> f64 {
    point1.iter().zip(point2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
}

fn nearest_center_index(point: &Vec<f64>, centers: &Vec<Vec<f64>>) -> usize {
    let mut min_dist = f64::MAX;
    let mut min_index = 0;
    for (i, center) in centers.iter().enumerate() {
        let dist = distance(point, center);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    min_index
}

fn compute_center(points: &Vec<Vec<f64>>, assignments: &Vec<usize>, center_index: usize, m: usize) -> Vec<f64> {
    let mut center = vec![0.0; m];
    let mut count = 0;
    
    for (point, &assign) in points.iter().zip(assignments.iter()) {
        if assign == center_index {
            for i in 0..m {
                center[i] += point[i];
            }
            count += 1;
        }
    }
    
    if count > 0 {
        for i in 0..m {
            center[i] /= count as f64;
        }
    }
    center
}

fn k_means(points: &Vec<Vec<f64>>, k: usize, m: usize) -> Vec<Vec<f64>> {
    // Initialize centers with first k points
    let mut centers: Vec<Vec<f64>> = points[0..k].to_vec();
    
    // Run for 20 iterations
    for _ in 0..20 {
        // Assign points to nearest centers
        let assignments: Vec<usize> = points.iter()
            .map(|point| nearest_center_index(point, &centers))
            .collect();
            
        // Update centers
        centers = (0..k)
            .map(|i| compute_center(points, &assignments, i, m))
            .collect();
    }
    
    centers
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../../../data/rosalind_ba8c.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    // Read k and m from first line
    let first_line = lines.next().ok_or("Missing first line")??;
    let parts: Vec<usize> = first_line.split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    let k = parts[0];
    let m = parts[1];
    
    // Read points
    let mut points: Vec<Vec<f64>> = Vec::new();
    for line in lines {
        let parts: Vec<f64> = line?
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<f64>, _>>()?;
        points.push(parts);
    }

    // Run k-means and print results
    let centers = k_means(&points, k, m);
    for center in centers {
        for (i, point) in center.iter().enumerate() {
            print!("{:.3}", point);
            if i < center.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }
    
    Ok(())
}
