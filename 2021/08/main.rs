

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

type Segment = [i8; 7];
type Digit = [Segment; 7];

fn restrict_segment(segment: &mut Segment, new_data: &[i8]) {
    for i in 0..7 {
        if segment[i] == 1 && new_data.contains(&(i as i8)) {
            segment[i] = 1;
        } else {
            segment[i] = 0;
        }
    }
}

fn restrict_digits(digits: &mut [Digit], segments: &[i8], new_data: &[i8]) {
    for d in digits {
        for s in segments {
            restrict_segment(&mut d[*s as usize], new_data);
        }
    }
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let line_iter = io::BufReader::new(file).lines();

    let unknown_segment: Segment = [1; 7];
    let unknown_digit: Digit = [unknown_segment; 7];

    let segments_for_number: [Vec<i8>; 10] = [
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];

    let mut easy_digit_count = 0;
    for line in line_iter {
        if let Ok(l) = line {
            let two_vec: Vec<&str> = l.split(" | ").collect();
            let mut possible_digits = vec![unknown_digit];

            for v in two_vec.iter() {
                for entry in v.split(' ') {
                    let nums: Vec<i8> = entry.bytes().map(|c| (c as i8) - ('a' as i8)).collect();

                    if entry.len() == 2 {
                        restrict_digits(&mut possible_digits, &segments_for_number[1], &nums);
                    } else if entry.len() == 4 {
                        restrict_digits(&mut possible_digits, &segments_for_number[4], &nums);
                    } else if entry.len() == 3 {
                        restrict_digits(&mut possible_digits, &segments_for_number[7], &nums);
                    } else if entry.len() == 6 { // 0 or 6 or 9
                        let count = possible_digits.len();
                        for _ in 0..2 {
                            for i in 0..count {
                                possible_digits.push(possible_digits[i]);
                            }
                        }
                        restrict_digits(&mut possible_digits[0..count], &segments_for_number[0], &nums);
                        restrict_digits(&mut possible_digits[count..2 * count], &segments_for_number[6], &nums);
                        restrict_digits(&mut possible_digits[2 * count..3 * count], &segments_for_number[9], &nums);
                    } else if entry.len() == 5 { // 2 or 3 or 5
                        let count = possible_digits.len();
                        for _ in 0..2 {
                            for i in 0..count {
                                possible_digits.push(possible_digits[i]);
                            }
                        }
                        restrict_digits(&mut possible_digits[0..count], &segments_for_number[2], &nums);
                        restrict_digits(&mut possible_digits[count..2 * count], &segments_for_number[3], &nums);
                        restrict_digits(&mut possible_digits[2 * count..3 * count], &segments_for_number[5], &nums);
                    }
                }

                possible_digits.retain(|d| {
                    for s in d {
                        let mut all_zero = true;
                        for n in s {
                            if *n != 0i8 {
                                all_zero = false;
                            }
                        }
                        if all_zero {
                            return false;
                        }
                    }
                    return true;
                });
            }

            let mut intersection = unknown_digit;
            for d in possible_digits.iter() {
                for s_i in 0..d.len() {
                    for n_i in 0..d[s_i].len() {
                        if d[s_i][n_i] == 0 {
                            intersection[s_i][n_i] = 0;
                        }
                    }
                }
            }

            println!("Possible digit count: {}", possible_digits.len());

            for entry in two_vec[1].split(' ') {
                if entry.len() == 2 || entry.len() == 7 || entry.len() == 4 || entry.len() == 3 {
                    easy_digit_count += 1
                }
            }

        }
    }

    println!("Easy digit count\n{}\n", easy_digit_count);
}