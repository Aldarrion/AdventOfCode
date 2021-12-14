use std::fs;
use std::io;
use std::io::BufRead;
use std::vec::Vec;

fn search(graph: &Vec<(String, Vec<usize>)>, start_pos: usize, mut can_visit_small_twice: bool) -> i32 {
    let mut num_paths = 0;

    let mut visited: Vec<&str> = Default::default();
    visited.push("start");
    let mut stack: Vec<(usize, usize, bool)> = Default::default(); // (node_idx, iteration_idx, is_second_small)
    stack.push((start_pos, 0, false));

    while !stack.is_empty() {
        let (node_idx, mut i, second_visit) = *stack.last().unwrap();
        let (node_name, connected) = &graph[node_idx];

        if node_name.eq("end") {
            visited.retain(|s| s != node_name);
            stack.pop();
            num_paths += 1;
            continue;
        }

        loop {
            // Dead end
            if i >= connected.len() {
                if !second_visit {
                    visited.retain(|s| s != node_name);
                } else {
                    can_visit_small_twice = true;
                }

                stack.pop();
                break;
            }

            let target_name = &graph[connected[i]].0[..];
            let is_lower = target_name.chars().next().unwrap().is_ascii_lowercase();
            let is_visited = visited.contains(&target_name);

            if !target_name.eq("start") && (!is_visited || can_visit_small_twice) {
                if is_visited {
                    can_visit_small_twice = false;
                } else if is_lower {
                    visited.push(target_name);
                }
                stack.last_mut().unwrap().1 = i + 1;
                stack.push((connected[i], 0, is_visited));
                break;
            } else {
                i += 1;
            }
        }
    }

    num_paths
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut graph = Vec::<(String, Vec<usize>)>::new();

    let mut start_pos: usize = 0;

    let line_iter = io::BufReader::new(file).lines();
    for line in line_iter {
        if let Ok(l) = line {
            let mut line_split = l.split("-");
            let nodes = [line_split.next().unwrap(), line_split.next().unwrap()];

            let first = graph.iter().position(|(name, _)| name.eq(nodes[0]));
            let second = graph.iter().position(|(name, _)| name.eq(nodes[1]));

            let first = if first == None {
                graph.push((nodes[0].to_string(), vec![]));
                graph.len() - 1
            } else {
                first.unwrap()
            };

            if nodes[0].eq("start") {
                start_pos = first;
            }

            let second = if second == None {
                graph.push((nodes[1].to_string(), vec![]));
                graph.len() - 1
            } else {
                second.unwrap()
            };

            if nodes[1].eq("start") {
                start_pos = second;
            }

            graph[first].1.push(second);
            graph[second].1.push(first);
        }
    }

    let first_result  = search(&graph, start_pos, false);
    let second_result = search(&graph, start_pos, true);

    //for (k, v) in graph.iter() {
    //    print!("{} -", k);
    //    for n in v {
    //        print!(" {}", graph[*n].0);
    //    }
    //    println!("");
    //}

    println!("First\n{}\nSecond\n{}\n", first_result, second_result);
}