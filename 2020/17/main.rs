
/*
--- Day 17: Conway Cubes ---

As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.

The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.

In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.

The energy source then proceeds to boot up by executing six cycles.

Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

During a cycle, all cubes simultaneously change their state according to the following rules:

    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.

The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.

Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?

Your puzzle answer was 218.
--- Part Two ---

For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.

The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.

Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.

The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.

For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)

After the full six-cycle boot process completes, 848 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

Your puzzle answer was 1908.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Position {
    fn add(&self, p: &Position) -> Position {
        Position {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
            w: self.w + p.w,
        }
    }
}

fn evaluate(init_charged: &HashSet<Position>, neighbors: &[Position]) -> i32 {
    let mut charged = [init_charged.clone(), init_charged.clone()];
    for iter_i in 0..6 {
        let charged_now_i = iter_i % 2;
        let charged_next_i = (iter_i + 1) % 2;
        let mut queue = HashSet::<Position>::new();
        for p in &charged[charged_now_i] {
            for n in neighbors {
                queue.insert(p.add(&n));
            }
        }
        charged[charged_next_i].clear();
        for p in queue {
            let this_charged = charged[charged_now_i].contains(&p);
            let mut charged_around = 0;
            for n in neighbors {
                let neighbor_pos = p.add(n);
                if charged[charged_now_i].contains(&neighbor_pos) {
                    charged_around += 1;
                }
            }
            if this_charged && charged_around == 2 || charged_around == 3 {
                charged[charged_next_i].insert(p);
            }
        }
    }
    charged[0].len() as i32
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let lines = io::BufReader::new(file).lines();

    let mut charged = HashSet::<Position>::new();
    for (y, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    charged.insert(Position{ x: x as i32, y: y as i32, z: 0, w: 0 });
                }
            }
        }
    }

    let mut neighbors_3d = vec!();
    let mut neighbors_4d = vec!();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if x != 0 || y != 0 || z != 0 {
                    neighbors_3d.push(Position{ x, y, z, w: 0 });
                }
                for w in -1..2 {
                    if x != 0 || y != 0 || z != 0 || w != 0 {
                        neighbors_4d.push(Position{ x, y, z, w });
                    }
                }
            }
        }
    }


    let res_3d = evaluate(&charged, &neighbors_3d);
    println!("Charged 3D after 6\n{}", res_3d);

    let res_4d = evaluate(&charged, &neighbors_4d);
    println!("Charged 4D after 6\n{}", res_4d);
}
