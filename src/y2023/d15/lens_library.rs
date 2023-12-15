use crate::utils;
use std::path::Path;

pub fn lens_library_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let sequence = lines[0].split(",").collect::<Vec<&str>>();
    let sum = sequence.iter().map(|x| get_hash_value(x)).sum::<u64>();
    println!("Lens library part one: {}", sum);
}

pub fn lens_library_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let sequence = lines[0].split(",").collect::<Vec<&str>>();
    let mut boxes: Vec<Vec<&str>> = vec![Vec::new(); 256];
    for s in sequence {
        let (box_num, is_eq) = get_hashmap_value(s);
        if is_eq {
            let label = s.split("=").collect::<Vec<&str>>()[0];
            let mut found = false;
            for (i, lens) in boxes[box_num as usize].iter().enumerate() {
                if lens.starts_with(label) {
                    boxes[box_num as usize][i] = s;
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[box_num as usize].push(s);
            }
        } else {
            let label = s.split("-").collect::<Vec<&str>>()[0];
            let mut index_to_remove: i32 = -1;
            for (i, lens) in boxes[box_num as usize].iter().enumerate() {
                if lens.starts_with(label) {
                    index_to_remove = i as i32;
                    break;
                }
            }
            if index_to_remove >= 0 {
                boxes[box_num as usize].remove(index_to_remove as usize);
            }
        }
    }
    let mut sum = 0;
    for (i, box_) in boxes.iter().enumerate() {
        if box_.len() > 0 {
            let focal_lengths = box_.iter().map(|x| x.split("=").collect::<Vec<&str>>()[1].parse::<u64>().unwrap()).collect::<Vec<u64>>();
            for (j, focal_length) in focal_lengths.iter().enumerate() {
                sum += focal_length * (j as u64 + 1) * (i as u64 + 1);
            }
        }
    }
    println!("Lens library part two: {}", sum);
}

fn get_hash_value(hash: &str) -> u64 {
    let mut hash_value = 0;
    for c in hash.chars() {
        hash_value += c as u64;
        hash_value *= 17;
        hash_value %= 256;
    }
    hash_value
}

fn get_hashmap_value(hash: &str) -> (u64, bool) {
    let mut hash_value = 0;
    let mut is_eq = true;
    for c in hash.chars() {
        if c == '=' {
            is_eq = true;
            break;
        }
        if c == '-' {
            is_eq = false;
            break;
        }
        hash_value += c as u64;
        hash_value *= 17;
        hash_value %= 256;
    }
    (hash_value, is_eq)
}