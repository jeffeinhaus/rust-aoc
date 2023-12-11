use crate::utils;
use std::path::Path;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl PartialEq for Galaxy {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn cosmic_expansion<P: AsRef<Path>>(path: P, expansion: usize) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let map = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let empty_rows = find_empty_rows(&map);
    let empty_columns = find_empty_columns(&map);
    
    let galaxies = get_galaxies(&map);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        let distances = dijkstra_priority_queue(&map, &galaxies[i], &empty_rows, &empty_columns, expansion);
        for j in i+1..galaxies.len() {
            sum += distances[galaxies[j].y][galaxies[j].x];
        }
    }
    let part = if expansion == 1 { "one" } else { "two" };
    println!("Cosmic expansion part {}: {}", part, sum);
}

fn find_empty_rows(map: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|&x| x == '.') {
            empty_rows.push(i);
        }
    }
    empty_rows
}

fn find_empty_columns(map: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_columns = Vec::new();
    for i in 0..map[0].len() {
        let mut empty = true;
        for j in 0..map.len() {
            if map[j][i] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_columns.push(i);
        }
    }
    empty_columns
}

fn get_galaxies(map: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column == '#' {
                galaxies.push(Galaxy { x: j, y: i });
            }
        }
    }
    galaxies
}

fn dijkstra_priority_queue(map: &Vec<Vec<char>>, start: &Galaxy, empty_rows: &Vec<usize>, empty_columns: &Vec<usize>, expansion: usize) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue: Vec<Galaxy> = Vec::new();
    distances[start.y][start.x] = 0;
    queue.push(Galaxy { x: start.x, y: start.y });
    while !queue.is_empty() {
        let current = queue.remove(0);
        let neighbours = get_neighbours(&map, &current);
        for neighbour in neighbours {
            let distance = distances[current.y][current.x] + 1;
            if distance < distances[neighbour.y][neighbour.x] {
                if empty_rows.contains(&neighbour.y) || empty_columns.contains(&neighbour.x) {
                    distances[neighbour.y][neighbour.x] = distance + expansion;
                } else {
                    distances[neighbour.y][neighbour.x] = distance;
                }
                queue.push(neighbour);
            }
        }
    }
    distances
}

fn get_neighbours(map: &Vec<Vec<char>>, current: &Galaxy) -> Vec<Galaxy> {
    let mut neighbours = Vec::new();
    if current.y > 0 {
        neighbours.push(Galaxy { x: current.x, y: current.y - 1 });
    }
    if current.y < map.len() - 1 {
        neighbours.push(Galaxy { x: current.x, y: current.y + 1 });
    }
    if current.x > 0 {
        neighbours.push(Galaxy { x: current.x - 1, y: current.y });
    }
    if current.x < map[0].len() - 1 {
        neighbours.push(Galaxy { x: current.x + 1, y: current.y });
    }
    neighbours
}