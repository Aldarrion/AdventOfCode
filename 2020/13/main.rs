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

    let mut bus_offsets = Vec::<(i32, i32)>::new();
    let mut offset = 0;
    for b in buses_str.split(",") {
        if let Ok(n) = b.parse::<i32>() {
            bus_offsets.push((n, offset));
            println!("bus: {}, offset {}", n, offset);
        }
        offset += 1;
    }

    let (max_b, max_o) = bus_offsets.iter().fold((0, 0), |(max_b, max_o), (b, o)| {
        if *b > max_b {
            (*b, *o)
        } else {
            (max_b, max_o)
        }
    });

    println!("max_b {}, max_o {}", max_b, max_o);
    bus_offsets = bus_offsets.iter().map(|(b, o)| (*b, *o - max_o)).collect();

    let mut time = 0i64;
    loop {
        let mut all_fine = true;
        for (b, o) in bus_offsets.iter() {
            if (time + *o as i64) % *b as i64 != 0 {
                time += max_b as i64;
                all_fine = false;
                break;
            }
        }

        if all_fine {
            println!("Result 2:\n{}", time);
            break;
        }
    }
}