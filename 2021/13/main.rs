/*
--- Day 13: Transparent Origami ---

You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.

Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:

Congratulations on your purchase! To activate this infrared thermal imaging
camera system, please enter the code found on page 1 of the manual.

Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:

6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5

The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):

...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
-----------
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........

Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:

#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........

Now, only 17 dots are visible.

Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.

Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.

The second fold instruction is fold along x=5, which indicates this line:

#.##.|#..#.
#...#|.....
.....|#...#
#...#|.....
.#.#.|#.###
.....|.....
.....|.....

Because this is a vertical line, fold left:

#####
#...#
#...#
#...#
#####
.....
.....

The instructions made a square!

The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.

How many dots are visible after completing just the first fold instruction on your transparent paper?

Your puzzle answer was 743.
--- Part Two ---

Finish folding the transparent paper according to the instructions. The manual says the code is always eight capital letters.

What code do you use to activate the infrared thermal imaging camera system?

Your puzzle answer was RCPLAKHL.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn print_paper(paper: &Vec<(i32, i32)>, width: i32, height: i32) {
    for y in 0..(height + 1) {
        for x in 0..(width + 1) {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut first_result = 0;

    let mut width = 0;
    let mut height = 0;

    let mut paper = Vec::<(i32, i32)>::new();
    let mut folds = Vec::<(i8, i32)>::new();

    let mut is_reading_coords = true;
    let line_iter = io::BufReader::new(file).lines();
    for line in line_iter {
        if let Ok(l) = line {
            if l.is_empty() {
                is_reading_coords = false;
                continue;
            }
            if is_reading_coords {
                let mut split = l.split(',');
                let x = split.next().unwrap().parse::<i32>().unwrap();
                let y = split.next().unwrap().parse::<i32>().unwrap();

                if x >= width {
                    width = x + 1;
                }
                if y >= height {
                    height = y + 1;
                }

                paper.push((x, y));

            } else {
                let mut split = l.rsplit(' ').next().unwrap().split('=');
                let axis = if split.next().unwrap() == "x" { 0i8 } else { 1i8 };
                let distance = split.next().unwrap().parse::<i32>().unwrap();
                folds.push((axis, distance));
            }
        }
    }

    for (i, (axis, dist)) in folds.iter().enumerate() {
        let mut new_width = width;
        let mut new_height = height;
        if *axis == 0 {
            new_width = *dist;
        } else {
            new_height = *dist;
        }

        let mut new_points = Vec::<(i32, i32)>::new();
        for (x, y) in paper.iter() {
            let _xx = x;
            let _yy = y;
            if *x > width || *y > height {
                continue;
            }

            if *x > new_width {
                new_points.push((width - 1 - *x, *y));
            } else if *y > new_height {
                new_points.push((*x, height - 1 - *y));
            }
        }

        paper.append(&mut new_points);

        width = new_width;
        height = new_height;

        if i == 0 {
            paper.sort();
            paper.dedup();

            for (x, y) in paper.iter() {
                if *x <= width && *y <= height {
                    first_result += 1;
                }
            }
        }
    }

    println!("First\n{}\nSecond", first_result);
    print_paper(&paper, width, height);
}