use crate::utils;
use std::path::Path;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    x: i32,
    y: i32,
    weight: u32,
    direction: Option<Direction>,
    consecutive_moves: u8,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Node {
    fn new(x: i32, y: i32, weight: u32) -> Self {
        Self {
            x,
            y,
            weight,
            direction: None,
            consecutive_moves: 0,
        }
    }

    fn successors_part_one(&self, grid: &Vec<Vec<usize>>) -> Vec<(Node, u32)> {
        let mut successors = Vec::new();
        if valid_position(self.x + 1, self.y, grid) && self.direction != Some(Direction::Left) {
            let weight = grid[self.y as usize][(self.x + 1) as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Right) => self.consecutive_moves + 1,
                _ => 0,
            };
            if consecutive_moves < 3 {
                successors.push((
                    Node {
                        x: self.x+ 1,
                        y: self.y,
                        weight,
                        direction: Some(Direction::Right),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if valid_position(self.x - 1, self.y, grid) && self.direction != Some(Direction::Right) {
            let weight = grid[self.y as usize][(self.x - 1) as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Left) => self.consecutive_moves + 1,
                _ => 0,
            };
            if consecutive_moves < 3 {
                successors.push((
                    Node {
                        x: self.x - 1,
                        y: self.y,
                        weight,
                        direction: Some(Direction::Left),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if valid_position(self.x, self.y + 1, grid) && self.direction != Some(Direction::Up) {
            let weight = grid[(self.y + 1) as usize][self.x as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Down) => self.consecutive_moves + 1,
                _ => 0,
            };
            if consecutive_moves < 3 {
                successors.push((
                    Node {
                        x: self.x,
                        y: self.y + 1,
                        weight,
                        direction: Some(Direction::Down),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if valid_position(self.x, self.y - 1, grid) && self.direction != Some(Direction::Down) {
            let weight = grid[(self.y - 1) as usize][self.x as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Up) => self.consecutive_moves + 1,
                _ => 0,
            };
            if consecutive_moves < 3 {
                successors.push((
                    Node {
                        x: self.x,
                        y: self.y - 1,
                        weight,
                        direction: Some(Direction::Up),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        successors
    }

    fn successors_part_two(&self, grid: &Vec<Vec<usize>>) -> Vec<(Node, u32)> {
        let mut successors = Vec::new();
        let can_change_direction = self.consecutive_moves == 0 || self.consecutive_moves >= 4;
        if (can_change_direction || self.direction == Some(Direction::Right)) && valid_position(self.x + 1, self.y, grid) && self.direction != Some(Direction::Left) {
            let weight = grid[self.y as usize][(self.x + 1) as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Right) | None => self.consecutive_moves + 1,
                _ => 1,
            };
            if consecutive_moves <= 10 {
                successors.push((
                    Node {
                        x: self.x+ 1,
                        y: self.y,
                        weight,
                        direction: Some(Direction::Right),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if (can_change_direction || self.direction == Some(Direction::Left)) && valid_position(self.x - 1, self.y, grid) && self.direction != Some(Direction::Right) {
            let weight = grid[self.y as usize][(self.x - 1) as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Left) | None => self.consecutive_moves + 1,
                _ => 1,
            };
            if consecutive_moves <= 10 {
                successors.push((
                    Node {
                        x: self.x - 1,
                        y: self.y,
                        weight,
                        direction: Some(Direction::Left),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if (can_change_direction || self.direction == Some(Direction::Down)) && valid_position(self.x, self.y + 1, grid) && self.direction != Some(Direction::Up) {
            let weight = grid[(self.y + 1) as usize][self.x as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Down) | None => self.consecutive_moves + 1,
                _ => 1,
            };
            if consecutive_moves <= 10 {
                successors.push((
                    Node {
                        x: self.x,
                        y: self.y + 1,
                        weight,
                        direction: Some(Direction::Down),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        if (can_change_direction || self.direction == Some(Direction::Up)) && valid_position(self.x, self.y - 1, grid) && self.direction != Some(Direction::Down) {
            let weight = grid[(self.y - 1) as usize][self.x as usize] as u32;
            let consecutive_moves = match self.direction {
                Some(Direction::Up) | None => self.consecutive_moves + 1,
                _ => 1,
            };
            if consecutive_moves <= 10 {
                successors.push((
                    Node {
                        x: self.x,
                        y: self.y - 1,
                        weight,
                        direction: Some(Direction::Up),
                        consecutive_moves,
                    },
                    weight,
                ));
            }
        }
        successors
    }
}

fn is_goal(node: &Node, goal: &Node) -> bool {
    node.x == goal.x && node.y == goal.y
}

fn valid_position(x: i32, y: i32, grid: &Vec<Vec<usize>>) -> bool {
    x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
}


pub fn clumsy_crucible_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let num_grid = lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let grid = num_grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, weight)| Node::new(x as i32, y as i32, *weight as u32))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let start = Node::new(0, 0, 0);
    let goal = Node::new((grid[0].len() - 1) as i32, (grid.len() - 1) as i32, 0);
    let result = dijkstra(&start, |node| node.successors_part_one(&num_grid), |node| is_goal(node, &goal));
    println!("Clumsy crucible part one: {}", result.unwrap().1);
}

pub fn clumsy_crucible_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let num_grid = lines
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let grid = num_grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, weight)| Node::new(x as i32, y as i32, *weight as u32))
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    let start = Node::new(0, 0, 0);
    let goal = Node::new((grid[0].len() - 1) as i32, (grid.len() - 1) as i32, 0);
    let result = dijkstra(&start, |node| node.successors_part_two(&num_grid), |node| is_goal(node, &goal));
    println!("Clumsy crucible part two: {}", result.unwrap().1);
}