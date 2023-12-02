use crate::utils;
use std::path::Path;
use std::cmp::max;

struct Bag {
    red: u64,
    blue: u64,
    green: u64,
}

pub fn cube_conundrum_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum: u64 = 0;
    for line in lines {
        let game_contents = line.split(": ").collect::<Vec<&str>>();
        let bag_grabs = game_contents[1].split("; ").collect::<Vec<&str>>();
        let mut game_failed = false;
        for grab in bag_grabs {
            let mut bag = Bag { red: 0, blue: 0, green: 0 };
            let grab_contents = grab.split(", ").collect::<Vec<&str>>();
            for cubes in grab_contents {
                let cube_contents = cubes.split(" ").collect::<Vec<&str>>();
                let cube_color = cube_contents[1];
                let cube_count = cube_contents[0].parse::<u64>().unwrap();
                match cube_color {
                    "red" => bag.red += cube_count,
                    "blue" => bag.blue += cube_count,
                    "green" => bag.green += cube_count,
                    _ => (),
                }
            }
            if bag.red > 12 || bag.blue > 14 || bag.green > 13 {
                game_failed = true;
                break;
            }
        }
        if !game_failed {
            let game_info = game_contents[0].split(" ").collect::<Vec<&str>>();
            sum += game_info[1].parse::<u64>().unwrap();
        }
    }
    println!("Cube Conundrum part one: {}", sum);
}

pub fn cube_conundrum_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum: u64 = 0;
    for line in lines {
        let game_contents = line.split(": ").collect::<Vec<&str>>();
        let bag_grabs = game_contents[1].split("; ").collect::<Vec<&str>>();
        let mut bag = Bag { red: 0, blue: 0, green: 0 };
        for grab in bag_grabs {
            let grab_contents = grab.split(", ").collect::<Vec<&str>>();
            for cubes in grab_contents {
                let cube_contents = cubes.split(" ").collect::<Vec<&str>>();
                let cube_color = cube_contents[1];
                let cube_count = cube_contents[0].parse::<u64>().unwrap();
                match cube_color {
                    "red" => bag.red = max(bag.red, cube_count),
                    "blue" => bag.blue = max(bag.blue, cube_count),
                    "green" => bag.green = max(bag.green, cube_count),
                    _ => (),
                }
            }
        }
        sum += bag.red * bag.blue * bag.green;
    }
    println!("Cube Conundrum part two: {}", sum);
}
