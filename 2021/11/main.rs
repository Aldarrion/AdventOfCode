
use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut neighbors: [(i32, i32); 8] = Default::default();
    let mut i = 0;
    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                neighbors[i] = (x, y);
                i += 1;
            }
        }
    }

    let mut octopuses = [(0, false); 100];

    let line_iter = io::BufReader::new(file).lines();
    let mut octo_i = 0;
    for line in line_iter {
        if let Ok(l) = line {
            let line_iter = l.chars().map(|c| (c.to_digit(10).unwrap() as i8, false));
            for o in line_iter {
                octopuses[octo_i] = o;
                octo_i += 1;
            }
        }
    }

    let mut stack = Vec::<i8>::new();
    let mut flash_count = 0;
    let mut first_all_flash = -1;
    let mut step = 1;

    while first_all_flash == -1 || step <= 100 {
        for o_i in 0..100 {
            octopuses[o_i].0 += 1;
            if octopuses[o_i].0 > 9 {
                stack.push(o_i as i8);
            }
        }

        while !stack.is_empty() {
            let o_i = stack.pop().unwrap() as usize;
            if octopuses[o_i].1 {
                continue;
            }
            octopuses[o_i].1 = true;

            if step <= 100 {
                flash_count += 1;
            }

            let o_x = o_i % 10;
            let o_y = o_i / 10;
            for (n_x, n_y) in neighbors.iter() {
                let x = o_x as i32 + n_x;
                let y = o_y as i32 + n_y;
                if x >= 0 && y >= 0 && x < 10 && y < 10 {
                    let i = (y * 10 + x) as usize;
                    octopuses[i].0 += 1;
                    if octopuses[i].0 > 9 {
                        stack.push(i as i8);
                    }
                }
            }
        }

        let mut all_flashed = true;
        for (energy, flashed) in octopuses.iter_mut() {
            if *flashed {
                *energy = 0;
                *flashed = false;
            } else {
                all_flashed = false;
            }
        }

        if first_all_flash == -1 && all_flashed {
            first_all_flash = step;
        }
        step += 1;
    }

    println!("Total flashes\n{}\nFirst all flash\n{}\n", flash_count, first_all_flash);
}