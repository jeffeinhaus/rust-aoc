use crate::utils;
use core::panic;
use std::path::Path;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,

}

pub fn pipe_maze_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut maze = Vec::new();
    let mut start = Node { x: 0, y: 0 };
    for (i, line) in lines.iter().enumerate() {
        maze.push(line.chars().collect::<Vec<char>>());
        if line.contains('S') {
            start = Node {
                x: line.chars().position(|x| x == 'S').unwrap(),
                y: i
            }
        }
    }
    let (con1,  con2) = find_start_pipe(&maze, &start);

    let mut con1_pos = con1.0;
    let mut con2_pos = con2.0;
    let mut con1_prev = con1.1;
    let mut con2_prev = con2.1;
    let mut steps = 1;
    while con1_pos.x != con2_pos.x || con1_pos.y != con2_pos.y {
        let (con1_move, con1_dir) = take_pipe(&maze, &con1_pos, &con1_prev);
        let (con2_move, con2_dir) = take_pipe(&maze, &con2_pos, &con2_prev);
        con1_pos = con1_move;
        con2_pos = con2_move;
        con1_prev = con1_dir;
        con2_prev = con2_dir;
        steps += 1;
    }

    println!("Pipe maze part one: {}", steps);

}

pub fn pipe_maze_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut maze = Vec::new();
    let mut start = Node { x: 0, y: 0 };
    for (i, line) in lines.iter().enumerate() {
        maze.push(line.chars().collect::<Vec<char>>());
        if line.contains('S') {
            start = Node {
                x: line.chars().position(|x| x == 'S').unwrap(),
                y: i
            }
        }
    }
    let (con1,  con2) = find_start_pipe(&maze, &start);
    let mut con1_pos = con1.0;
    let mut con2_pos = con2.0;
    let mut con1_prev = con1.1;
    let mut con2_prev = con2.1;
    let mut loop_nodes = Vec::new();
    loop_nodes.push(Node { x: start.x, y: start.y});
    loop_nodes.push(Node { x: con1_pos.x, y: con1_pos.y});
    loop_nodes.push(Node { x: con2_pos.x, y: con2_pos.y})
;    while con1_pos.x != con2_pos.x || con1_pos.y != con2_pos.y {
        let (con1_move, con1_dir) = take_pipe(&maze, &con1_pos, &con1_prev);
        let (con2_move, con2_dir) = take_pipe(&maze, &con2_pos, &con2_prev);
        con1_pos = con1_move;
        con2_pos = con2_move;
        con1_prev = con1_dir;
        con2_prev = con2_dir;
        loop_nodes.push(Node { x: con1_pos.x, y: con1_pos.y});
        loop_nodes.push(Node { x: con2_pos.x, y: con2_pos.y})
    }

    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            let mut found = false;
            for node in &loop_nodes {
                if node.x == j && node.y == i {
                    print!("{}", c);
                    found = true;
                    break;
                }
            }
            if !found {
                print!(" ");
            }
        }
        println!();
    }
}

fn find_start_pipe(maze: &Vec<Vec<char>>, start: &Node) -> ((Node, char), (Node, char)) {
    let above = if start.y == 0 { None } else { Some(maze[start.y - 1][start.x]) };
    let below = if start.y == maze.len() - 1 { None } else { Some(maze[start.y + 1][start.x]) };
    let right = if start.x == maze[0].len() - 1 { None } else { Some(maze[start.y][start.x + 1]) };
    let left = if start.x == 0 { None } else { Some(maze[start.y][start.x - 1]) };

    let connects_from_above = above == Some('|') || above == Some('F') || above == Some('7') ;
    let connects_from_below = below == Some('|') || below == Some('L') || below == Some('J');
    let connects_from_right = right == Some('-') || right == Some('J') || right == Some('7');
    let connects_from_left =  left == Some('-') || left == Some('L') || left == Some('F');

    if connects_from_above && connects_from_below {
        ((Node { y: start.y - 1, x: start.x }, 'N'), (Node {y: start.y + 1, x: start.x }, 'S'))
    } else if connects_from_above && connects_from_left {
        ((Node { y: start.y - 1, x: start.x }, 'N' ), (Node { y: start.y, x: start.x - 1 }, 'W'))
    } else if connects_from_above && connects_from_right {
        ((Node { y: start.y - 1, x: start.x }, 'N'), (Node { y: start.y, x: start.x + 1 }, 'E'))
    } else if connects_from_below && connects_from_left {
        ((Node { y: start.y + 1, x: start.x }, 'S'), (Node { y: start.y, x: start.x - 1 }, 'W'))
    } else if connects_from_below && connects_from_right {
        ((Node { y: start.y + 1, x: start.x}, 'S'), (Node { y: start.y, x: start.x + 1 }, 'E'))
    } else if connects_from_left && connects_from_right {
        ((Node { y: start.y, x: start.x - 1 }, 'W'), (Node { y: start.y, x: start.x + 1 }, 'E'))
    } else {
        panic!("No start pipe found");
    }
}

fn take_pipe(maze: &Vec<Vec<char>>, curr_pos: &Node, prev_dir: &char) -> (Node, char) {
    let pipe = maze[curr_pos.y][curr_pos.x];
    match pipe {
        '|' => {
            match &prev_dir {
                'N' => (Node { y: curr_pos.y - 1, x: curr_pos.x }, 'N'),
                'S' => (Node { y: curr_pos.y + 1, x: curr_pos.x }, 'S'),
                _ => panic!("Invalid direction")
            }
        },
        '-' => {
            match &prev_dir {
                'E' => (Node { y: curr_pos.y, x: curr_pos.x + 1 }, 'E'),
                'W' => (Node { y: curr_pos.y, x: curr_pos.x - 1 }, 'W'),
                _ => panic!("Invalid direction")
            }
        },
        'L' => {
            match &prev_dir {
                'S' => (Node { y: curr_pos.y, x: curr_pos.x + 1 }, 'E'),
                'W' => (Node { y: curr_pos.y - 1, x: curr_pos.x }, 'N'),
                _ => panic!("Invalid direction")
            }
        },
        'J' => {
            match &prev_dir {
                'E' => (Node { y: curr_pos.y - 1, x: curr_pos.x }, 'N'),
                'S' => (Node { y: curr_pos.y, x: curr_pos.x - 1 }, 'W'),
                _ => panic!("Invalid direction")
            }
        },
        '7' => {
            match &prev_dir {
                'E' => (Node { y: curr_pos.y + 1, x: curr_pos.x }, 'S'),
                'N' => (Node { y: curr_pos.y, x: curr_pos.x - 1 }, 'W'),
                _ => panic!("Invalid direction")
            }
        },
        'F' => {
            match &prev_dir {
                'N' => (Node { y: curr_pos.y, x: curr_pos.x + 1 }, 'E'),
                'W' => (Node { y: curr_pos.y + 1, x: curr_pos.x }, 'S'),
                _ => panic!("Invalid direction")
            }
        },
        _ => panic!("Invalid pipe")
    }
}