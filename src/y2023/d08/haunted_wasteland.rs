use crate::utils;
use std::path::Path;
use std::collections::HashMap;
use num_integer::lcm;

pub fn haunted_wasteland_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut nodes = HashMap::new();
    let mut steps = 0;
    let directions = lines[0].chars().collect::<Vec<char>>();
    let graph_lines = lines[2..].to_vec();
    for line in &graph_lines {
        let line_split = line.split(" = ").collect::<Vec<&str>>();
        let node = line_split[0].to_string();
        let paths = line_split[1].replace("(", "").replace(")", "");
        let next_nodes = paths.split(", ").collect::<Vec<&str>>();  
        nodes.insert(node, (next_nodes[0].to_string(), next_nodes[1].to_string()));
    }
    
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let direction = directions[steps % directions.len()];
        if direction == 'R' {
            current_node = &nodes.get(current_node).unwrap().1;
        } else {
            current_node = &nodes.get(current_node).unwrap().0;
        }
        steps += 1;
    }

    println!("Haunted wasteland part one: {}", steps);
}

pub fn haunted_wasteland_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut nodes = HashMap::new();
    let directions = lines[0].chars().collect::<Vec<char>>();
    let graph_lines = lines[2..].to_vec();
    for line in &graph_lines {
        let line_split = line.split(" = ").collect::<Vec<&str>>();
        let node = line_split[0].to_string();
        let paths = line_split[1].replace("(", "").replace(")", "");
        let next_nodes = paths.split(", ").collect::<Vec<&str>>();  
        nodes.insert(node, (next_nodes[0].to_string(), next_nodes[1].to_string()));
    }
    
    let mut current_nodes = Vec::new();
    for (node, _) in &nodes {
        if node.ends_with("A") {
            current_nodes.push(node);
        }
    }

    let mut step_values = Vec::new();

    for node in &current_nodes {
        let mut current_node = &node.to_string();
        let mut steps = 0;
        while !current_node.ends_with("Z") {
            let direction = directions[steps % directions.len()];
            if direction == 'R' {
                current_node = &nodes.get(current_node).unwrap().1;
            } else {
                current_node = &nodes.get(current_node).unwrap().0;
            }
            steps += 1;
        }
        step_values.push(steps);
    }

    println!("Haunted wasteland part two: {}", step_values.iter().fold(1, |acc, x| lcm(acc, *x)));
}

