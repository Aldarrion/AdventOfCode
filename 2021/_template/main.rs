use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut first_result = 0;
    let mut second_result = 0;

    let line_iter = io::BufReader::new(file).lines();
    for line in line_iter {
        if let Ok(l) = line {
        }
    }

    println!("First\n{}\nSecond\n{}\n", first_result, second_result);
}