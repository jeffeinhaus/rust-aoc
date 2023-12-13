use crate::utils;
use std::path::Path;

pub fn point_of_incidence_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    let mut patterns = Vec::new();
    let mut current_pattern = Vec::new();
    for line in lines {
        if line == "" {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            current_pattern.push(line.chars().collect::<Vec<char>>());
        }
    }
    if current_pattern.len() > 0 {
        patterns.push(current_pattern);
    }
    
    for pattern in patterns {
        if let Some(i) = find_vertical_reflection(&pattern) {
            sum += i;
        }
        if let Some(i) = find_horizontal_reflection(&pattern) {
            sum += i * 100;
        }
    }
    println!("Point of Incidence part one: {}", sum);
}

pub fn point_of_incidence_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    let mut patterns = Vec::new();
    let mut current_pattern = Vec::new();
    for line in lines {
        if line == "" {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            current_pattern.push(line.chars().collect::<Vec<char>>());
        }
    }
    if current_pattern.len() > 0 {
        patterns.push(current_pattern);
    }
    
    for pattern in patterns {
        if let Some(i) = find_vertical_reflection_with_smudges(&pattern) {
            sum += i;
        }
        if let Some(i) = find_horizontal_reflection_with_smudges(&pattern) {
            sum += i * 100;
        }
    }
    println!("Point of Incidence part two: {}", sum);
}

fn find_vertical_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..pattern[0].len()-1 {
        let mut left_bound: i32 = i as i32;
        let mut right_bound : i32 = i as i32 + 1;
        let mut valid = true;
        while left_bound >= 0 && right_bound < pattern[0].len() as i32 {
            let mut col_valid = true;
            for j in 0..pattern.len() {
                if pattern[j][left_bound as usize] != pattern[j][right_bound as usize] {
                    col_valid = false;
                    break;
                }
            }
            if !col_valid {
                valid = false;
                break;
            }
            left_bound -= 1;
            right_bound += 1;
        }
        if valid {
            return Some(i + 1);
        }
    }
    None
}

fn find_horizontal_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..pattern.len()-1 {
        let mut upper_bound: i32 = i as i32;
        let mut lower_bound : i32= i as i32 + 1;
        let mut valid = true;
        while upper_bound >= 0 && lower_bound < pattern.len() as i32 {
            if pattern[upper_bound as usize] != pattern[lower_bound as usize] {
                valid = false;
                break;
            }
            upper_bound -= 1;
            lower_bound += 1;
        }
        if valid {
            return Some(i + 1);
        }
    }
    None
}

fn find_horizontal_reflection_with_smudges(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..pattern.len()-1 {
        let mut upper_bound: i32 = i as i32;
        let mut lower_bound : i32= i as i32 + 1;
        let mut valid = true;
        let mut smudges = 0;
        while upper_bound >= 0 && lower_bound < pattern.len() as i32 {
            let mut row_valid = true;
            for j in 0..pattern[i].len() {
                if pattern[upper_bound as usize][j] != pattern[lower_bound as usize][j] {
                    smudges += 1;
                    if smudges > 1 {
                        row_valid = false;
                        break;
                    }
                }
            }
            if !row_valid {
                valid = false;
                break;
            }
            upper_bound -= 1;
            lower_bound += 1;
        }
        if valid && smudges == 1 {
            return Some(i + 1);
        }
    }
    None
}

fn find_vertical_reflection_with_smudges(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 0..pattern[0].len()-1 {
        let mut left_bound: i32 = i as i32;
        let mut right_bound : i32 = i as i32 + 1;
        let mut valid = true;
        let mut smudges = 0;
        while left_bound >= 0 && right_bound < pattern[0].len() as i32 {
            let mut col_valid = true;
            for j in 0..pattern.len() {
                if pattern[j][left_bound as usize] != pattern[j][right_bound as usize] {
                    smudges += 1;
                    if smudges > 1 {
                        col_valid = false;
                        break;
                    }
                }
            }
            if !col_valid {
                valid = false;
                break;
            }
            left_bound -= 1;
            right_bound += 1;
        }
        if valid && smudges == 1 {
            return Some(i + 1);
        }
    }
    None
}