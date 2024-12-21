use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not find input");

    let words: Vec<&str> = input
        .split("\n\n")
        .collect();
    
    let rules: Vec<(u32, u32)> = words[0]
        .split("\n")
        .map(|x| {
            let temp: Vec<u32> = x
                .split("|")
                .map(|y| y.parse::<u32>().unwrap())
                .collect();
            (temp[0], temp[1])
        })
        .collect();

    let mut reports: Vec<Vec<u32>> = words[1]
        .split("\n")
        .map(|x| {
            x.split(",").map(|x| x.parse::<u32>().unwrap())
            .collect()
        })
        .collect();

   
    // let total = part_one(reports, rules);
    // println!("total is {total:?}");

    let total = part_two(&mut reports, rules);
    println!("total is {total:?}");
}

fn check_rules(rules: Vec<(u32, u32)>, report: &mut Vec<u32>) -> (bool, Vec<u32>){

    let did_pass = true;

    for rule in rules.iter(){

        let comes_before_index = match report.iter().position(|&r| r == rule.0){
            Some(comes_before_index) => comes_before_index,
            None => continue,
        };

        let comes_after_index = match report.iter().position(|&r| r == rule.1){
            Some(comes_after_index) => comes_after_index,
            None => continue,
        };

        if comes_before_index > comes_after_index {
            report.swap(comes_before_index,comes_after_index);
            return (false, report.to_vec())
        }
    }

    (did_pass, report.to_vec())
}

fn part_two(reports: &mut Vec<Vec<u32>>, rules: Vec<(u32, u32)>) -> u32{

    let mut total = 0;

    for report in reports {

        let mut counter = 0;
        let mut rule_check = check_rules(rules.clone(), report);

        while rule_check.0 == false {
            counter += 1;
            rule_check = check_rules(rules.clone(), &mut rule_check.1);
        }

        if counter > 0{
            total += rule_check.1.get(rule_check.1.len() / 2).unwrap();
        }
    }

    total
}


fn part_one(reports: Vec<Vec<u32>>, rules: Vec<(u32, u32)>) -> u32{

    let mut total = 0;

    for report in reports {

        let mut is_good = true;
        for rule in rules.iter(){


            let comes_before_index = match report.iter().position(|&r| r == rule.0){
                Some(comes_before_index) => comes_before_index,
                None => continue,
            };

            let comes_after_index = match report.iter().position(|&r| r == rule.1){
                Some(comes_after_index) => comes_after_index,
                None => continue,
            };

            if comes_before_index > comes_after_index {
                is_good = false;
            }
        };

        if is_good {
            total += report.get(report.len() / 2).unwrap();
        }
    }

    total
}
