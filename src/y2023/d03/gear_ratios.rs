use crate::utils;
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;

struct Gear {
    num: u32,
    x_range: Vec<usize>,
    y: usize,
}

pub fn gear_ratios_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    let mut line_count = 0;
    let re = Regex::new(r"\d+").unwrap();
    for line in &lines {
        let matches = re.find_iter(&line);
        for m in matches {
            let num = &line[m.start()..m.end()];
            let x = m.start() as usize;
            let y = line_count;
            if is_adjacent_to_symbol(&lines, num, x, y) {
                sum += num.parse::<u32>().unwrap();
            }
        }
        line_count += 1;
    }
    println!("Gear Ratios part one: {}", sum);
}

pub fn gear_ratios_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    let mut line_count = 0;
    let num_re = Regex::new(r"\d+").unwrap();
    let mut gears = Vec::new();
    for line in &lines {
        let matches = num_re.find_iter(&line);
        for m in matches {
            gears.push(Gear {
                num: line[m.start()..m.end()].parse::<u32>().unwrap(),
                x_range: (m.start()..m.end()).collect(),
                y: line_count,
            });
        }
        line_count += 1;
    }
    let star_reg = Regex::new(r"\*").unwrap();
    line_count = 0;
    for line in &lines {
        let matches = star_reg.find_iter(&line);
        for m in matches {
            if let Some((x, y)) = is_adjacent_to_gears(&gears, m.start() as usize, line_count) {
                sum += x * y;
            }
        }
        line_count += 1;
    }
    println!("Gear Ratios part two: {}", sum);
}

fn is_adjacent_to_gears(gears: &Vec<Gear>, x: usize, y: usize) -> Option<(u32, u32)> {
    let mut unique_options = HashSet::new();
    for gear in gears {
        let gear_y32 = gear.y as i32;
        let adjacent_options = [
            if gear.x_range.iter().any(|&i| i == x + 1) && gear.y == y { Some(gear.num) } else { None },
            if gear.x_range.iter().any(|&i| i == x - 1) && gear.y == y { Some(gear.num) } else { None },
            if gear.y + 1 == y && gear.x_range.iter().any(|&i| i == x) { Some(gear.num) } else { None },
            if gear_y32 - 1 >= 0 && gear.y - 1 == y && gear.x_range.iter().any(|&i| i == x) { Some(gear.num) } else { None },
            if gear.y + 1 == y && gear.x_range.iter().any(|&i| i == x + 1) { Some(gear.num) } else { None },
            if gear.y + 1 == y && gear.x_range.iter().any(|&i| i == x - 1) { Some(gear.num) } else { None },
            if gear_y32 - 1 >= 0 && gear.y - 1 == y && gear.x_range.iter().any(|&i| i == x + 1) { Some(gear.num) } else { None },
            if gear_y32 - 1 >= 0 && gear.y - 1 == y && gear.x_range.iter().any(|&i| i == x - 1) { Some(gear.num) } else { None },
        ];

        let adjacent_options: Vec<u32> = adjacent_options.iter().filter_map(|&x| x).collect();
        for option in adjacent_options {
            unique_options.insert(option);
        }
    }
    if unique_options.len() == 2 {
        let unique_options: Vec<u32> = unique_options.into_iter().collect();
        return Some((unique_options[0], unique_options[1]));
    }
    
    return None;
}

fn is_adjacent_to_symbol(lines: &Vec<String>, num: &str, x: usize, y: usize) -> bool {
    let num_len = num.len();
    let all_lines_length = lines.len() as i32;
    let line_length = lines.get(y).unwrap().len() as i32;
    for i in x..x+num_len {
        let i32_value = i as i32;
        let y32_value = y as i32;
        let adjacent_options = [
            if i32_value + 1 < line_length { Some(lines.get(y).unwrap().chars().nth(i + 1).unwrap()) } else { None },
            if i32_value - 1 >= 0 { Some(lines.get(y).unwrap().chars().nth(i - 1).unwrap()) } else { None },
            if y32_value + 1 < all_lines_length { Some(lines.get(y + 1).unwrap().chars().nth(i).unwrap()) } else { None },
            if y32_value - 1 >= 0 { Some(lines.get(y - 1).unwrap().chars().nth(i).unwrap()) } else { None },
            if y32_value + 1 < all_lines_length && i32_value + 1 < line_length { Some(lines.get(y + 1).unwrap().chars().nth(i + 1).unwrap()) } else { None },
            if y32_value + 1 < all_lines_length && i32_value - 1 >= 0 { Some(lines.get(y + 1).unwrap().chars().nth(i - 1).unwrap()) } else { None },
            if y32_value - 1 >= 0 && i32_value + 1 < line_length { Some(lines.get(y - 1).unwrap().chars().nth(i + 1).unwrap()) } else { None },
            if y32_value- 1 >= 0 && i32_value - 1 >= 0 { Some(lines.get(y - 1).unwrap().chars().nth(i - 1).unwrap()) } else { None },
        ];
        if adjacent_options.iter().any(|&x| {
            if let Some(c) = x {
                !c.is_numeric() && c != '.'
            } else {
                false
            }
        }) {
            return true;
        }
    }
    return false;
}