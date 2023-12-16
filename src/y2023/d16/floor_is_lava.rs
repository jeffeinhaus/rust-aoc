use crate::utils;
use std::path::Path;
use std::collections::HashSet;
use rayon::prelude::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,

}

pub fn floor_is_lava_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut layout = vec![];
    let start = (0, 0);
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        layout.push(row);
    }
    
    println!("Floor is lava part one: {}", find_energized_tiles(&layout, &start, &Direction::Right));
}

pub fn floor_is_lava_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut layout = vec![];
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        layout.push(row);
    }
    
    let mut starts: Vec<(usize, usize, Direction)> = Vec::new();
    for i in 0..layout.len() {
        starts.push((i, 0, Direction::Right));
    }
    for i in 0..layout[0].len() {
        starts.push((0, i, Direction::Down));
    }
    for i in 0..layout.len() {
        starts.push((i, layout[0].len() - 1, Direction::Left));
    }
    for i in 0..layout[0].len() {
        starts.push((layout.len() - 1, i, Direction::Up));
    }

    let max = starts.par_iter().map(|x: &(usize, usize, Direction)| find_energized_tiles(&layout, &(x.0, x.1), &x.2)).max().unwrap();
    println!("Floor is lava part two: {}", max);
}

fn find_energized_tiles(layout: &Vec<Vec<char>>, start: &(usize, usize), start_dir: &Direction) -> usize {
    let mut queue = vec![(start.0, start.1, start_dir)];
    let mut visited = HashSet::new();
    let mut visited_lens = Vec::new();

    while !queue.is_empty() {
        if visited_lens.len() > 10 {
            let mut same = true;
            for i in 0..10 {
                if visited_lens[visited_lens.len() - i - 1] != visited_lens[visited_lens.len() - i - 2] {
                    same = false;
                    break;
                }
            }
            if same {
                break;
            }
        }
        visited_lens.push(visited.len());
        let mut next_queue = vec![];
        for (row, col, dir) in queue {
            let mut next_steps = move_beam(&layout, row, col, &dir);
            next_queue.append(&mut next_steps);
            visited.insert((row, col));
        }
        queue = next_queue;
    }
    visited.len()
}

fn move_beam<'a>(layout: &'a Vec<Vec<char>>, row: usize, col: usize, dir: &'a Direction) -> Vec<(usize, usize, &'a Direction)> {
    match layout[row][col] {
        '-' => {
            match dir {
                Direction::Left => {
                    if col == 0 {
                        vec![]
                    } else {
                        vec![(row, col - 1, &Direction::Left)]
                    }
                },
                Direction::Right => {
                    if col == layout[0].len() - 1 {
                        vec![]
                    } else {
                        vec![(row, col + 1, &Direction::Right)]
                    }
                },
                Direction::Up | Direction::Down => {
                    if col == 0 {
                        vec![(row, col + 1, &Direction::Right)]
                    } else if col == layout[0].len() - 1 {
                        vec![(row, col - 1, &Direction::Left)]
                    } else {
                        vec![(row, col - 1, &Direction::Left), (row, col + 1, &Direction::Right)]
                    }
                },
            }
        },
        '|' => {
            match dir {
                Direction::Up => {
                    if row == 0 {
                        vec![]
                    } else {
                        vec![(row - 1, col, &Direction::Up)]
                    }
                },
                Direction::Down => {
                    if row == layout.len() - 1 {
                        vec![]
                    } else {
                        vec![(row + 1, col, &Direction::Down)]
                    }
                },
                Direction::Left | Direction::Right => {
                    if row == 0 {
                        vec![(row + 1, col, &Direction::Down)]
                    } else if row == layout.len() - 1 {
                        vec![(row - 1, col, &Direction::Up)]
                    } else {
                        vec![(row - 1, col, &Direction::Up), (row + 1, col, &Direction::Down)]
                    }
                },
            }
        },
        '\\' => {
            match dir {
                Direction::Up => {
                    if col == 0 {
                        vec![]
                    } else {
                        vec![(row, col - 1, &Direction::Left)]
                    }
                },
                Direction::Down => {
                    if col == layout[0].len() - 1 {
                        vec![]
                    } else {
                        vec![(row, col + 1, &Direction::Right)]
                    }
                },
                Direction::Left => {
                    if row == 0 {
                        vec![]
                    } else {
                        vec![(row - 1, col, &Direction::Up)]
                    }
                },
                Direction::Right => {
                    if row == layout.len() - 1 {
                        vec![]
                    } else {
                        vec![(row + 1, col, &Direction::Down)]
                    }
                },
            }
        },
        '/' => {
            match dir {
                Direction::Up => {
                    if col == layout[0].len() - 1 {
                        vec![]
                    } else {
                        vec![(row, col + 1, &Direction::Right)]
                    }
                },
                Direction::Down => {
                    if col == 0 {
                        vec![]
                    } else {
                        vec![(row, col - 1, &Direction::Left)]
                    }
                },
                Direction::Left => {
                    if row == layout.len() - 1 {
                        vec![]
                    } else {
                        vec![(row + 1, col,&Direction::Down)]
                    }
                },
                Direction::Right => {
                    if row == 0 {
                        vec![]
                    } else {
                        vec![(row - 1, col, &Direction::Up)]
                    }
                },
            }
        },
        _ => {
            match dir {
                Direction::Up => {
                    if row == 0 {
                        vec![]
                    } else {
                        vec![(row - 1, col, &Direction::Up)]
                    }
                },
                Direction::Down => {
                    if row == layout.len() - 1 {
                        vec![]
                    } else {
                        vec![(row + 1, col, &Direction::Down)]
                    }
                },
                Direction::Left => {
                    if col == 0 {
                        vec![]
                    } else {
                        vec![(row, col - 1, &Direction::Left)]
                    }
                },
                Direction::Right => {
                    if col == layout[0].len() - 1 {
                        vec![]
                    } else {
                        vec![(row, col + 1, &Direction::Right)]
                    }
                },
            }
        },
    }
}