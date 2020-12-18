use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

struct Field {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Field {
    fn new() -> Field {
        Field {
            name: "".to_string(),
            ranges: Vec::<(i32, i32)>::new()
        }
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let lines = io::BufReader::new(file).lines();

    let mut my_ticket = Vec::<i32>::new();
    let mut tickets = Vec::<Vec<i32>>::new();
    let mut fields = Vec::<Field>::new();
    let mut segment = 0;

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                segment += 1;
                continue;
            }

            if segment == 0 {
                let mut field = Field::new();
                let mut field_str = line.split(": ");
                field.name = field_str.nth(0).unwrap().into();
                let ranges_str = field_str.nth(0).unwrap().split(" or ");
                for range_str in ranges_str {
                    let mut range_min_max = range_str.split("-");
                    let min = range_min_max.nth(0).unwrap().parse::<i32>().unwrap();
                    let max = range_min_max.nth(0).unwrap().parse::<i32>().unwrap();
                    field.ranges.push((min, max));
                }
                fields.push(field);
            } else if segment == 2 {
                my_ticket = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            } else if segment == 4 {
                tickets.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
            } else {
                segment += 1;
            }
        }
    }

    let mut error_rate = 0;
    for ticket_i in (0..tickets.len()).rev() {
        let mut is_ticket_ok = true;
        for n in &tickets[ticket_i] {
            let mut is_ok = false;
            for f in &fields {
                for (min, max) in &f.ranges {
                    if *n >= *min && *n <= *max {
                        is_ok = true;
                    }
                }
            }
            if !is_ok {
                error_rate += n;
                is_ticket_ok = false;
            }
        }
        if !is_ticket_ok {
            tickets.remove(ticket_i);
        }
    }

    println!("Error rate:\n{}", error_rate);
}