use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn find_idx(all: &[&str], id: &str) -> (usize, u32) {
    let i = all.iter().position(|&s| s == id).unwrap();
    (i, 1 << i)
}

fn validate(idx: usize, value: &str) -> bool {
    let ranges = [
        [1920, 2002],
        [2010, 2020],
        [2020, 2030],
    ];

    let eye_colors = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ];

    let height_extensions = [ "cm", "in" ];
    let height_ranges = [ [150, 193], [59, 76] ];

    match idx {
        0..=2 => {
            if let Ok(int_val) = value.parse::<u32>() {
                if int_val >= ranges[idx][0]
                && int_val <= ranges[idx][1]
                {
                    return true;
                }
            }
        },
        3 => {
            for (i, ext) in height_extensions.iter().enumerate() {
                let hg: Vec<&str> = value.split(ext).collect();
                if hg.len() != 2 {
                    continue;
                }

                if let Ok(hg_num) = hg[0].parse::<u32>() {
                    if hg_num >= height_ranges[i][0]
                        && hg_num <= height_ranges[i][1]
                    {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        },
        4 => {
            if value.len() != 7 {
                return false;
            }

            if let Ok(_) = i64::from_str_radix(&value[1..], 16) {
                return true;
            }
        }
        5 => return eye_colors.contains(&value),
        6 => {
            if value.len() != 9 {
                return false;
            } else if let Ok(_) = value.parse::<u32>() {
                return true;
            }
        },
        _ => return false,
    }

    false
}

fn main() {
    let all = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        "cid",
    ];

    let required = (1 << 7) - 1;

    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut partially_valid_count = 0;
    let mut valid_count = 0;
    let mut current_valid_fields = 0;
    let mut current_fields = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l.is_empty() {
                // Evaluate current passport
                if (current_fields & required) == required {
                    //println!("current: {:b}, required:{:b}", current_document, required);
                    partially_valid_count += 1;
                }
                current_fields = 0;

                if (current_valid_fields & required) == required {
                    valid_count += 1;
                }
                current_valid_fields = 0;
            } else {
                for pair in l.split(' ') {
                    let pair_vec: Vec<&str> = pair.split(':').collect();
                    if pair_vec.len() == 2 {
                        let key = pair_vec[0];
                        let value = pair_vec[1];
                        let (idx, mask) = find_idx(&all, key);
                        current_fields |= mask;

                        if validate(idx, value) {
                            current_valid_fields |= mask;
                        }

                        //println!("pair: {}, mask: {:b}", pair, current_document)
                    } else {
                        panic!("key not found in pair");
                    }
                }
            }
        }
    }

    if (current_fields & required) == required {
        //println!("current: {:b}, required:{:b}", current_document, required);
        partially_valid_count += 1;
    }

    if (current_valid_fields & required) == required {
        valid_count += 1;
    }

    println!("Partially valid:\n{}", partially_valid_count);
    println!("Valid:\n{}", valid_count);
}