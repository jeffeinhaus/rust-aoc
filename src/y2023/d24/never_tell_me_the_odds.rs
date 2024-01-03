use z3::ast::{Int, Real, Ast};

use crate::utils;
use std::path::Path;

#[derive(Debug)]
struct Line {
    m: f64,
    b: f64,
    x: u64,
    y: u64,
    vx: i32,
    vy: i32,
}

impl Line {
    fn new(x: u64, y: u64, vx: i32, vy: i32) -> Self {
        let m = vy as f64 / vx as f64;
        let b = y as f64 - m * x as f64;
        Self { m, b, x, y, vx, vy }
    }

    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.m == other.m {
            return None;
        }
        let x = (other.b - self.b) / (self.m - other.m);
        let y = self.m * x + self.b;
        if self.vx > 0 && x < self.x as f64 
            || self.vx < 0 && x > self.x as f64 
            || self.vy > 0 && y < self.y as f64
            || self.vy < 0 && y > self.y as f64
            || other.vx > 0 && x < other.x as f64
            || other.vx < 0 && x > other.x as f64
            || other.vy > 0 && y < other.y as f64
            || other.vy < 0 && y > other.y as f64 {
            return None;
        }
        Some((x, y))
    }
}

fn parse_line(line: &str) -> Line {
    let parts = line.split(" @ ").collect::<Vec<&str>>();
    let mut coords = parts[0].split(",");
    let mut velocity = parts[1].split(",");
    let x = coords.next().unwrap().trim().parse::<u64>().unwrap();
    let y = coords.next().unwrap().trim().parse::<u64>().unwrap();
    let vx = velocity.next().unwrap().trim().parse::<i32>().unwrap();
    let vy = velocity.next().unwrap().trim().parse::<i32>().unwrap();
    Line::new(x, y, vx, vy)
}

fn parse_throw(throw: &str) -> ((f64, f64, f64), (f64, f64, f64)) {
    let parts = throw.split(" @ ").collect::<Vec<&str>>();
    let mut coords = parts[0].split(",");
    let mut velocity = parts[1].split(",");
    let x = coords.next().unwrap().trim().parse::<f64>().unwrap();
    let y = coords.next().unwrap().trim().parse::<f64>().unwrap();
    let z = coords.next().unwrap().trim().parse::<f64>().unwrap();
    let vx = velocity.next().unwrap().trim().parse::<f64>().unwrap();
    let vy = velocity.next().unwrap().trim().parse::<f64>().unwrap();
    let vz = velocity.next().unwrap().trim().parse::<f64>().unwrap();
    ((x, y, z), (vx, vy, vz))
}

pub fn never_tell_me_the_odds_part_one<P: AsRef<Path>>(path: P, min: u64, max: u64) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let lines: Vec<Line> = lines.iter().map(|line| parse_line(line)).collect();
    let mut intersections = Vec::new();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some((x, y)) = lines[i].intersect(&lines[j]) {
                if x as u64 >= min && x as u64 <= max && y as u64 >= min && y as u64 <= max {
                    intersections.push((x, y));
                }
            }
        }
    }
    println!("Part one never tell me the odds: {}", intersections.len());
}

pub fn never_tell_me_the_odds_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let context = z3::Context::new(&z3::Config::new());
    let solver = z3::Solver::new(&context);
    let [fx, fy, fz, fdx, fdy, fdz] = ["fx", "fy", "fz", "fdx", "fdy", "fdz"]
        .map(|x| Real::new_const(&context, x));

    let zero = Int::from_i64(&context, 0).to_real();
    let throws: Vec<((f64, f64, f64), (f64, f64, f64))> = lines.iter().map(|throw| parse_throw(throw)).collect();
    for (i, &((x,y,z), (vx, vy, vz))) in throws.iter().enumerate() {
        let [x,y,z,vx,vy,vz] = [x,y,z,vx,vy,vz].map(|x| Int::from_i64(&context, x as _).to_real());
        let t = Real::new_const(&context, format!("t{i}"));
        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &vx * &t)._eq(&(&fx + &fdx * &t))));
        solver.assert(&((&y + &vy * &t)._eq(&(&fy + &fdy * &t))));
        solver.assert(&((&z + &vz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let result = solver.get_model().unwrap().eval(&(&fx + &fy + &fz), true).unwrap();
    println!("Part two never tell me the odds: {}", result.to_string().strip_suffix(".0").unwrap().parse::<u64>().unwrap());
}