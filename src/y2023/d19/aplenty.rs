use regex::Regex;

use crate::utils;
use std::path::Path;
use std::collections::HashMap;

struct Workflow {
    label: String,
    ops: Vec<String>,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Copy, Debug)]
struct CombinationRange {
    min: usize,
    max: usize,
}

#[derive(Clone, Debug)]
enum Rule {
    ACCEPTED,
    REJECTED,
    GOTO(String),
    GT(String, usize, String),
    LT(String, usize, String),
}

fn parse_workflow(workflow: &str) -> Workflow {
    let replaced_workflow = workflow.replace("{", " ").replace("}", "");
    let split_worklow = replaced_workflow.split(" ").collect::<Vec<&str>>();
    Workflow {
        label: split_worklow[0].to_string(),
        ops: split_worklow[1].split(",").map(|x| x.to_string()).collect::<Vec<String>>(),
    }
}

fn parse_part(part: &str) -> Part {
    let replaced_part = part.replace("{", "").replace("}", "");
    let split_part = replaced_part.split(",").collect::<Vec<&str>>();
    Part {
        x: split_part[0].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
        m: split_part[1].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
        a: split_part[2].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
        s: split_part[3].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap(),
    }
}

fn execute_instruction(instruction: &Workflow, part: &Part) -> String {
    for op in &instruction.ops[0..instruction.ops.len()-1] {
        let split_op = op.split(":").collect::<Vec<&str>>();
        let compare_value = match op.chars().nth(0).unwrap() {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Unknown op"),
        };
        let compare_op = match op.chars().nth(1).unwrap() {
            '<' => "<",
            '>' => ">",
            _ => panic!("Unknown op"),
        };
        let mut compare_target = String::new();
        for i in 2..split_op[0].len() {
            compare_target.push_str(split_op[0].chars().nth(i).unwrap().to_string().as_str());
        }
        let compare_target = compare_target.parse::<usize>().unwrap();
        let compare_result = match compare_op {
            "<" => compare_value < compare_target,
            ">" => compare_value > compare_target,
            _ => panic!("Unknown op"),
        };
        if compare_result {
            return split_op[1].to_string();
        }
    }
    instruction.ops[instruction.ops.len()-1].clone()
}

pub fn aplenty_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    for line in lines {
        if line.starts_with("{") {
            let part = parse_part(&line);
            parts.push(part);
        } else if line != "" {
            let workflow = parse_workflow(&line);
            workflows.insert(workflow.label.clone(), workflow);
        }
    }
    let mut result = 0;
    for part in parts {
        let mut current_label = String::from("in");
        while current_label != "A" && current_label != "R" {
            let current_workflow = workflows.get(&current_label).unwrap();
            let next_workflow_label = execute_instruction(current_workflow, &part);
            current_label = String::from(next_workflow_label);
        }
        if current_label == "A" {
            result += part.x + part.m + part.a + part.s;
        }
    }
    println!("Aplenty part one: {}", result);
}

