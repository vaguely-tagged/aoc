use std::fs;
use std::collections::HashMap;

fn main() {
    
    let input = fs::read_to_string("input.txt")
        .expect("could not read file");

    let mut first_column = Vec::new();
    let mut seccond_column = Vec::new();

    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.truncate(lines.len() - 1);

    for line in lines{

        let columns: Vec<i64> = line.split("   ")
            .map(|x| x.parse().unwrap())
            .collect();

        first_column.push(columns.get(0).unwrap().clone());
        seccond_column.push(columns.get(1).unwrap().clone());
    }

    // first_column.sort();
    // seccond_column.sort();

    // let mut total = 0;
    // for i in 0..first_column.len(){

    //     let first_item = first_column.get(i).unwrap();
    //     let seccond_item = seccond_column.get(i).unwrap();

    //     if first_item < seccond_item{
    //         let diff = seccond_item - first_item;
    //         total += diff;
    //     } else{
    //         let diff = first_item - seccond_item;
    //         total += diff;
    //     }

        
    // }

    // println!("total is {total:?}");

    let mut seccond_list = HashMap::new();

    for num in seccond_column{

        let mut insert_value = 1;

        if seccond_list.contains_key(&num){
            let old_val = seccond_list.get(&num).unwrap();
            insert_value += old_val;
        }

        seccond_list.insert(num, insert_value);
    }

    let mut total = 0;
    for num in first_column{

        let mut multiple = 0;

        if seccond_list.contains_key(&num){
            multiple = seccond_list.get(&num).unwrap().clone();
        }

        total += num * multiple;
    }

    println!("total is {total:?}");
}
