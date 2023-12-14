use crate::utils;
use std::path::Path;
use regex::Regex;

pub fn hot_springs_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let re = Regex::new(r"#+").unwrap();
    let mut sum: i32 = 0;
    for record in lines {
        let row = record.split(" ").collect::<Vec<&str>>();
        let permutations = record_permutations_recursive(&row[0]);
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
        let permutations = record_permutations_recursive(&new_row);
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

fn record_permutations_recursive(row: &str) -> Vec<String> {
    if row.is_empty() {
        return vec![String::new()];
    }

    let mut permutations = Vec::new();
    let first_char = row.chars().next().unwrap();
    let rest_of_row = &row[1..];

    let sub_permutations = record_permutations_recursive(rest_of_row);

    if first_char == '?' {
        for permutation in sub_permutations {
            permutations.push(format!("#{}", permutation));
            permutations.push(format!(".{}", permutation));
        }
    } else {
        for permutation in sub_permutations {
            permutations.push(format!("{}{}", first_char, permutation));
        }
    }

    permutations
}