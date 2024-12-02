use std::fs;
use std::thread;

fn main() {
    
    let input = fs::read_to_string("input.txt")
        .unwrap();

    let reports: Vec<&str> = input.split("\n").collect();

    let mut safe = 0;
    for report in reports{

        if is_report_safe_dampener(report){
            safe += 1;
        }
    }

    println!("there are {safe:?} safe reports");
}

fn is_report_safe_dampener(report: &str) -> bool{

    let levels: Vec<i32> = report
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    
    let did_pass = is_report_safe(&levels);
    if did_pass {
        return  true;
    }

    let mut joins = Vec::new();

    for _ in 0..levels.len(){

        let levels_clone = levels.clone();
        let len = levels_clone.len();

        let pass = thread::spawn(move || {

            
            for n in 0..len{
                let mut clone = levels_clone.clone();
                clone.remove(n);
                let res = is_report_safe(&clone);
                if res {
                    return true
                }
            }

            return false
        });

        joins.push(pass);
    }

    for join in joins{

        let res = join.join().unwrap();
        if res == true{
            return true
        }
    }

    false
}


fn is_report_safe(levels: &Vec<i32>) -> bool{


    let direction;
    let mut prev: i32;

    let first_num = levels.get(0).unwrap();
    let second_num = levels.get(1).unwrap();

    prev = first_num.clone();

    if first_num < second_num{
        direction = "increasing";
    } else{
        direction = "decreasing";
    }

    for (index,number) in levels.iter().enumerate().skip(1){

        let number = *number;

        //incorrect
        if number <= prev && direction == "increasing"{
            return false;
        }

        if number >= prev && direction == "decreasing"{
            return false;
        }


        if (number - prev).abs() < 1 || (number - prev).abs() > 3{
            return false;
        }

        prev = number.clone();
    }

    true
}