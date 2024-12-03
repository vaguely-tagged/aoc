use std::fs;
use regex::Regex;

fn main() {
    let mut total = 0;

    let contents = fs::read_to_string("input.txt")
        .unwrap();

    let indicies: Vec<_> = contents.match_indices("mul(").collect();

    let mut last_index = 0;
    let mut state = "enabled";

    for index in indicies{


        let index = index.0;

        let prev_string = &contents[last_index..index];
        last_index = index;
        

        println!("prev string is {prev_string:?}");
        let dont_index = prev_string.rfind("don't()");

        if dont_index.is_some(){
            state = "disabled";
        }

        let do_index = prev_string.rfind("do()");

        if do_index.is_some() {

            if dont_index.is_some() && dont_index.unwrap() < do_index.unwrap(){
                state = "enabled";
            } else{
                state = "enabled"
            }


        }

        if state == "disabled"{
            continue;
        }



        let slice_chars = &contents[index..contents.len()].chars();

        let mut closing_index = None;
        for (closing_index_itter, char) in slice_chars.clone().into_iter().enumerate() {
            if char == ')'{
                closing_index = Some(closing_index_itter + index + 1);
                break;
            }
        }

        if closing_index.is_none(){
            continue;
        }

        let closing_index = closing_index.unwrap();
        let mult_string = &contents[index..closing_index];

        let regex = Regex::new(r"^\s*mul\(\d{1,3},\d{1,3}\)$").unwrap();

        if !regex.is_match(mult_string){
            println!("mult failed string is {mult_string:?}");
            continue;
        }

        let numbers: Vec<&str> = mult_string.split(",").collect();

        let first_number = numbers.get(0).unwrap();
        let first_number: i32 = (&first_number[4..first_number.len()]).parse().unwrap();

        let second_number = numbers.get(1).unwrap();
        let closing_index = second_number.find(")").unwrap();
        let second_number: i32 = (&second_number[0..closing_index]).parse().unwrap();

       let multiple = first_number * second_number;
       total += multiple;
    }

    println!("total is {total:?}");

}
