use crate::utils;
use std::path::Path;

fn area(instructions: &Vec<(u8, isize)>) -> isize {
    let (mut area, mut y, mut x) = (0, 0, 0);
    for (direction, dist) in instructions {
        let (dy, dx) = (y, x);
        match direction {
            b'U' => y -= dist,
            b'D' => y += dist,
            b'L' => x -= dist,
            b'R' => x += dist,
            _ => panic!("Unknown direction: {}", direction),
        };
        area +=  (x + dx) * (y - dy) + dist;
    }
    area / 2 + 1
}

pub fn lavaduct_lagoon_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let instructions = lines.iter().map(|l| {
        let (dist, _) = l[2..].split_once(' ').unwrap();
        (l.as_bytes()[0], dist.parse::<isize>().unwrap())
    }).collect::<Vec<(u8, isize)>>();
    println!("Lavaduct lagoon part one: {}", area(&instructions));
}

pub fn lavaduct_lagoon_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let instructions = lines.iter().map(|l| {
        let (_, color) = l.split_once(" (#").unwrap();
        let direction = match color.as_bytes()[color.len() - 2] {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => panic!("Unknown color: {}", color),
        };
        (direction, isize::from_str_radix(&color[..color.len() - 2], 16).unwrap())
    }).collect::<Vec<(u8, isize)>>();
    println!("Lavaduct lagoon part two: {}", area(&instructions));
}