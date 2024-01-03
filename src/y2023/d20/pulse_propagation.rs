use std::collections::{HashMap, VecDeque};
use crate::utils;
use std::path::Path;
use num_integer::lcm;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug, PartialEq)]
struct Message {
    source: String,
    pulse: Pulse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ComponentType {
    Broadcast,
    Conjunction,
    FlipFlop,
    Sink,
}

#[derive(Clone, Debug)]
struct Component {
    kind: ComponentType,
    targets: Vec<String>,
    memory: Option<HashMap<String, Pulse>>,
    enabled: Option<bool>,
}

fn build_components(lines: &Vec<String>) -> HashMap<String, Component> {
    let mut components: HashMap<String, Component> = HashMap::new();
    for line in lines {
        let (source_part, dest_part) = line.split_once(" -> ").unwrap();
        match &source_part[0..1] {
            "%" => {
                let name = String::from(&source_part[1..]);
                let component = Component {
                    kind: ComponentType::FlipFlop,
                    targets: dest_part.split(", ").map(|s| s.to_string()).collect::<Vec<String>>(),
                    memory: None,
                    enabled: Some(false),
                };
                components.insert(name, component);
            }
            "&" => {
                let name = String::from(&source_part[1..]);
                let component = Component {
                    kind: ComponentType::Conjunction,
                    targets: dest_part.split(", ").map(|s| s.to_string()).collect::<Vec<String>>(),
                    memory: Some(HashMap::new()),
                    enabled: None,
                };
                components.insert(name, component);
            }
            _ => {
                let name = String::from(source_part);
                let component = Component {
                    kind: ComponentType::Broadcast,
                    targets: dest_part.split(", ").map(|s| s.to_string()).collect::<Vec<String>>(),
                    memory: None,
                    enabled: None,
                };
                components.insert(name, component);
            }
        }
    }
    let conjunctions: Vec<String> = components.iter()
        .filter(|(_, v)| v.kind == ComponentType::Conjunction)
        .map(|(k, _)| k)
        .cloned()
        .collect();
    let components_clone = components.clone();
    for conjunction in conjunctions.iter() {
        let sources: Vec<String> = components_clone
            .iter()
            .filter(|(_, v)| v.targets.contains(conjunction))
            .map(|(k, _)| k)
            .cloned()
            .collect();
        for source in sources.iter() {
            components.entry(conjunction.clone()).and_modify(|e| {
                e.memory.as_mut().unwrap().insert(source.clone(), Pulse::Low);
            });
        }
    }
    for component in components_clone.values() {
        if component.targets.iter().any(|t| !components_clone.contains_key(t)) {
            for sink in component.targets.iter().filter(|&t| !components_clone.contains_key(t)) {
                components.insert(
                    sink.clone(),
                    Component {
                        kind: ComponentType::Sink,
                        targets: vec![],
                        memory: None,
                        enabled: None,
                    },
                );
            }
        }
    }
    components
}

fn push_button_part_one(components: &mut HashMap<String, Component>) -> u32 {
    let mut low_count: u32 = 0;
    let mut high_count: u32 = 0;
    let mut queues: HashMap<String, VecDeque<Message>> = HashMap::new();
    for label in components.keys() {
        queues.insert(label.clone(), VecDeque::new());
    }
    for _ in 0..1000 {
        let current: String = String::from("broadcaster");
        queues.entry(current).and_modify(|e| {
            e.push_back(Message {
                source: String::from("button"),
                pulse: Pulse::Low,
            })
        });
        low_count += 1;
        while queues.values().any(|v| !v.is_empty()) {
            for label in queues
                .clone()
                .iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(k, _)| k)
            {
                let component = components.get_mut(label).unwrap();
                match component.kind {
                    ComponentType::Broadcast => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            for target in component.targets.iter() {
                                queues.get_mut(target).unwrap().push_back(Message {
                                    source: label.clone(),
                                    pulse: message.pulse,
                                });
                                match message.pulse {
                                    Pulse::Low => low_count += 1,
                                    Pulse::High => high_count += 1,
                                };
                            }
                        }
                    }
                    ComponentType::FlipFlop => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            if message.pulse == Pulse::Low {
                                let enabled = component.enabled.unwrap();
                                for target in component.targets.iter() {
                                    let target_queue = queues.get_mut(target).unwrap();
                                    if enabled {
                                        target_queue.push_back(Message {
                                            source: label.clone(),
                                            pulse: Pulse::Low,
                                        });
                                        low_count += 1;
                                    } else {
                                        target_queue.push_back(Message {
                                            source: label.clone(),
                                            pulse: Pulse::High,
                                        });
                                        high_count += 1;
                                    }
                                }
                                component.enabled = Some(!enabled);
                            }
                        }
                    }
                    ComponentType::Conjunction => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            component.memory.as_mut().unwrap().entry(message.source).and_modify(|p| *p = message.pulse);
                            for target in component.targets.iter() {
                                let target_queue = queues.get_mut(target).unwrap();
                                if component.memory.as_mut().unwrap().values().all(|p| *p == Pulse::High)
                                {
                                    target_queue.push_back(Message {
                                        source: label.clone(),
                                        pulse: Pulse::Low,
                                    });
                                    low_count += 1;
                                } else {
                                    target_queue.push_back(Message {
                                        source: label.clone(),
                                        pulse: Pulse::High,
                                    });
                                    high_count += 1;
                                }
                            }
                        }
                    }
                    ComponentType::Sink => {
                        while !queues.get(label).unwrap().is_empty() {
                            queues.get_mut(label).unwrap().pop_front().unwrap();
                        }
                    }
                };
            }
        }
    }
    low_count * high_count
}

