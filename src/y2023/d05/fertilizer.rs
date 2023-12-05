use crate::utils;
use std::path::Path;

pub fn fertilizer_part_one<P: AsRef<Path>>(path: P) {
    find_location(true, &path);
}

pub fn fertilizer_part_two<P: AsRef<Path>>(path: P) {
    find_location(false, &path);
}

fn find_location<P: AsRef<Path>>(is_part_one: bool, path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut seeds = Vec::new();
    let mut seed_mapped = Vec::new();
    for line in lines {
        if line.starts_with("seeds: ") {
            let seed_line = line.split(": ").collect::<Vec<&str>>();
            seeds = if is_part_one {
                parse_seeds_simple(seed_line[1])
            } else {
                parse_seeds_range(seed_line[1])
            };
            seed_mapped = seeds.iter().map(|_| false).collect();
        } else if line.chars().next().map_or(false, |c| c.is_digit(10)) {
            let map_line = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            let dest_range_start = map_line[0];
            let source_range_start = map_line[1];
            let range_length = map_line[2];
            for (i, seed) in seeds.iter_mut().enumerate() {
                if !seed_mapped[i] && *seed >= source_range_start && *seed < source_range_start + range_length {
                    *seed = dest_range_start + (*seed - source_range_start);
                    seed_mapped[i] = true;
                }
            }

        } else if line.chars().next().map_or(false, |c| c.is_alphabetic()) {
            seed_mapped = seeds.iter().map(|_| false).collect();
        }
    }
    if is_part_one {
        println!("Fertilizer part one: {}", seeds.iter().min().unwrap());
    } else {
        println!("Fertilizer part two: {}", seeds.iter().min().unwrap());
    }
}

fn parse_seeds_simple(seeds: &str) -> Vec<u64> {
    seeds.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

fn parse_seeds_range(seeds: &str) -> Vec<u64> {
    let mut seeds_vec = Vec::new();
    let seed_nums = seeds.split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for i in (0..seed_nums.len()).step_by(2) {
        let start = seed_nums[i];
        let range = seed_nums[i+1];
        seeds_vec.extend(start..start+range);
    }
    seeds_vec
}