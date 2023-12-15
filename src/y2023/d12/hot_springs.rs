use crate::utils;
use std::path::Path;
use std::collections::HashMap;

pub fn hot_springs_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for record in lines {
        let row = record.split(" ").collect::<Vec<&str>>();
        let groupings = row[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        sum += count_permutations(&row[0], &groupings, 0, 0, 0, &mut HashMap::new());
    }
    
    println!("Hot springs part one: {}", sum);
}

pub fn hot_springs_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for record in lines {
        let row = record.split(" ").collect::<Vec<&str>>();
        let mut expanded_row = String::from(row[0]);
        for _ in 0..4 {
            expanded_row.push_str("?");
            expanded_row.push_str(&row[0]);
        }
        let groupings = row[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let mut expanded_groupings = groupings.clone();
        for _ in 0..4 {
            expanded_groupings.append(&mut groupings.clone());
        }
        sum += count_permutations(&expanded_row, &expanded_groupings, 0, 0, 0, &mut HashMap::new());
    }
    
    println!("Hot springs part two: {}", sum);
}

fn count_permutations(s: &str, pattern: &Vec<usize>, idx: usize, pat_idx: usize, curr_len: usize, memo: &mut HashMap<(usize, usize, usize), usize>) -> usize {
    if memo.contains_key(&(idx, pat_idx, curr_len)) {
        return memo[&(idx, pat_idx, curr_len)];
    }
    if idx == s.len() {
        if pat_idx == pattern.len() && curr_len == 0 {
            return 1;
        } else if pat_idx == pattern.len()-1 && pattern[pat_idx] == curr_len {
            return 1;
        } else {
            return 0;
        }
    }
    let mut result = 0;
    for c in ['.', '#'] {
        if s[idx..].starts_with(c) || s[idx..].starts_with('?') {
            if c == '.' && curr_len == 0 {
                result += count_permutations(s, pattern, idx + 1, pat_idx, 0, memo);
            } else if c =='.' && curr_len > 0 && pat_idx < pattern.len() && pattern[pat_idx] == curr_len {
                result += count_permutations(s, pattern, idx + 1, pat_idx + 1, 0, memo);
            } else if c == '#' {
                result += count_permutations(s, pattern, idx + 1, pat_idx, curr_len + 1, memo);
            }
        } 
    }
    memo.insert((idx, pat_idx, curr_len), result);
    result
}