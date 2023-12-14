use crate::utils;
use std::path::Path;

pub fn parabolic_reflector_dish_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut sum = 0;
    let mut grid = Vec::new();
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }
    let mut final_grid = grid.clone();
    for (row, row_char) in grid.iter().enumerate() {
        for (col, _) in row_char.iter().enumerate() {
            if final_grid[row][col] == 'O' {
                let ceiling = get_north_ceiling(&final_grid, row, col);
                if ceiling != row as i32 - 1  {
                    final_grid[(ceiling+1) as usize][col] = 'O';
                    final_grid[row][col] = '.';
                }
            }
        }
    }
    for (row, row_char) in final_grid.iter().enumerate() {
        for col in row_char {
            if col == &'O' {
                sum += final_grid.len() - row;
            }
            
        }
    }
    println!("Parabolic Reflector Dish part one: {}", sum);
}

pub fn parabolic_reflector_dish_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut grid = Vec::new();
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }
    let mut final_grid = grid.clone();
    let mut results = Vec::new();
    for _ in 0..1_000_000_000 {
        if let Some((loop_start, loop_len)) = find_start_of_loop(&results) {
            if loop_len > 1 {
                let loop_start = loop_start;
                let result_idx = (1_000_000_000 - loop_start) % loop_len;
                println!("Parabolic Reflector Dish part two: {}", results[loop_start + result_idx - 1]);
                break;
            }
        }
        let mut sum = 0;
        for (row, row_char) in grid.iter().enumerate() {
            for (col, _) in row_char.iter().enumerate() {
                if final_grid[row][col] == 'O' {
                    let ceiling = get_north_ceiling(&final_grid, row, col);
                    if ceiling != row as i32 - 1  {
                        final_grid[(ceiling+1) as usize][col] = 'O';
                        final_grid[row][col] = '.';
                    }
                }
            }
        }
        for (row, row_char) in grid.iter().enumerate() {
            for (col, _) in row_char.iter().enumerate() {
                if final_grid[row][col] == 'O' {
                    let ceiling = get_west_ceiling(&final_grid, row, col);
                    if ceiling != col as i32 - 1  {
                        final_grid[row][(ceiling+1) as usize] = 'O';
                        final_grid[row][col] = '.';
                    }
                }
            }
        }
        for (row, row_char) in grid.iter().enumerate().rev() {
            for (col, _) in row_char.iter().enumerate() {
                if final_grid[row][col] == 'O' {
                    let ceiling = get_south_ceiling(&final_grid, row, col);
                    if ceiling != row as i32 + 1  {
                        final_grid[(ceiling-1) as usize][col] = 'O';
                        final_grid[row][col] = '.';
                    }
                }
            }
        }
        for (row, row_char) in grid.iter().enumerate() {
            for (col, _) in row_char.iter().enumerate().rev() {
                if final_grid[row][col] == 'O' {
                    let ceiling = get_east_ceiling(&final_grid, row, col);
                    if ceiling != col as i32 + 1  {
                        final_grid[row][(ceiling-1) as usize] = 'O';
                        final_grid[row][col] = '.';
                    }
                }
            }
        }
        for (row, row_char) in final_grid.iter().enumerate() {
            for col in row_char {
                if col == &'O' {
                    sum += final_grid.len() - row;
                } 
            }
        }
        results.push(sum);
    }
}

fn get_north_ceiling(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if row == 0 {
        return -1
    }
    for i in (0..row).rev() {
        if grid[i][col] == '#' || grid[i][col] == 'O' {
            return i as i32
        }
    }
    -1
}

fn get_south_ceiling(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if row == grid.len() - 1 {
        return grid.len() as i32
    }
    for i in row+1..grid.len() {
        if grid[i][col] == '#' || grid[i][col] == 'O' {
            return i as i32
        }
    }
    grid.len() as i32
}

fn get_east_ceiling(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if col == grid[row].len() - 1 {
        return grid[row].len() as i32
    }
    for i in col+1..grid[row].len() {
        if grid[row][i] == '#' || grid[row][i] == 'O' {
            return i as i32
        }
    }
    grid[row].len() as i32
}

fn get_west_ceiling(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    if col == 0 {
        return -1
    }
    for i in (0..col).rev() {
        if grid[row][i] == '#' || grid[row][i] == 'O' {
            return i as i32
        }
    }
    -1
}

fn find_start_of_loop(vec: &Vec<usize>) -> Option<(usize, usize)> {
    for start in 0..vec.len() {
        for end in start + 1..vec.len() {
            let loop_candidate = &vec[start..end];

            let mut index = end;
            while index + end - start <= vec.len() {
                if loop_candidate == &vec[index..index + end - start] {
                    index += end - start;
                } else {
                    break;
                }
            }

            if index == vec.len() {
                return Some((start, end - start));
            }
        }
    }

    None
}