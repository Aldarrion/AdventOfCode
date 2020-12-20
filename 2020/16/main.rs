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

    let mut possible_fields: Vec<Vec<i32>> = vec![(0..fields.len() as i32).collect(); fields.len()];
    for ticket_i in 0..tickets.len() {
        for (ticket_field_i, n) in tickets[ticket_i].iter().enumerate() {
            for field_i in (0..possible_fields[ticket_field_i].len()).rev() {
                let mut can_be = false;
                let f = &fields[possible_fields[ticket_field_i][field_i] as usize];
                for (min, max) in &f.ranges {
                    if *n >= *min && *n <= *max {
                        can_be = true;
                    }
                }
                if !can_be {
                    possible_fields[ticket_field_i].remove(field_i);
                }
            }
        }
    }

    let mut certain_fields = vec![-1; fields.len()];
    let mut fields_to_match = possible_fields.len() as i32;
    while fields_to_match > 0 {
        for i in 0..possible_fields.len() {
            if possible_fields[i].len() == 1 {
                certain_fields[i] = possible_fields[i][0];
                fields_to_match -= 1;
                let num_to_remove = certain_fields[i];
                for pf_i in 0..possible_fields.len() {
                    for f_i in 0..possible_fields[pf_i].len() {
                        if possible_fields[pf_i][f_i] == num_to_remove {
                            possible_fields[pf_i].remove(f_i);
                            break;
                        }
                    }
                }
                break;
            }
        }
    }

    let mut res = 1 as u64;
    for i in 0..certain_fields.len() {
        let f = &fields[certain_fields[i] as usize];
        if let Some(0) = f.name.find("departure") {
            res *= my_ticket[i] as u64;
        }
    }

    println!("Multiplied departures:\n{}", res);
}
