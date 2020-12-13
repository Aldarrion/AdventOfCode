
/*
--- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.

The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11

These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
    F11 would move the ship 11 units south to east 17, south 8.

At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

Your puzzle answer was 2280.
--- Part Two ---

Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

    Action N means to move the waypoint north by the given value.
    Action S means to move the waypoint south by the given value.
    Action E means to move the waypoint east by the given value.
    Action W means to move the waypoint west by the given value.
    Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    Action F means to move forward to the waypoint a number of times equal to the given value.

The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

    F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
    N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
    F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
    R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
    F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.

After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?

Your puzzle answer was 38693.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

//------------------------------------------------------------------------------
enum Instruction
{
    Move(i32, i32),
    Forward(i32),
    Rotate(i32),
}

//------------------------------------------------------------------------------
fn main()
{
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut instructions = Vec::<Instruction>::new();
    for line in io::BufReader::new(file).lines()
    {
        let line = line.unwrap();
        let num = line[1..].parse::<i32>().unwrap();
        let line = line.as_bytes();
        let instruction = match line[0] as char {
            'F' => Instruction::Forward(num),
            'L' => Instruction::Rotate(-num / 90),
            'R' => Instruction::Rotate(num / 90),
            'N' => Instruction::Move(0, num),
            'S' => Instruction::Move(0, -num),
            'E' => Instruction::Move(num, 0),
            'W' => Instruction::Move(-num, 0),
            _ => panic!(),
        };

        instructions.push(instruction);
    }

    // Part 1
    {
        let dir_vectors = [
            (1, 0),
            (0, -1),
            (-1, 0),
            (0, 1),
        ];

        let mut ship_x = 0;
        let mut ship_y = 0;
        let mut ship_dir: i32 = 0; // East

        for instruction in instructions.iter() {
            let (dx, dy) = dir_vectors[ship_dir as usize];
            match instruction {
                Instruction::Forward(n) => {
                    ship_x += n * dx;
                    ship_y += n * dy;
                },
                Instruction::Move(mx, my) => {
                    ship_x += mx;
                    ship_y += my;
                },
                Instruction::Rotate(num) => {
                    ship_dir += num;
                    while ship_dir < 0 {
                        ship_dir += 4;
                    }
                    while ship_dir >= 4 {
                        ship_dir -= 4;
                    }
                },
            }
        }

        let dist_x = ship_x.abs();
        let dist_y = ship_y.abs();

        println!("Distance sum 1:\n{}", dist_x + dist_y);
    }

    // Part 2
    {
        let rot_signs = [
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ];

        let mut ship_x = 0;
        let mut ship_y = 0;
        let mut wp_x = 10;
        let mut wp_y = 1;

        for instruction in instructions.iter() {
            match instruction {
                Instruction::Forward(n) => {
                    ship_x += n * wp_x;
                    ship_y += n * wp_y;
                },
                Instruction::Move(mx, my) => {
                    wp_x += mx;
                    wp_y += my;
                },
                Instruction::Rotate(num) => {
                    let mut rotation = *num;
                    while rotation < 0 {
                        rotation += 4;
                    }
                    while rotation >= 4 {
                        rotation -= 4;
                    }
                    assert!(rotation >= 0 && rotation < 4);

                    let mut new_wp_x = wp_x;
                    let mut new_wp_y = wp_y;
                    if rotation != 2 {
                        new_wp_x = wp_y;
                        new_wp_y = wp_x;
                    }

                    let (sx, sy) = rot_signs[rotation as usize];
                    wp_x = new_wp_x * sx;
                    wp_y = new_wp_y * sy;
                },
            }
        }

        let dist_x = ship_x.abs();
        let dist_y = ship_y.abs();

        println!("Distance sum 2:\n{}", dist_x + dist_y);
    }
}