pub fn aplenty_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    for line in lines {
        if line != "" && !line.starts_with("{") {
            let line_clone = line.clone();
            let (key, rules) = parse_workflow_part2(line_clone);
            workflows.insert(key, rules);
        }
    }
    let mut stack: Vec<((CombinationRange, CombinationRange, CombinationRange, CombinationRange), &str, usize)> = 
        vec!{((
            CombinationRange{min: 1, max: 4000}, 
            CombinationRange{min: 1, max: 4000}, 
            CombinationRange{min: 1, max: 4000}, 
            CombinationRange{min: 1, max: 4000}), 
            "in", 
            0
        )};
    let mut accepted: Vec<(CombinationRange, CombinationRange, CombinationRange, CombinationRange)> = Vec::new();
    while let Some(range) = stack.pop() {
        let ((x, m, a, s), workflow_key, rule_key) = range;
        if workflow_key == "A" {
            accepted.push((x, m, a, s));
            continue;
        } else if workflow_key == "R" {
            continue;
        }

        if x.min > x.max || m.min > m.max || a.min > a.max || s.min > s.max {
            continue;
        }

        let rules = workflows.get(workflow_key).unwrap();
        let rule = rules.get(rule_key).unwrap();
        match rule {
            Rule::ACCEPTED => {
                accepted.push((x, m, a, s));
                continue;
            },
            Rule::REJECTED => {
                continue;
            },
            Rule::GOTO(key) => {
                stack.push(((x, m, a, s), key, 0));
                continue;
            },
            Rule::GT(label, value, key) => {
                match label.as_str() {
                    "x" => {
                        stack.push(((CombinationRange { min: value + 1, max: x.max }, m, a, s), key, 0));
                        stack.push(((CombinationRange { min: x.min, max: *value }, m, a, s), workflow_key, rule_key + 1));
                    },
                    "m" => {
                        stack.push(((x, CombinationRange { min: value + 1, max: m.max }, a, s), key, 0));
                        stack.push(((x, CombinationRange { min: m.min, max: *value }, a, s), workflow_key, rule_key + 1));
                    },
                    "a" => {
                        stack.push(((x, m, CombinationRange { min: value + 1, max: a.max }, s), key, 0));
                        stack.push(((x, m, CombinationRange { min: a.min, max: *value }, s), workflow_key, rule_key + 1));
                    },
                    "s" => {
                        stack.push(((x, m, a, CombinationRange { min: value + 1, max: s.max }), key, 0));
                        stack.push(((x, m, a, CombinationRange { min: s.min, max: *value }), workflow_key, rule_key + 1));
                    },
                    _ => panic!("Unknown label {}", label),
                }
            },
            Rule::LT(label, value, key) => {
                match label.as_str() {
                    "x" => {
                        stack.push(((CombinationRange { min: x.min, max: *value - 1 }, m, a, s), key, 0));
                        stack.push(((CombinationRange { min: *value, max: x.max }, m, a, s), workflow_key, rule_key + 1));
                    },
                    "m" => {
                        stack.push(((x, CombinationRange { min: m.min, max: *value - 1 }, a, s), key, 0));
                        stack.push(((x, CombinationRange { min: *value, max: m.max }, a, s), workflow_key, rule_key + 1));
                    },
                    "a" => {
                        stack.push(((x, m, CombinationRange { min: a.min, max: *value - 1 }, s), key, 0));
                        stack.push(((x, m, CombinationRange { min: *value, max: a.max }, s), workflow_key, rule_key + 1));
                    },
                    "s" => {
                        stack.push(((x, m, a, CombinationRange { min: s.min, max: *value - 1 }), key, 0));
                        stack.push(((x, m, a, CombinationRange { min: *value, max: s.max }), workflow_key, rule_key + 1));
                    },
                    _ => panic!("Unknown label {}", label),
                }
            },
        }
    }

    let result = accepted.iter()
        .map(|(x, m, a, s)| (x.max - x.min + 1) * (m.max - m.min + 1) * (a.max - a.min + 1) * (s.max - s.min + 1))
        .sum::<usize>();
    println!("Aplenty part two: {}", result);
}

fn parse_workflow_part2(workflow: String) -> (String, Vec<Rule>) {
    let re = Regex::new(r"^(.+)\{(.+)\}$").unwrap();
    let captures = re.captures(&workflow).unwrap();
    let key = captures.get(1).unwrap().as_str();
    let rules_string = captures.get(2).unwrap().as_str();

    let rules = rules_string.split(",").map(|x| {
        if x == "A" {
            return Rule::ACCEPTED;
        }
        if x == "R" {
            return Rule::REJECTED;
        }
        if !x.contains(":") {
            return Rule::GOTO(x.to_string());
        }

        let (rule, dest) = x.split_once(":").unwrap();

        return if rule.contains(">") {
            let (label, value) = rule.split_once(">").unwrap();
            Rule::GT(label.to_string(), value.parse::<usize>().unwrap(), dest.to_string())
        } else if rule.contains("<") {
            let (label, value) = rule.split_once("<").unwrap();
            Rule::LT(label.to_string(), value.parse::<usize>().unwrap(), dest.to_string())
        } else {
            panic!("Unknown rule {}", rule);
        }
    }).collect();

    (key.to_string(), rules)
}
