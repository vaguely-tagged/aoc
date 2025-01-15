use std::{collections::HashSet, fs};

const DOWN: (i32, i32) = (1 , 0);
const UP: (i32, i32) = (-1 , 0);
const LEFT: (i32, i32) = (0 , -1);
const RIGHT: (i32, i32) = (0 , 1);

fn main() {
    
    let grid = fs::read_to_string("input.txt")
        .expect("could not unwrap");

    let grid_size = grid.len() as i32;

    let grid: Vec<Vec<char>> = grid.lines()
    .map(|x|{
        x.chars().collect()
    }).collect();


    let mut starting: Option<(i32, i32)> = None;
    for (y, line) in grid.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if *item == '^' {
                starting = Some((y as i32, x as i32));
                break;
            }  
        }
    }

    let mut loop_count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {

            if *item == '.' {

                let mut temp_grid = grid.clone();
                temp_grid[y][x] = '#';

                if visit_loop(&mut temp_grid, starting.unwrap(), grid_size){
                    loop_count += 1;
                }
            }  
        }
    }

    println!("loop count is {loop_count:?}");



    // let visited = visit(&mut grid, starting.unwrap());
    // for line in grid {
    //     println!("{line:?}");
    // }

    //println!("total is {:?}",visited.len());

    // let mut loops = 0;
    // let mut already_visited = HashSet::new();

    // for (y, line) in grid.iter().enumerate() {
    //     for (x, item) in line.iter().enumerate() {
    //         if *item == '.' && !already_visited.contains(&(x,y)){
    //             already_visited.insert((x,y));

    //             let grid_clone = grid.clon
    //             break;
    //         }  
    //     }
    // }

}

fn visit_loop(grid: &mut Vec<Vec<char>>, starting: (i32, i32), grid_size: i32) -> bool{
    let mut visited  = HashSet::new();

    let mut curr_direction = UP;
    let mut curr_coords = starting;

    visited.insert((curr_coords.0, curr_coords.1));

    let mut count = 0;
    loop {

        count += 1;

        let mut y = curr_coords.0;
        let mut x = curr_coords.1;

        y += curr_direction.0;
        x += curr_direction.1;

        let y = y as usize;
        let x = x as usize;

        if count >= grid_size {
            return true;
        }

        if let Some(row) = grid.get_mut(y){
            if let Some(new_position) = row.get_mut(x){

                if *new_position == '#'{
                    curr_direction = match curr_direction {
                        UP => RIGHT,
                        RIGHT => DOWN,
                        DOWN => LEFT,
                        LEFT => UP,
                        _ => panic!("invalid")
                    };
                } else {
                    *new_position = 'X';
                    visited.insert((y as i32,x as i32));
                    curr_coords.0 = y as i32;
                    curr_coords.1 = x as i32;
                }
            } else {break}
        } else {break}
    }

    false
}

fn visit(grid: &mut Vec<Vec<char>>, starting: (i32, i32)) -> HashSet<(i32, i32)>{
    let mut visited  = HashSet::new();

    let mut curr_direction = UP;
    let mut curr_coords = starting;

    visited.insert((curr_coords.0, curr_coords.1));

    loop {

        let mut y = curr_coords.0;
        let mut x = curr_coords.1;

        y += curr_direction.0;
        x += curr_direction.1;

        let y = y as usize;
        let x = x as usize;

        if let Some(row) = grid.get_mut(y){
            if let Some(new_position) = row.get_mut(x){

                if *new_position == '#'{
                    curr_direction = match curr_direction {
                        UP => RIGHT,
                        RIGHT => DOWN,
                        DOWN => LEFT,
                        LEFT => UP,
                        _ => panic!("invalid")
                    };
                } else {
                    *new_position = 'X';
                    visited.insert((y as i32,x as i32));
                    curr_coords.0 = y as i32;
                    curr_coords.1 = x as i32;
                }
            } else {break}
        } else {break}
    }
    visited
}