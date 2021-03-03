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

fn first_part() -> i64 {
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

    res
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Open,
    Close,
}

fn get_num(t: Token) -> i64 {
    if let Token::Num(n) = t {
        n
    } else {
        panic!("not a number");
    }
}

fn eval_last_op(stack: &mut Vec<Token>) {
    let a = get_num(stack.pop().unwrap());
    let op = stack.pop().unwrap();
    let b = get_num(stack.pop().unwrap());

    let res = match op {
        Token::Add => a + b,
        Token::Mul => a * b,
        _ => panic!("bad op"),
    };

    stack.push(Token::Num(res));
}

fn eval_line(line: &str) -> i64 {
    let tokens = line.split_whitespace();

    let mut stack = vec![Token::Num(0), Token::Add];
    for token in tokens {
        match token {
            "+" => stack.push(Token::Add),
            "*" => stack.push(Token::Mul),
            "(" => stack.push(Token::Open),
            ")" => stack.push(Token::Close),
            _ => stack.push(Token::Num(token.parse::<i64>().unwrap())),
        }

        // Process stack
        match stack[stack.len() - 1] {
            Token::Close => {
                stack.pop();
                while stack.len() > 2 && stack[stack.len() - 2] != Token::Open {
                    eval_last_op(&mut stack);
                }

                let val = stack.pop().unwrap();
                stack.pop();
                stack.push(val);

                while stack.len() > 2 && stack[stack.len() - 2] == Token::Add {
                    eval_last_op(&mut stack);
                }
            },
            Token::Num(_) => {
                if stack[stack.len() - 2] == Token::Add {
                    eval_last_op(&mut stack);
                }
            },
            _ => {},
        }
    }

    while stack.len() > 1 {
        eval_last_op(&mut stack);
    }

    if let Token::Num(result) = stack[0] {
        result
    } else {
        panic!("no result");
    }
}

fn second_part() -> i64 {
    let file = fs::File::open("input.txt").expect("Could not open the input");

    let lines = io::BufReader::new(file).lines();
    let mut res = 0i64;
    for line in lines {
        if let Ok(line) = line {
            let line = line.replace("(", "( ").replace(")", " )");
            let line_res = eval_line(&line);
            res += line_res;
        }
    }

    res
}

fn main() {
    let first = first_part();
    println!("Sum of results 1:\n{}", first);

    let second = second_part();
    println!("Sum of results 2:\n{}", second);
}
