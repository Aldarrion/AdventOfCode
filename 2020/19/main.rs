use std::fs;
use std::io;
use std::io::BufRead;
use std::str;
use std::collections::HashMap;

enum RuleVal {
    Rule(i32),
    Text(String),
}

/*
0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

0 = 1 opt with 2 vals
1 = 1 opt with 1 val
2 = 2 opts with 2 vals each
3 = 1 opt with 1 val

2 = [ab, ba]
0 = [aab, aba]



0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

3 = [ab, ba]
2 = [aa, bb]
1 = [abaa, abbb, baaa, babb]
0 = [aabaab, aabbbb, abaaab, ababbb]
*/

fn gen_all(rules: &HashMap<i32, Vec<Vec<RuleVal>>>, rule_num: i32) -> Vec<String> {
    let rule = rules.get(&rule_num).unwrap();

    let mut opts = Vec::new();
    for opt in rule {
        let mut opt_vars = vec!["".to_string()];
        for val in opt {
            let gen_opts = match val {
                RuleVal::Text(t) => vec![t.clone()],
                RuleVal::Rule(opt_rule_num) => gen_all(&rules, *opt_rule_num),
            };

            let mut opt_vars_2 = Vec::new();
            for opt_var in &mut opt_vars {
                for gen_opt in &gen_opts {
                    opt_vars_2.push(opt_var.to_owned() + gen_opt);
                }
            }
            opt_vars = opt_vars_2;
        }

        opts.append(&mut opt_vars);
    }

    opts
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let mut rules = HashMap::new();

    let mut lines = io::BufReader::new(file).lines();
    for line in &mut lines {
        if let Ok(line) = line {
            //println!("{}", line);
            if line.is_empty() {
                break;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            let num_str = tokens[0].trim_end_matches(':');
            let rule_num = num_str.parse::<i32>().unwrap();

            let mut options = Vec::new();
            options.push(Vec::new());
            for i in 1..tokens.len() {
                if tokens[i] == "|" {
                    options.push(Vec::new());
                } else {
                    let opt_i = options.len() - 1;
                    match tokens[i].parse::<i32>() {
                        Ok(rule) => {
                            options[opt_i].push(RuleVal::Rule(rule));
                        },
                        Err(_) => {
                            let c = tokens[i].trim_matches('"');
                            options[opt_i].push(RuleVal::Text(c.to_string()));
                        }
                    }
                }
            }

            rules.insert(rule_num, options);
        }
    }

    let acceptable = gen_all(&rules, 0);

    //for a in acceptable {
    //    println!("{}", a);
    //}

    let mut num_accepted = 0;
    for line in lines {
        if let Ok(line) = line {
            for a in &acceptable {
                if a == &line {
                    num_accepted += 1;
                    break;
                }
            }
        }
    }

    println!("Accepted count:\n{}", num_accepted);
}