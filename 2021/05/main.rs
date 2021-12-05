use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut segments = Vec::<((i32, i32), (i32, i32))>::new();
    let mut width = 0;
    let mut height = 0;

    let line_iter = io::BufReader::new(file).lines();
    for line in line_iter {
        if let Ok(l) = line {
            let mut points: [(i32, i32);2] = Default::default();
            l.split(" -> ").enumerate().for_each(|(i, point)| {
                let mut coords = point.split(',');
                let x = coords.next().unwrap().parse::<i32>().unwrap();
                let y = coords.next().unwrap().parse::<i32>().unwrap();
                points[i] = (x, y);
            });
            let (x1, y1) = points[0];
            let (x2, y2) = points[1];
            if y1 > y2 || (y1 == y2 && x1 > x2) {
                let tmp = points[0];
                points[0] = points[1];
                points[1] = tmp;
            }
            let (x1, y1) = points[0];
            let (x2, y2) = points[1];

            segments.push(((x1, y1), (x2, y2)));
            width = std::cmp::max(width, x2 + 1);
            height = std::cmp::max(height, y2 + 1);
        }
    }

    let mut field = Vec::<i32>::new();
    field.resize((width * height) as usize, 0);

    for ((x1, y1), (x2, y2)) in segments.iter() {
        if x1 == x2 || y1 == y2 {
            for x in *x1..(*x2 + 1) {
                for y in *y1..(*y2 + 1) {
                    let idx = (y * width + x) as usize;
                    field[idx] += 1;
                }
            }
        } else { // Diagonal
            // y2 > y1
            let x_mul = if x1 < x2 {
                1
            } else {
                -1
            };

            let dist = y2 + 1 - y1;
            for n in 0..dist {
                let x = x1 + x_mul * n;
                let y = y1 + n;
                let idx = (y * width + x) as usize;
                field[idx] += 1;
            }
        }
    }

    let mut multi_count = 0;
    for (_i, p) in field.iter().enumerate() {
        //if i % width as usize == 0 {
        //    print!("\n");
        //}
        //if *p == 0 {
        //    print!(".");
        //} else {
        //    print!("{}", *p);
        //}

        if *p > 1 {
            multi_count += 1;
        }
    }

    println!("\nResult\n{}\n", multi_count);
}