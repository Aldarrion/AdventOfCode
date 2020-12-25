use std::fs;
use std::io;
use std::io::BufRead;
use std::str;

#[derive(Copy, Clone)]
enum Op {
    None,
    Add,
    Mul,
}

fn do_op(op: Op, x: i64, y: i64) -> i64 {
    match op {
        Op::Add => x + y,
        Op::Mul => x * y,
        Op::None => x + y,
    }
}

fn eval(mut acc: i64, mut expr: &str) -> (i64, &str) {
    let mut op = Op::None;
    while !expr.is_empty() {
        let first_char = expr.chars().nth(0).unwrap();
        if first_char == ')' {
            expr = &expr[1..];
            return (acc, expr);
        } else if first_char == '(' {
            expr = &expr[1..];
            let (to_add, new_expr) = eval(0, &expr);
            expr = new_expr;
            acc = do_op(op, acc, to_add);
        } else if first_char == '+' {
            op = Op::Add;
            expr = &expr[1..];
        } else if first_char == '*' {
            op = Op::Mul;
            expr = &expr[1..];
        } else {
            let tags: &[_] = &['+', '*', ')'];
            match expr.find(tags) {
                None => {
                    let num = expr.parse::<i64>().unwrap();
                    acc = do_op(op, acc, num);
                    return (acc, expr);
                },
                Some(pos) => {
                    let num = expr[..pos].parse::<i64>().unwrap();
                    acc = do_op(op, acc, num);
                    expr = &expr[pos..];
                },
            }
        }
    }
    (acc, expr)
}

fn main() {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let lines = io::BufReader::new(file).lines();
    let mut res = 0i64;
    for line in lines {
        if let Ok(line) = line {
            let mut line = line.replace(' ', "");

            let (line_res, _) = eval(0i64, &mut line);
            res += line_res;
        }
    }

    println!("Sum of results:\n{}", res);
}
