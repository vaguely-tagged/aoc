use std::{collections::HashSet, f32::consts::PI, fs};

fn main() {
    let equations = fs::read_to_string("input.txt").expect("no file found!");
    let equations = equations.lines();

    let mut sum = 0;

    for equation in equations {
        let pair = parser(equation);

        let total = pair.0;
        let numbers = pair.1;

        let ops_count = numbers.len() - 1;
        if does_pass(total, &numbers, String::new(), ops_count) {
            sum += total;
        }
    }

    println!("total is {sum:?}");
}

fn parser(equation: &str) -> (u64, Vec<u64>) {
    let split: Vec<&str> = equation.split(": ").collect();

    let total = split.get(0).unwrap();
    let total = match total.parse() {
        Ok(total) => total,
        Err(error) => {
            eprintln!("error is {error:?}");
            println!("{split:?}");
            panic!("died");
        }
    };

    let numbers = split.get(1).unwrap();
    let numbers = numbers
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    (total, numbers)
}

fn does_pass(total: u64, numbers: &Vec<u64>, op_string: String, len: usize) -> bool {
    if check_total(&op_string, numbers, total) {
        return true;
    }

    if len == op_string.len() {
        return false;
    }

    for c in ['+', '*'] {
        if does_pass(
            total,
            numbers,
            (op_string.clone() + &c.to_string()).to_string(),
            len,
        ) {
            return true;
        }
    }

    false
}

fn check_total(operations: &str, numbers: &Vec<u64>, total: u64) -> bool {
    if operations.len() + 1 < numbers.len() {
        return false;
    }
    let curr_total = *numbers.first().unwrap();
    let mut curr_total: u64 = curr_total as u64;

    let operations: Vec<char> = operations.chars().collect();

    for (n, num) in numbers.iter().skip(1).enumerate() {
        let num = num.clone() as u64;

        match operations.get(n).unwrap() {
            '+' => curr_total += num,
            '*' => curr_total *= num,
            _ => panic!("invalid op"),
        }
    }

    if total as u64 == curr_total {
        return true;
    }

    false
}
