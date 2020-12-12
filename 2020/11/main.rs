
/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##

This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##

At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

Your puzzle answer was 2283.
--- Part Two ---

As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....

The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

.............
.L.L.#.#.#.#.
.............

The empty seat below would see no occupied seats:

.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.

Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#

#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?

Your puzzle answer was 2054.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

//------------------------------------------------------------------------------
#[derive(std::cmp::PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
enum Spot
{
    Empty,
    Occupied,
    Floor,
}

type StateT = Vec<Vec<Spot>>;

//------------------------------------------------------------------------------
fn get_spot(state: &StateT, x: i32, y: i32) -> Spot
{
    if x < 0 || y < 0 || x >= state[0].len() as i32 || y >= state.len() as i32 {
        Spot::Empty
    } else {
        state[y as usize][x as usize]
    }
}

//------------------------------------------------------------------------------
fn step_first(a: usize, b: usize, states: &mut [StateT]) -> bool
{
    let neighbors: [(i32, i32); 8] =
    [
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
        (-1,  0),
        ( 1,  0),
        (-1, -1),
        ( 0, -1),
        ( 1, -1)
    ];

    let mut any_change = false;

    for y in 0..states[a].len()
    {
        for x in 0..states[a][y].len()
        {
            let mut occupied_count = 0;
            for (n_x, n_y) in neighbors.iter()
            {
                let n_state = get_spot(&states[a], x as i32 + n_x, y as i32 + n_y);
                if n_state == Spot::Occupied {
                    occupied_count += 1;
                }
            }
            let prev_state = get_spot(&states[a], x as i32, y as i32);
            match prev_state {
                Spot::Occupied if occupied_count >= 4 => {
                    any_change = true;
                    states[b][y][x] = Spot::Empty;
                },
                Spot::Empty if occupied_count == 0 => {
                    any_change = true;
                    states[b][y][x] = Spot::Occupied;
                },
                _ => states[b][y][x] = prev_state,
            }
        }
    }

    any_change
}

//------------------------------------------------------------------------------
fn step_second(a: usize, b: usize, states: &mut [StateT]) -> bool
{
    let dirs: [(i32, i32); 8] =
    [
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
        (-1,  0),
        ( 1,  0),
        (-1, -1),
        ( 0, -1),
        ( 1, -1)
    ];

    let mut any_change = false;

    for y in 0..states[a].len()
    {
        for x in 0..states[a][y].len()
        {
            let mut occupied_count = 0;
            for (d_x, d_y) in dirs.iter()
            {
                let mut i = 1;
                loop {
                    let d_state = get_spot(&states[a], x as i32 + d_x * i, y as i32 + d_y * i);
                    i += 1;
                    if d_state == Spot::Occupied {
                        occupied_count += 1;
                        break;
                    } else if d_state == Spot::Empty {
                        break;
                    }
                }
            }
            let prev_state = get_spot(&states[a], x as i32, y as i32);
            match prev_state {
                Spot::Occupied if occupied_count >= 5 => {
                    any_change = true;
                    states[b][y][x] = Spot::Empty;
                },
                Spot::Empty if occupied_count == 0 => {
                    any_change = true;
                    states[b][y][x] = Spot::Occupied;
                },
                _ => states[b][y][x] = prev_state,
            }
        }
    }

    any_change
}

//------------------------------------------------------------------------------
fn sum_occupied(state: &StateT) -> i32
{
    let mut occupied_count = 0;
    for y in 0..state.len() {
        for x in 0..state[y].len() {
            if state[y][x] == Spot::Occupied {
                occupied_count += 1;
            }
        }
    }

    // Eh, ez win fo plain old for loops above
    //let occupied_count = state[0].iter()
    //    .fold(0, |acc, v| {
    //        acc + v.iter()
    //            .fold(0, |v_acc, spot| {
    //                if *spot == Spot::Occupied {
    //                    v_acc + 1
    //                } else {
    //                    v_acc
    //                }
    //            })
    //    });
    occupied_count
}

//------------------------------------------------------------------------------
fn main()
{
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut state = StateT::new();
    for line in io::BufReader::new(file).lines()
    {
        if let Ok(line) = line
        {
            let mut line_vec = Vec::<Spot>::new();
            for c in line.chars()
            {
                match c {
                    '.' => line_vec.push(Spot::Floor),
                    'L' => line_vec.push(Spot::Empty),
                    _ => panic!("unknown character"),
                }
            }
            state.push(line_vec.clone());
        }
    }

    let init_states = |to_init: &mut [StateT], init_from: &StateT| {
        for y in 0..init_from.len() {
            for s in 0..to_init.len() {
                to_init[s].push(init_from[y].clone());
            }
        }
    };

    type StepFnT = fn (usize, usize, &mut [StateT]) -> bool;
    let run_sim = |state: &StateT, step_fn: StepFnT| -> i32
    {
        let mut states = [Vec::<Vec<Spot>>::new(), Vec::<Vec<Spot>>::new()];
        init_states(&mut states, state);
        let mut i = 0;
        while step_fn(i % 2, (i + 1) % 2, &mut states) {
            i += 1;
        }

        sum_occupied(&states[0])
    };

    let occupied_count = run_sim(&state, step_first);
    println!("Occupied when stable first rules:\n{}", occupied_count);

    let occupied_count = run_sim(&state, step_second);
    println!("Occupied when stable second rules:\n{}", occupied_count);
}