/*
--- Day 14: Extended Polymerization ---

The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

For example:

NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C

The first line is the polymer template - this is the starting point of the process.

The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.

So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:

    The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
    The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
    The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.

Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.

After the first step of this process, the polymer becomes NCNBCHB.

Here are the results of a few steps using the above rules:

Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.

Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?

Your puzzle answer was 2447.
--- Part Two ---

The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.

In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.

Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?

Your puzzle answer was 3018019237563.
*/

use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashMap;

fn expand(steps: i32, initial_state: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    //println!("Expanding {} {}", initial_state[0], initial_state[1]);
    let mut state = initial_state.clone();

    for _s in 0..steps {
        let mut next_state = Vec::<char>::new();
        for i in 0..(state.len() - 1) {
            let frame = (state[i], state[i + 1]);
            next_state.push(state[i]);
            if let Some(new) = rules.get(&frame) {
                next_state.push(*new);
            }
        }

        next_state.push(*state.last().unwrap());
        state = next_state;
    }

    state
}

fn count_chars(state: &[char], dst: &mut [i64;26]) -> [i64;26] {
    let mut histogram = [0i64; 26];
    for c in state {
        let idx = *c as i8 - 'A' as i8;
        histogram[idx as usize] += 1;
    }
    add_to_histogram(dst, &histogram);
    histogram
}

fn add_to_histogram(dst: &mut [i64], src: &[i64]) {
    for i in 0..dst.len() {
        dst[i] += src[i];
    }
}

fn min_max_diff(histogram: &[i64]) -> i64 {
    let mut min: i32 = -1;
    let mut max: i32 = -1;
    for (i, n) in histogram.iter().enumerate() {
        if *n == 0 {
            continue;
        }

        if min == -1 || histogram[min as usize] > *n {
            min = i as i32;
        }
        if max == -1 || histogram[max as usize] < *n {
            max = i as i32;
        }
    }

    histogram[max as usize] - histogram[min as usize]
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut line_iter = io::BufReader::new(file).lines();
    let state: Vec<char> = line_iter.next().unwrap().unwrap().chars().collect();
    line_iter.next();

    let mut rules: HashMap<(char, char), char> = Default::default();
    let mut rules_after_20: HashMap<(char, char), Vec<char>> = Default::default();

    for line in line_iter {
        if let Ok(l) = line {
            let mut split = l.split(" -> ");
            let key: Vec<char> = split.next().unwrap().to_string().chars().collect();
            let key: (char, char) = (key[0], key[1]);
            let value = split.next().unwrap().chars().next().unwrap();
            rules.insert(key, value);
        }
    }

    let mut first_histogram = [0i64; 26];
    let first_state = expand(10, &state, &rules);
    count_chars(&first_state, &mut first_histogram);
    let first_result = min_max_diff(&first_histogram);

    for r in rules.keys() {
        let rule = vec![r.0, r.1];
        let expanded_rule = expand(20, &rule, &rules);
        rules_after_20.insert(*r, expanded_rule);
    }
    println!("DONE expanding rules");

    let mut cache = HashMap::<(char, char), [i64; 26]>::new();
    let mut second_histogram = [0i64; 26];
    for i in 0..(state.len() - 1) {
        let frame = (state[i], state[i + 1]);
        println!("Working on {}{}", frame.0, frame.1);
        if let Some(expanded_rule) = rules_after_20.get(&frame) {
            for j in 0..(expanded_rule.len() - 1) {
                let frame_2 = (expanded_rule[j], expanded_rule[j + 1]);
                if let Some(expanded_rule_2) = rules_after_20.get(&frame_2) {
                    if let Some(cached) = cache.get(&frame_2) {
                        add_to_histogram(&mut second_histogram, cached);
                    } else {
                        let new_histogram = count_chars(&expanded_rule_2[..expanded_rule_2.len() - 1], &mut second_histogram);
                        cache.insert(frame_2, new_histogram);
                    }
                } else {
                    count_chars(&[frame_2.0], &mut second_histogram);
                }
            }
        } else {
            count_chars(&[frame.0], &mut second_histogram);
        }
    }
    count_chars(&state[state.len() - 1..state.len()], &mut second_histogram);

    let second_result = min_max_diff(&second_histogram);

    println!("First\n{}\nSecond\n{}\n", first_result, second_result);
}