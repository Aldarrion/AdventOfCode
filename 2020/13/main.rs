use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;
use std::cmp::Ordering;

//------------------------------------------------------------------------------
struct Event
{
    time: i32,
    buses: Vec<i32>,
}

//------------------------------------------------------------------------------
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

//------------------------------------------------------------------------------
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

//------------------------------------------------------------------------------
impl Eq for Event {}

//------------------------------------------------------------------------------
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//------------------------------------------------------------------------------
fn get_inverse(num: i64, p: i64) -> i64 {
    // There must be a better way to find an inverse in a division ring but this
    // works as well.
    for i in 1..p {
        if (i * num) % p == 1 {
            return i;
        }
    }

    panic!("");
}

fn get_congruence_right_side(a: i64, p: i64, offset: i64) -> i64 {
    /*
    This returns a time t which is a solution to the bus problem
    t % a == 0
    (t + offset) % p == 0

    a = the bus with 0 offset = 37
    p = prime that represents the second bus
    offset = offset of the second bus
    k = iteration of the time of the bus where

    ~ marks a congruence relation
    */

    // We work in mod p
    let a_mod = a % p;

    let offset_mod = p - (offset % p);

    // To sove the equation for x
    // a_mod * x ~ offset_mod (mod p)
    // we need to find an inverse of `a_mod` in (mod p)
    // and multiply the whole equation by it which gives us result
    // = the right side of the congruence equation

    let a_mod_inverse = get_inverse(a_mod, p);
    (offset_mod * a_mod_inverse) % p
}

//------------------------------------------------------------------------------
fn gen_time(a: i64, p: i64, offset: i64, k: i64) -> i64 {
    let res = get_congruence_right_side(a, p, offset);
    // Here we return t as the multiple of prime with given offset, we also need
    // to take the original `a` since we no longer work in (mod p)
    a * (res + k * p)
}

//------------------------------------------------------------------------------
// Soves congruence eqation `a * x + b = c (mod p)`, returns i in `k = i (mod p)`
fn solve_congruence(a: i64, b: i64, c :i64, p: i64) -> i64 {
    let a_mod = a % p;
    let b_mod = b % p;
    let c_mod = c % p;
    let mut rs = c_mod - b_mod;
    if rs < 0 {
        rs += p;
    }
    let inverse = get_inverse(a, p);
    println!("  solving {} * k + {} = {} (mod {}) ==== {}", a, b, c, p, (rs * inverse) % p);
    (rs * inverse) % p
}

//------------------------------------------------------------------------------
fn chinese_reminder_theorem(i: usize, equations: &[(i64, i64, i64)], a: i64, b: i64) -> i64 {
    if i >= equations.len() {
        println!("a: {}, b: {}, ret: {}", a, b, a + b);
        a + b
    } else {
        let (p, _, right_side) = equations[i];
        // x = right_side + k * p
        // x = 17 + k * 14
        // 41 * k + 17 = 12 (mod 13)
        // k = 4 (mod 13)
        // next level...
        let new_rs = solve_congruence(a, b, right_side, p);
        let crp = chinese_reminder_theorem(i + 1, equations, p, new_rs);
        println!("  ret {} + {} * {} = {}", b, a, crp, b + a * crp);
        b + a * crp
    }
}

//------------------------------------------------------------------------------
fn main()
{
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut lines = io::BufReader::new(file).lines();
    let earliest = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let buses_str = lines.next().unwrap().unwrap();
    let mut buses = buses_str.split(",").filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    buses.sort();
    let mut events: Vec::<Event> = buses.iter().map(|x| Event{ time: *x, buses: vec!(*x) }).collect();

    loop {
        let event = events.remove(0);
        if event.time >= earliest {
            let wait_time = event.time - earliest;
            println!("Result 1:\n{}", event.buses[0] * wait_time);
            break;
        } else {
            for b in event.buses.iter() {
                let new_time = event.time + b;
                let new_event = Event{ time: new_time, buses: vec!(*b) };
                match events.binary_search(&new_event) {
                    Ok(pos) => events[pos].buses.push(*b),
                    Err(pos) => events.insert(pos, new_event),
                }
            }
        }
    }

    let mut bus_offsets = Vec::<(i64, i64, i64)>::new();
    let mut offset = 0;
    let mut i = 0;
    for b in buses_str.split(",") {
        if let Ok(p) = b.parse::<i64>() {
            i += 1;
                let right_side = if bus_offsets.is_empty() {
                    0
                } else {
                    get_congruence_right_side(bus_offsets[0].0 as i64, p as i64, offset)
                };
                bus_offsets.push((p, offset, right_side));
                println!("bus: {}, offset: {}, right_side: {}", p, offset, right_side);
        }
        offset += 1;
    }

    println!("");
    let mul = bus_offsets[0].0;
    let (p, _, rs) = bus_offsets[1];
    let res = mul * chinese_reminder_theorem(2, &bus_offsets[0..], p, rs);
    println!("{}\n", res);

    for (bus, offset, _) in bus_offsets {
        println!("bus: {}, offset: {}, reminder: {}", bus, offset, (res + offset) % bus);
    }
}