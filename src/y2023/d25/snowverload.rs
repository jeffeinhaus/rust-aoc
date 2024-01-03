use crate::utils;
use std::path::Path;
use std::collections::{HashSet, HashMap};

pub fn snowverload_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        let line = line.replace(':', "");
        let mut components = line.split_whitespace();
        if let Some(connection) = components.next() {
            for component in components {
                let conn_string = connection.to_string();
                let comp_string = component.to_string();
                graph.entry(conn_string.clone()).or_insert_with(HashSet::new).insert(comp_string.clone());
                graph.entry(comp_string.clone()).or_insert_with(HashSet::new).insert(conn_string.clone());
            }
        }
    }

    let mut connections_set: HashSet<String> = graph.keys().cloned().collect();

    while connections_set.iter().map(|node| graph[node].difference(&connections_set).count()).sum::<usize>() != 3 {
        let max_connections = connections_set.iter().max_by_key(|node| graph[*node].difference(&connections_set).count()).unwrap().clone();
        connections_set.remove(&max_connections);
    }

    println!("Snowverload part 1: {}", connections_set.len() * (graph.len() - connections_set.len()));

}