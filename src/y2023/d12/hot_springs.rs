use crate::utils;
use std::path::Path;
use regex::Regex;

pub fn hot_springs_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let re = Regex::new(r"#+").unwrap();
    let mut sum: i32 = 0;
    let mut count = 0;
    for record in lines {
        dbg!(&count);
        let row = record.split(" ").collect::<Vec<&str>>();
        let permutations = record_permutations(&row[0]);
        let groupings = row[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        for permutation in permutations {
            let matches = re.find_iter(&permutation).collect::<Vec<_>>();
            if groupings.len() == matches.len() {
                let mut valid = true;
                for (i, mat) in matches.iter().enumerate() {
                    if groupings[i] != mat.len() {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    sum += 1;
                }
            }
        }
        count += 1;
    }
    
    println!("Hot springs part one: {}", sum);
}

pub fn hot_springs_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let re = Regex::new(r"#+").unwrap();
    let mut sum: i32 = 0;
    let mut count = 0;
    for record in lines {
        dbg!(&count);
        let row = record.split(" ").collect::<Vec<&str>>();
        let springs = String::from(row[0]);
        let mut new_row = String::from(row[0]);
        for _ in 0..4 {
            new_row.push('?');
            new_row.push_str(&springs);
        }
        let permutations = record_permutations(&new_row);
        let mut groupings = row[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let original = groupings.clone();
        for _ in 0..4 {
            groupings.extend(&original);
        }
        for permutation in permutations {
            let matches = re.find_iter(&permutation).collect::<Vec<_>>();
            if groupings.len() == matches.len() {
                let mut valid = true;
                for (i, mat) in matches.iter().enumerate() {
                    if groupings[i] != mat.len() {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    sum += 1;
                }
            }
        }
        count += 1;
    }
    
    println!("Hot springs part two: {}", sum);
}

fn record_permutations(row: &str) -> Vec<String> {
    let mut permutations = Vec::new();
    if row.starts_with("?") {
        permutations.push("#".to_string());
        permutations.push(".".to_string());
    } else {
        permutations.push(row.chars().take(1).collect());
    }
    for character in row.chars().skip(1) {
        let mut new_permutations = Vec::new();
        for permutation in permutations.iter_mut() {
            if character == '?' {
                let mut first_permutation = permutation.clone();
                let mut second_permutation = permutation.clone();
                first_permutation.push('#');
                second_permutation.push('.');
                new_permutations.push(first_permutation);
                new_permutations.push(second_permutation);
            } else {
                let mut new_permutation = permutation.clone();
                new_permutation.push(character);
                new_permutations.push(new_permutation);
            }
        }
        permutations = new_permutations;
    }
    permutations
}