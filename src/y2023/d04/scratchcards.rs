use crate::utils;
use core::num;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

pub fn scratchcards_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum: u64 = 0;
    for line in lines {
        let scratchcard = line.split(": ").collect::<Vec<&str>>();
        let contents = scratchcard[1].split(" | ").collect::<Vec<&str>>();
        let re = Regex::new(r"\d+").unwrap();
        let my_numbers: Vec<u64> = re.find_iter(contents[0]).filter_map(|m| m.as_str().parse::<u64>().ok()).collect();
        let winning_numbers: Vec<u64> = re.find_iter(contents[1]).filter_map(|m| m.as_str().parse::<u64>().ok()).collect();
        let mut points = 0;
        for my_number in &my_numbers {
            if winning_numbers.contains(my_number) {
                points += 1;
            }
        }
        sum += if points == 0 { 0 }  else { 2u64.pow(points - 1) };
    }
    println!("Scratchcards part one: {}", sum);
}

pub fn scratchcards_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut match_map = HashMap::new();
    for line in lines {
        let scratchcard = line.split(": ").collect::<Vec<&str>>();
        let contents = scratchcard[1].split(" | ").collect::<Vec<&str>>();
        let re = Regex::new(r"\d+").unwrap();
        let my_numbers: Vec<u64> = re.find_iter(contents[0]).filter_map(|m| m.as_str().parse::<u64>().ok()).collect();
        let winning_numbers: Vec<u64> = re.find_iter(contents[1]).filter_map(|m| m.as_str().parse::<u64>().ok()).collect();
        let mut num_matches = 0;
        for my_number in &my_numbers {
            if winning_numbers.contains(my_number) {
                num_matches += 1;
            }
        }
        let card_id = re.find(scratchcard[0]).unwrap().as_str().parse::<u64>().unwrap();
        match_map.insert(card_id, num_matches);
    }
    let mut sum = 0;
    let mut copies = HashMap::new();
    for (key, _) in &match_map {
        copies.insert(key, 0);
    }
    for i in 1..match_map.len()+1 {
        let num_matches = match_map.get(&(i as u64)).unwrap();
        sum += 1 + copies.get(&(i as u64)).unwrap();
        for j in i+1..i+1+num_matches {
           *copies.get_mut(&(j as u64)).unwrap() += 1 + copies.get(&(i as u64)).unwrap(); 
        }
    }
    println!("Scratchcards part two: {}", sum);
}