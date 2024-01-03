use crate::utils;
use std::path::Path;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    row: usize,
    col: usize,
    slope: Option<Direction>,
}

fn get_neighbors(map: &Vec<Vec<char>>, visited: &Vec<(usize, usize)>, row: usize, col: usize, prev_slope: &Option<Direction>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    if row > 0 && map[row - 1][col] != '#' && !visited.contains(&(row - 1, col))  {
        if prev_slope.is_none() || prev_slope.as_ref().unwrap() == &Direction::Up {
            neighbors.push(Node {
                row: row - 1,
                col,
                slope: to_direction(map[row - 1][col]),
            });
        }
    }
    if row < map.len() - 1 && map[row + 1][col] != '#' && !visited.contains(&(row + 1, col)) {
        if prev_slope.is_none() || prev_slope.as_ref().unwrap() == &Direction::Down {
            neighbors.push(Node {
                row: row + 1,
                col,
                slope: to_direction(map[row + 1][col]),
            });
        }
    }
    if col > 0 && map[row][col - 1] != '#' && !visited.contains(&(row, col - 1)) {
        if prev_slope.is_none() || prev_slope.as_ref().unwrap() == &Direction::Left {
            neighbors.push(Node {
                row,
                col: col - 1,
                slope: to_direction(map[row][col - 1]),
            });
        } 
    }
    if col < map[0].len() - 1 && map[row][col + 1] != '#' && !visited.contains(&(row, col + 1)) {
        if prev_slope.is_none() || prev_slope.as_ref().unwrap() == &Direction::Right {
            neighbors.push(Node {
                row,
                col: col + 1,
                slope: to_direction(map[row][col + 1]),
            });
        }
    }
    neighbors
}

fn to_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None,
    }
}

fn dfs(
    map: &Vec<Vec<char>>, 
    current_path: &mut Vec<Node>, 
    all_paths: &mut Vec<Vec<Node>>, 
    current: Node, 
    destination: Node
) {
    let current_row = current.row;
    let current_col = current.col;
    let current_slope = current.slope.clone();
    current_path.push(current);

    if current_row == destination.row && current_col == destination.col {
        all_paths.push(current_path.clone());
    } else {
        let neighbors = get_neighbors(
            map, 
            &current_path.iter().map(|n| (n.row, n.col)).collect(), 
            current_row, 
            current_col, 
            &current_slope
        );

        for neighbor in neighbors {
            dfs(map, current_path, all_paths, neighbor, destination.clone());
        }
    }

    current_path.pop();
}

fn dfs_part_two(grid: &[&[u8]], seen: &mut Vec<Vec<bool>>, (r,c): (usize, usize), dist: usize, max_dist: &mut usize) {
    if r == grid.len() - 1 {
      *max_dist = (*max_dist).max(dist);
    }
  
    let neighbours = [(-1,0),(1,0),(0,-1),(0,1)].as_slice();
    for &(dr, dc) in neighbours {
      let rr = (r as isize + dr) as usize;
      let cc = (c as isize + dc) as usize;
      let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {continue};
      if tile == b'#' || seen[rr][cc] {
        continue;
      }
      seen[rr][cc] = true;
      dfs_part_two(grid, seen, (rr, cc), dist + 1, max_dist);
      seen[rr][cc] = false;
    }
}

pub fn long_walk_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut map = Vec::new();
    for (_, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (_, c) in line.chars().enumerate() {
            row.push(c);
        }
        map.push(row);
    }

    let start_idx = map[0].iter().position(|c| *c == '.').unwrap();
    let end_idx = map[map.len() - 1].iter().position(|c| *c == '.').unwrap();

    let start = Node {
        row: 0,
        col: start_idx,
        slope: None,
    };
    let end = Node {
        row: map.len() - 1,
        col: end_idx,
        slope: None,
    };

    let mut all_paths = Vec::new();
    dfs(&map, &mut Vec::new(), &mut all_paths, start, end);

    let mut max_steps = 0;
    for path in all_paths {
        if path.len() > max_steps {
            max_steps = path.len();
        }
    }

    println!("A long walk part one: {}", max_steps - 1);
}

pub fn long_walk_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let grid = lines.iter().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut max_dist = 0;
    dfs_part_two(&grid, &mut seen, (0, 1), 0, &mut max_dist);

    println!("A long walk part two: {}", max_dist);
}