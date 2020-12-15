
/*
--- Day 13: Shuttle Search ---

Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.

Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.

Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.

The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!

Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.

To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)

For example, suppose you have the following notes:

939
7,13,x,x,59,x,31,19

Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:

time   bus 7   bus 13  bus 59  bus 31  bus 19
929      .       .       .       .       .
930      .       .       .       D       .
931      D       .       .       .       D
932      .       .       .       .       .
933      .       .       .       .       .
934      .       .       .       .       .
935      .       .       .       .       .
936      .       D       .       .       .
937      .       .       .       .       .
938      D       .       .       .       .
939      .       .       .       .       .
940      .       .       .       .       .
941      .       .       .       .       .
942      .       .       .       .       .
943      .       .       .       .       .
944      .       .       D       .       .
945      D       .       .       .       .
946      .       .       .       .       .
947      .       .       .       .       .
948      .       .       .       .       .
949      .       D       .       .       .

The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.

What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?

Your puzzle answer was 261.
--- Part Two ---

The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)

For example, suppose you have the same list of bus IDs as above:

7,13,x,x,59,x,31,19

An x in the schedule means there are no constraints on what bus IDs must depart at that time.

This means you are looking for the earliest timestamp (called t) such that:

    Bus ID 7 departs at timestamp t.
    Bus ID 13 departs one minute after timestamp t.
    There are no requirements or restrictions on departures at two or three minutes after timestamp t.
    Bus ID 59 departs four minutes after timestamp t.
    There are no requirements or restrictions on departures at five minutes after timestamp t.
    Bus ID 31 departs six minutes after timestamp t.
    Bus ID 19 departs seven minutes after timestamp t.

The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.

In this example, the earliest timestamp at which this occurs is 1068781:

time     bus 7   bus 13  bus 59  bus 31  bus 19
1068773    .       .       .       .       .
1068774    D       .       .       .       .
1068775    .       .       .       .       .
1068776    .       .       .       .       .
1068777    .       .       .       .       .
1068778    .       .       .       .       .
1068779    .       .       .       .       .
1068780    .       .       .       .       .
1068781    D       .       .       .       .
1068782    .       D       .       .       .
1068783    .       .       .       .       .
1068784    .       .       .       .       .
1068785    .       .       D       .       .
1068786    .       .       .       .       .
1068787    .       .       .       D       .
1068788    D       .       .       .       D
1068789    .       .       .       .       .
1068790    .       .       .       .       .
1068791    .       .       .       .       .
1068792    .       .       .       .       .
1068793    .       .       .       .       .
1068794    .       .       .       .       .
1068795    D       D       .       .       .
1068796    .       .       .       .       .
1068797    .       .       .       .       .

In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.

Here are some other examples:

    The earliest timestamp that matches the list 17,x,13,19 is 3417.
    67,7,59,61 first occurs at timestamp 754018.
    67,x,7,59,61 first occurs at timestamp 779210.
    67,7,x,59,61 first occurs at timestamp 1261476.
    1789,37,47,1889 first occurs at timestamp 1202161486.

However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!

What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?

Your puzzle answer was 807435693182510.
*/

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
    //println!("inverse: {}, {} = {}", a, p, inverse);
    let res = (rs * inverse) % p;
    println!("  solving {} * k + {} = {} (mod {}) ==== {}", a_mod, b_mod, c_mod, p, res);
    res
}

//------------------------------------------------------------------------------
fn chinese_remainder_theorem(i: usize, equations: &[(i64, i64)], a: i64, b: i64) -> i64 {
    if i >= equations.len() {
        println!("a: {}, b: {}, ret: {}", a, b, a + b);
        a + b
    } else {
        let (p, offset) = equations[i];
        let new_rs = solve_congruence(a, offset + b, 0, p);
        let crt = chinese_remainder_theorem(i + 1, equations, a * p, a * new_rs + b);
        crt
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

    /*
    We need to observe some things about the input such as that the bus intervals are primes.
    We are looking for a whole-number solution `x` to a set of equations, where x is the earliest time.

    We can represent this as a set of congruence eqations in the following form:
    ~ marks congruence relation.
    x is the earliest time we are looking for

    x      ~ 0 (mod 37)
    x + 27 ~ 0 (mod 41)
    x + 37 ~ 0 (mod 457)
    ...

    Since in the (mod p) part the p is always a prime we can use the Chinese remainder theorem without further
    thinking about it.
    The solution is to change the equation to the form `x ~ a (mod p)` where y is the unknown and a and p are from Zp.
    Then from this we can equate y using a parameter `k` like so: `x = p * k + a` and put this instead of x to the previous
    equation. Since we still have parameter k and more equations we do the same thing with the next equation and modify it to
    `k ~ b (mod p2)` => `k = p2 * l + b`. Now we have x in terms of the parameter l. We do this all the way recursively
    until we exhaust all the equations. Which gives us the final result.
    As the theorem states the result is valid modulo all the primes multiplied. So we do that in the end.

    */

    let mut bus_offsets = Vec::<(i64, i64)>::new();
    let mut offset = 0;
    for b in buses_str.split(",") {
        if let Ok(p) = b.parse::<i64>() {
            bus_offsets.push((p, offset));
            println!("bus: {}, offset: {}", p, offset);
        }
        offset += 1;
    }

    println!("");
    let (p, o) = bus_offsets[0];
    let res = chinese_reminder_theorem(1, &bus_offsets[0..], p, o);

    let mut total_mod = 1;
    for (bus, offset) in bus_offsets {
        total_mod *= bus;
        println!("bus: {}, offset: {}, reminder: {}", bus, offset, (res + offset) % bus);
    }

    println!("Total result:\n{}", res % total_mod);
}