use crate::utils;
use std::path::Path;
use regex::Regex;

pub fn trebuchet_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for line in lines {
        let mut calibration = String::new();
        let re = Regex::new(r"\d").unwrap();
        if let Some(matched) = re.find(&line) {
            let first_match = &line[matched.start()..matched.end()];
            calibration.push_str(first_match);
        }
        let reversed = line.chars().rev().collect::<String>();
        if let Some(matched) = re.find(&reversed) {
            let second_match = &reversed[matched.start()..matched.end()];
            calibration.push_str(second_match);
        }
        let calibration: i32 = calibration.parse().unwrap();
        sum += calibration;
    }
    println!("Trebuchet part one: {}", sum);
}

pub fn trebuchet_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    for line in lines {
        let mut calibration = String::new();
        let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        if let Some(matched) = re.find(&line) {
            let first_match = &line[matched.start()..matched.end()];
            calibration.push_str(get_match(first_match));
        }
        let len = line.len();
        for i in 0..len {
            let test_string = &line[len-i-1..];
            if let Some(matched) = re.find(test_string) {
                let second_match = &test_string[matched.start()..matched.end()];
                calibration.push_str(get_match(second_match));
                break;
            }
        }
        let calibration: i32 = calibration.parse().unwrap();
        sum += calibration;
    }
    println!("Trebuchet part two: {}", sum);
}

fn get_match(line: &str) -> &str {
    match line {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => line,
    }
}