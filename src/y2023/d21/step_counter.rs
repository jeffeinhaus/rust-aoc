use crate::utils;
use std::path::Path;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    row: isize,
    col: isize,
}

fn take_step(garden: &Vec<Vec<char>>, positions: HashSet<Pos>) -> HashSet<Pos> {
    let mut new_positions = HashSet::new();
    for position in positions {
        if position.row > 0 && garden[(position.row - 1) as usize][position.col as usize] == '.' {
            new_positions.insert(Pos {
                row: position.row - 1,
                col: position.col,
            });
        }
        if (position.row as usize) < garden.len() - 1 && garden[(position.row + 1) as usize][position.col as usize] == '.' {
            new_positions.insert(Pos {
                row: position.row + 1,
                col: position.col,
            });
        }
        if position.col > 0 && garden[position.row as usize][(position.col - 1) as usize] == '.' {
            new_positions.insert(Pos {
                row: position.row,
                col: position.col - 1,
            });
        }
        if (position.col as usize) < garden[0].len() - 1 && garden[position.row as usize][(position.col + 1) as usize] == '.' {
            new_positions.insert(Pos {
                row: position.row,
                col: position.col + 1,
            });
        }        
    }
    new_positions
}

pub fn step_counter_part_one<P: AsRef<Path>>(path: P, steps: usize) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut garden = Vec::new();
    let mut positions = HashSet::new();
    for (row_idx, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (col_idx, c) in line.chars().enumerate() {
            if c == 'S' {
                positions.insert(Pos {
                    row: row_idx as isize,
                    col: col_idx as isize,
                });
                row.push('.')
            } else {
                row.push(c);
            }
        }
        garden.push(row);
    }
    for _ in 0..steps {
        positions = take_step(&garden, positions);
    }
    println!("Step counter part one: {}", positions.len());
}

pub fn step_counter_part_two<P: AsRef<Path>>(path: P, steps: usize) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut garden: HashMap<(i32, i32), char> = HashMap::new();
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if ".S".contains(c) {
                garden.insert((row_idx as i32, col_idx as i32), c);
            }
        }
    }
    let line_len = lines.len() as i32;

    let mut done = vec![];
    let mut todo: HashSet<(i32, i32)> = garden.iter()
        .filter(|(_, &v)| v == 'S')
        .map(|(&k, _)| k)
        .collect();
    let cmod = |x: (i32, i32)| -> (i32, i32) {
        ((x.0 % line_len + line_len) % line_len, (x.1 % line_len + line_len) % line_len)
    };

    for s in 0..(3 * line_len) {
        if s % line_len == line_len / 2 {
            done.push(todo.len());
        }

        todo = todo.iter()
            .flat_map(|&p| vec![(p.0 + 1, p.1), (p.0 - 1, p.1), (p.0, p.1 + 1), (p.0, p.1 - 1)])
            .filter(|&d| garden.contains_key(&cmod(d)))
            .collect();
    }

    let f = |n, a, b, c| a as i64 + n as i64 * (b as i64 - a as i64 + (n as i64 - 1) * (c as i64 - 2 * b as i64 + a as i64) / 2);
    println!("Step counter part two: {}", f(steps / lines.len(), done[0], done[1], done[2]));
}

