use crate::utils;
use std::path::Path;

pub fn calorie_counting_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut calories = 0;
    let mut max_calories = 0;
    for line in lines {
        if line == "" {
            max_calories = std::cmp::max(max_calories, calories);
            calories = 0;
        } else {
            let tmp_calories : i32 = line.parse().unwrap();
            calories += tmp_calories;
        }
    }
    println!("Calories Part 1: {}", max_calories);
}

pub fn calorie_counting_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut calories = 0;
    let mut calorie_counts = Vec::new();
    for line in lines {
        if line == "" {
            calorie_counts.push(calories);
            calories = 0;
        } else {
            let tmp_calories : i32 = line.parse().unwrap();
            calories += tmp_calories;
        }
    }
    calorie_counts.push(calories);
    calorie_counts.sort();
    calorie_counts.reverse();
    let mut result = 0;
    for i in 0..=2 {
        result += calorie_counts[i];
    }
    println!("Calories Part 2: {}", result);
}