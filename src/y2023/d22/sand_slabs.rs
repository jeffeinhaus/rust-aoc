use std::cmp::{min, max};
use crate::utils;
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    x: u32,
    y: u32,
    z: u32,
    dx: u32,
    dy: u32,
    dz: u32,
}

fn parse_brick(brick: &str) -> Brick {
    let brick = brick.replace("~", ",");
    let mut brick = brick.split(",");
    let mut brick = Brick {
        x: brick.next().unwrap().parse().unwrap(),
        y: brick.next().unwrap().parse().unwrap(),
        z: brick.next().unwrap().parse().unwrap(),
        dx: brick.next().unwrap().parse().unwrap(),
        dy: brick.next().unwrap().parse().unwrap(),
        dz: brick.next().unwrap().parse().unwrap(),
    };
    brick.x = min(brick.x, brick.dx);
    brick.y = min(brick.y, brick.dy);
    brick.z = min(brick.z, brick.dz);
    brick.dx = max(brick.x, brick.dx);
    brick.dy = max(brick.y, brick.dy);
    brick.dz = max(brick.z, brick.dz);
    brick
}

fn is_solid(support_set: HashSet<(u32, u32, u32)>, x: u32, y: u32, z: u32) -> bool {
    if z == 0 {
        return true;
    }
    support_set.contains(&(x, y, z))
} 

fn drop_bricks(bricks: &Vec<Brick>) -> (bool, Vec<Brick>) {
    let mut fell = false;
    let mut support_set: HashSet<(u32, u32, u32)> = HashSet::new();
    for brick in bricks {
        for x in brick.x..brick.dx+1 {
            for y in brick.y..brick.dy+1 {
                support_set.insert((x, y, brick.dz));
            }
        }
    }
    let mut new_bricks = Vec::new();
    for brick in bricks {
        let mut supported = false;
        for x in brick.x..brick.dx+1 {
            for y in brick.y..brick.dy+1 {
                if is_solid(support_set.clone(), x, y, brick.z - 1) {
                    supported = true;
                    break;
                } 
            }
            if supported {
                break;
            }
        }
        if !supported {
            fell = true;
            new_bricks.push(Brick {
                x: brick.x,
                y: brick.y,
                z: brick.z - 1,
                dx: brick.dx,
                dy: brick.dy,
                dz: brick.dz - 1,
            }); 
        } else {
            new_bricks.push(brick.clone());
        }
    }
    (fell, new_bricks)
}

pub fn sand_slabs_part_one<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut bricks: Vec<Brick> = Vec::new();
    for line in lines {
        bricks.push(parse_brick(&line));
    }

    let mut fell = true;
    while fell {
        let (f, b) = drop_bricks(&bricks);
        fell = f;
        bricks = b;
    }
    let settled_bricks = bricks.clone();
    let mut bricks_to_disintegrate = 0;

    for i in 0..settled_bricks.len() {
        let mut settled_bricks_clone = settled_bricks.clone();
        settled_bricks_clone.remove(i);
        let (f, _) = drop_bricks(&settled_bricks_clone);
        if !f {
            bricks_to_disintegrate += 1;
        }
    }
    println!("Sand slabs part one: {}", bricks_to_disintegrate);
}

pub fn sand_slabs_part_two<P: AsRef<Path>>(path: P) {
    let lines = utils::read_file_line_by_line(path).unwrap();
    let mut bricks: Vec<Brick> = Vec::new();
    for line in lines {
        bricks.push(parse_brick(&line));
    }

    let mut fell = true;
    while fell {
        let (f, b) = drop_bricks(&bricks);
        fell = f;
        bricks = b;
    }
    let settled_bricks = bricks.clone();
    let mut falling_bricks = 0;

    for i in 0..settled_bricks.len() {
        let mut bricks_to_fall = settled_bricks.clone();
        bricks_to_fall.remove(i);
        let settled_bricks_clone = bricks_to_fall.clone();
        let mut fell = true;
        while fell {
            let (f, b) = drop_bricks(&bricks_to_fall);
            fell = f;
            bricks_to_fall = b;
        }
        let mut fallen_bricks = 0;
        for (brick1, brick2) in bricks_to_fall.iter().zip(settled_bricks_clone.iter()) {
            if brick1 != brick2 {
                fallen_bricks += 1;
            }
        }
        falling_bricks += fallen_bricks;
    }
    println!("Sand slabs part two: {}", falling_bricks);
}