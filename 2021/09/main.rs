/*
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678

Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?

Your puzzle answer was 580.
--- Part Two ---

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678

The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678

The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?

Your puzzle answer was 856716.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut heightmap = Vec::<Vec<(i8, bool)>>::new();

    let line_iter = io::BufReader::new(file).lines();
    for line in line_iter {
        if let Ok(l) = line {
            heightmap.push(l.chars().map(|c| (c.to_digit(10).unwrap() as i8, false)).collect());
        }
    }

    let width = heightmap[0].len() as i32;
    let height = heightmap.len() as i32;

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut total_risk: i32 = 0;
    let mut biggest_area = [0i32, 0i32, 0i32];
    for x in 0..width {
        for y in 0..height {
            let (val, visited) = heightmap[y as usize][x as usize];
            if visited || val == 9 {
                continue;
            }

            let mut lowest = true;
            for (n_x, n_y) in neighbors.iter() {
                let check_x: i32 = x + n_x;
                let check_y: i32 = y + n_y;
                if check_x >= 0 && check_y >= 0 && check_x < width && check_y < height {
                    let (n_val, _) = heightmap[check_y as usize][check_x as usize];
                    if n_val <= val {
                        lowest = false;
                        break;
                    }
                }
            }

            if lowest {
                total_risk += 1 + val as i32;

                let mut size = 0;
                let mut stack = Vec::<(i32, i32)>::new();
                stack.push((x, y));
                while !stack.is_empty() {
                    let (s_x, s_y) = stack.pop().unwrap();

                    if heightmap[s_y as usize][s_x as usize].1 {
                        continue;
                    }

                    size += 1;

                    heightmap[s_y as usize][s_x as usize].1 = true;
                    let (s_val, _) = heightmap[s_y as usize][s_x as usize];

                    for (n_x, n_y) in neighbors.iter() {
                        let check_x: i32 = s_x + n_x;
                        let check_y: i32 = s_y + n_y;
                        if check_x >= 0 && check_y >= 0 && check_x < width && check_y < height {
                            let (n_val, n_visited) = heightmap[check_y as usize][check_x as usize];
                            if !n_visited && n_val != 9 && n_val >= s_val {
                                stack.push((check_x, check_y));
                            }
                        }
                    }
                }

                if size > biggest_area[0] {
                    biggest_area[2] = biggest_area[1];
                    biggest_area[1] = biggest_area[0];
                    biggest_area[0] = size;
                } else if size > biggest_area[1] {
                    biggest_area[2] = biggest_area[1];
                    biggest_area[1] = size;
                } else if size > biggest_area[2] {
                    biggest_area[2] = size;
                }
            }
        }
    }

    println!("Total risk\n{}\nArea mul\n{}\n", total_risk, biggest_area[0] * biggest_area[1] * biggest_area[2]);
}