fn push_button_part_two(components: &mut HashMap<String, Component>) -> u64 {
    let mut queues: HashMap<String, VecDeque<Message>> = HashMap::new();
    for label in components.keys() {
        queues.insert(label.clone(), VecDeque::new());
    }
    let mut loop_count: u64 = 0;
    let mut rx_high_counts: HashMap<String, u64> = HashMap::new();
    let rx_parent = components
        .iter()
        .filter(|(_, v)| v.targets.contains(&String::from("rx")))
        .map(|(k, _)| k)
        .next()
        .unwrap()
        .clone();
    let rx_num_inputs = components
        .get(&rx_parent)
        .unwrap()
        .memory
        .as_ref()
        .unwrap()
        .keys()
        .count();
    loop {
        loop_count += 1;
        let current: String = String::from("broadcaster");
        queues.entry(current).and_modify(|e| {
            e.push_back(Message {
                source: String::from("button"),
                pulse: Pulse::Low,
            })
        });
        while queues.values().any(|v| !v.is_empty()) {
            for label in queues
                .clone()
                .iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(k, _)| k)
            {
                let component = components.get_mut(label).unwrap();
                match component.kind {
                    ComponentType::Broadcast => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            for target in component.targets.iter() {
                                queues.get_mut(target).unwrap().push_back(Message {
                                    source: label.clone(),
                                    pulse: message.pulse,
                                });
                            }
                        }
                    }
                    ComponentType::FlipFlop => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            if message.pulse == Pulse::Low {
                                let enabled = component.enabled.unwrap();
                                for target in component.targets.iter() {
                                    let target_queue = queues.get_mut(target).unwrap();
                                    if enabled {
                                        target_queue.push_back(Message {
                                            source: label.clone(),
                                            pulse: Pulse::Low,
                                        });
                                    } else {
                                        target_queue.push_back(Message {
                                            source: label.clone(),
                                            pulse: Pulse::High,
                                        });
                                    }
                                }
                                component.enabled = Some(!enabled);  
                            }
                        }
                    }
                    ComponentType::Conjunction => {
                        while !queues.get(label).unwrap().is_empty() {
                            let message = queues.get_mut(label).unwrap().pop_front().unwrap();
                            let source = message.source.clone();
                            component.memory.as_mut().unwrap().entry(message.source).and_modify(|p| *p = message.pulse);
                            for target in component.targets.iter() {
                                let target_queue = queues.get_mut(target).unwrap();
                                if component.memory.as_mut().unwrap().values().all(|p| *p == Pulse::High) {
                                    target_queue.push_back(Message {
                                        source: label.clone(),
                                        pulse: Pulse::Low,
                                    });
                                } else {
                                    target_queue.push_back(Message {
                                        source: label.clone(),
                                        pulse: Pulse::High,
                                    });
                                }
                            }
                            if label.clone() == rx_parent && message.pulse == Pulse::High && !rx_high_counts.contains_key(&source) {
                                rx_high_counts.insert(source, loop_count);
                            }
                        }
                    }
                    ComponentType::Sink => {
                        while !queues.get(label).unwrap().is_empty() {
                            queues.get_mut(label).unwrap().pop_front().unwrap();
                        }
                    }
                };
            }
        }
        if rx_high_counts.keys().count() == rx_num_inputs {
            break;
        }
    }
    rx_high_counts.values().cloned().reduce(lcm).unwrap()
}

pub fn pulse_propagation_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut components = build_components(&lines);
    let result = push_button_part_one(&mut components);
    println!("Pulse propagation part one: {result}");
}

pub fn pulse_propagation_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut components = build_components(&lines);
    let result = push_button_part_two(&mut components);
    println!("Pulse propagation part two: {result}");
}