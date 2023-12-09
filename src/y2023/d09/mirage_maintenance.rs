use crate::utils;
use std::path::Path;

pub fn mirage_maintenance_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for line in lines {
        let sequence = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += extrapolate_forwards(&sequence);
    }

    println!("Mirage maintenance part one: {}", sum);
}

pub fn mirage_maintenance_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for line in lines {
        let sequence = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += extrapolate_backwards(&sequence);
    }

    println!("Mirage maintenance part two: {}", sum);
}

fn extrapolate_forwards(sequence: &Vec<i32>) -> i32 {
    let mut sequences = vec![sequence.clone()];
    while sequences[sequences.len() - 1].iter().any(|&x| x != 0) {
        let mut new_sequence = Vec::new();
        let last_sequence: Vec<i32> = sequences[sequences.len() - 1].clone();
        for i in 0..last_sequence.len()-1 {
            new_sequence.push(last_sequence[i+1] - last_sequence[i]);
        }
        sequences.push(new_sequence);
    }
    let sequences_length = sequences.len();
    sequences[sequences_length-1].push(0);
    for i in (0..sequences.len()-2).rev() {
        let next_sequence = sequences[i+1].clone();
        let current_sequence = sequences[i].clone();
        sequences[i].push(next_sequence[next_sequence.len() - 1] + current_sequence[current_sequence.len() - 1]);
    }
    sequences[0][sequences[0].len()-1]   
}

fn extrapolate_backwards(sequence: &Vec<i32>) -> i32 {
    let mut sequences = vec![sequence.clone()];
    while sequences[sequences.len() - 1].iter().any(|&x| x != 0) {
        let mut new_sequence = Vec::new();
        let last_sequence: Vec<i32> = sequences[sequences.len() - 1].clone();
        for i in 0..last_sequence.len()-1 {
            new_sequence.push(last_sequence[i+1] - last_sequence[i]);
        }
        sequences.push(new_sequence);
    }
    let sequences_length = sequences.len();
    sequences[sequences_length-1].insert(0, 0);
    for i in (0..sequences.len()-2).rev() {
        let next_sequence = sequences[i+1].clone();
        let current_sequence = sequences[i].clone();
        sequences[i].insert(0, current_sequence[0] - next_sequence[0])
    }
    sequences[0][0] 
}