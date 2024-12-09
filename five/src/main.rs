use std::{collections::HashMap, fs};

fn main() {
    let lines = fs::read_to_string("input.txt").expect("could not find input");
    let mut lines = lines.lines().collect();

    let results = order_pages(&lines);

    let updates = lines.split_off(results.1 + 1);
    let rules = results.0;

    let mut total = 0;
    for update in updates {
        let update: Vec<usize> = update
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if check_update(&update, &rules) {
            let half = update.get(update.len() / 2).unwrap();
            total += half;
        }
    }

    println!("the total is {total:?}");
}

fn check_update(update: &Vec<usize>, rules: &HashMap<usize, usize>) -> bool {
    let mut update_map = HashMap::new();

    for (index, entry) in update.iter().enumerate() {
        update_map.insert(entry, index);
    }

    for rule in rules {
        if let Some(key_index) = update_map.get(rule.0) {
            if let Some(value_index) = update_map.get(rule.1) {
                if value_index < key_index {
                    return false;
                }
            }
        }
    }
    true
}

fn order_pages(lines: &Vec<&str>) -> (HashMap<usize, usize>, usize) {
    let mut order = HashMap::new();
    for (count, line) in lines.iter().enumerate() {
        let line = *line;

        if line == "" {
            return (order, count);
        }

        let rule: Vec<&str> = line.split("|").collect();

        let key: usize = rule.get(0).unwrap().parse().unwrap();
        let value: usize = rule.get(1).unwrap().parse().unwrap();
        order.insert(key, value);
    }
    (order, 0)
}
