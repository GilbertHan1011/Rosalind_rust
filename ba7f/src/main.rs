use std::collections::{HashMap, BTreeMap};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

fn sum(a: &[i32], b: &mut [i32]) {
    for i in 0..a.len() {
        b[i] += a[i];
    }
}

fn find_min(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    a[0] + b[0]
}

fn traverse(
    alphabet: &[char],
    tree: &BTreeMap<i32, Vec<i32>>,
    sk: &mut Vec<Vec<Vec<i32>>>,
    root: i32,
) {
    if root < *tree.keys().next().unwrap() {
        return;
    }

    let daughter = tree[&root][0];
    let son = tree[&root][1];
    traverse(alphabet, tree, sk, daughter);
    traverse(alphabet, tree, sk, son);

    for i in 0..sk[0].len() {
        for (k_idx, &k) in alphabet.iter().enumerate() {
            let mut min = i32::MAX;
            let mut delta_son = vec![1; 4];
            delta_son[k_idx] = 0;
            let mut delta_daughter = vec![1; 4];
            delta_daughter[k_idx] = 0;

            let daughter_idx = daughter as usize;
            let son_idx = son as usize;
            let root_idx = root as usize;

            let mut sum_daughter = delta_daughter.clone();
            sum(&sk[daughter_idx][i], &mut sum_daughter);
            let mut sum_son = delta_son.clone();
            sum(&sk[son_idx][i], &mut sum_son);

            let check = find_min(sum_daughter, sum_son);
            if check < min {
                min = check;
            }
            sk[root_idx][i][k_idx] = min;
        }
    }
}

fn backtrack(
    alphabet: &[char],
    tree: &BTreeMap<i32, Vec<i32>>,
    sk: &mut Vec<Vec<Vec<i32>>>,
    root: i32,
    parents: &HashMap<i32, i32>,
    chars: &mut Vec<Vec<char>>,
) {
    if root < *tree.keys().next().unwrap() {
        return;
    }

    let root_idx = root as usize;
    for i in 0..sk[root_idx].len() {
        let c = &sk[root_idx][i];
        
        if root == *tree.keys().last().unwrap() {
            let (min_idx, _) = c.iter()
                .enumerate()
                .min_by_key(|(_, &val)| val)
                .unwrap();
            chars[root_idx][i] = alphabet[min_idx];
        } else {
            let parent_char = chars[parents[&root] as usize][i];
            let place = alphabet.iter().position(|&x| x == parent_char).unwrap();
            let mut delta = vec![1; 4];
            delta[place] = 0;
            
            let mut summed = delta.clone();
            sum(c, &mut summed);
            
            let (min_idx, _) = summed.iter()
                .enumerate()
                .min_by_key(|(_, &val)| val)
                .unwrap();
            chars[root_idx][i] = alphabet[min_idx];
        }
    }

    backtrack(alphabet, tree, sk, tree[&root][0], parents, chars);
    backtrack(alphabet, tree, sk, tree[&root][1], parents, chars);
}

fn small_parsimony(
    labels: &[String],
    tree: &BTreeMap<i32, Vec<i32>>,
    parents: &HashMap<i32, i32>,
) -> Vec<String> {
    let alphabet = vec!['A', 'C', 'G', 'T'];
    let mut sk = Vec::new();

    // Initialize sk for labels
    for label in labels {
        let mut sk_labels = vec![vec![48; 4]; label.len()];
        for (i, c) in label.chars().enumerate() {
            let k = alphabet.iter().position(|&x| x == c).unwrap();
            sk_labels[i][k] = 0;
        }
        sk.push(sk_labels);
    }

    // Initialize sk for internal nodes
    for _ in labels.len()..(2 * (labels.len() - 1) + 1) {
        sk.push(vec![vec![48; 4]; labels[0].len()]);
    }

    let root = *tree.keys().last().unwrap();
    traverse(&alphabet, tree, &mut sk, root);

    let mut chars = vec![vec![' '; labels[0].len()]; sk.len()];
    for (i, label) in labels.iter().enumerate() {
        chars[i] = label.chars().collect();
    }

    backtrack(&alphabet, tree, &mut sk, root, parents, &mut chars);

    let mut output = Vec::new();

    // Calculate parsimony score
    let mut parsimony = 0;
    let root_idx = root as usize;
    for i in 0..sk[root_idx].len() {
        let min_score = sk[root_idx][i].iter().min().unwrap();
        parsimony += min_score;
    }
    output.push(parsimony.to_string());

    // Format edges
    let mut strings = Vec::new();
    for (&key, children) in tree {
        let parent: String = chars[key as usize].iter().collect();
        let left_child: String = chars[children[0] as usize].iter().collect();
        let right_child: String = chars[children[1] as usize].iter().collect();
        
        strings.push(format!("{}->{}:{}", parent, left_child, distance(&parent, &left_child)));
        strings.push(format!("{}->{}:{}", parent, right_child, distance(&parent, &right_child)));
    }

    for s in &strings {
        output.push(s.to_string());
        let parts: Vec<&str> = s.split("->").collect();
        let get_parent = parts[0];
        let child_parts: Vec<&str> = parts[1].split(":").collect();
        let child = child_parts[0];
        let distance = child_parts[1];
        output.push(format!("{}->{}{}{}", child, get_parent, ":", distance));
    }

    output
}

fn main() -> io::Result<()> {
    let file = File::open("../../../data/rosalind_ba7f.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let n: i32 = lines.next().unwrap()?.parse().unwrap();
    let mut tree = BTreeMap::new();
    let mut parents = HashMap::new();
    let mut labels = Vec::new();
    let mut count = -1;

    while let Some(line) = lines.next() {
        let line = line?;
        if line == "exit" {
            break;
        }

        let parts: Vec<&str> = line.split("->").collect();
        let first: i32 = parts[0].parse().unwrap();

        if is_integer(parts[1]) {
            let second: i32 = parts[1].parse().unwrap();
            tree.entry(first)
                .or_insert_with(Vec::new)
                .push(second);
            parents.insert(second, first);
        } else {
            count += 1;
            labels.push(parts[1].to_string());
            tree.entry(first)
                .or_insert_with(Vec::new)
                .push(count);
            parents.insert(count, first);
        }
    }

    let output = small_parsimony(&labels, &tree, &parents);
    let mut output_file = File::create("../../../data/output_ba7f.txt")?;
    for line in output {
        writeln!(output_file, "{}", line)?;
    }
    Ok(())
}