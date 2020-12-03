use std::fs;
use std::vec::Vec;
use std::io;
use std::io::BufRead;

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut lines = Vec::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            lines.push(l);
        }
    }

    let dim = [lines[0].len(), lines.len()];

    let slopes = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    let mut result: u64 = 1;
    for slope in slopes.iter() {
        let mut pos = [slope[0], slope[1]];

        let mut tree_count = 0;
        while pos[1] < dim[1] {

            if lines[pos[1]].chars().nth(pos[0]).unwrap() == '#' {
                tree_count += 1;
            }

            pos[0] = (pos[0] + slope[0]) % dim[0];
            pos[1] += slope[1];
        }

        result *= tree_count;
        println!("Slope: {}, {}: {}", slope[0], slope[1], tree_count);
    }

    println!("Result:\n{}", result);
}