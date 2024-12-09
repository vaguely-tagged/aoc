use std::fs;

fn main() {
    let words = fs::read_to_string("input.txt").expect("no file");

    let word_arr: Vec<Vec<char>> = words.lines().map(|x| x.chars().collect()).collect();

    let mut count = 0;

    for (y, word) in word_arr.iter().enumerate() {
        for (x, current_char) in word.iter().enumerate() {
            // if *current_char == 'X' {
            //     count += find_xmas(x, y, &word_arr);
            // }
            //
            if *current_char == 'A' {
                count += find_xmas_two(x, y, &word_arr);
            }
        }
    }

    println!("count is {count:?}");
}
//this might be the worst code I've ever written
fn find_xmas_two(x: usize, y: usize, word_arr: &Vec<Vec<char>>) -> u32 {
    let y: i32 = y as i32;
    let x: i32 = x as i32;

    if x - 1 < 0 || y - 1 < 0 {
        return 0;
    }

    let mut left_right = vec!['M', 'S'];
    let top_left = (-1 + x, y + 1);
    let bottom_right = (1 + x, y + -1);

    let mut right_left = vec!['M', 'S'];
    let top_right = (1 + x, y + 1);
    let bottom_left = (-1 + x, y + -1);

    if let Some(row) = word_arr.get(top_left.1 as usize) {
        if let Some(character) = row.get(top_left.0 as usize) {
            if *character == 'M' {
                left_right.remove(0);
            } else if *character == 'S' {
                left_right.remove(1);
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }

    if let Some(row) = word_arr.get(bottom_right.1 as usize) {
        if let Some(character) = row.get(bottom_right.0 as usize) {
            if *character != *left_right.get(0).unwrap() {
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    //others
    if let Some(row) = word_arr.get(top_right.1 as usize) {
        if let Some(character) = row.get(top_right.0 as usize) {
            if *character == 'M' {
                right_left.remove(0);
            } else if *character == 'S' {
                right_left.remove(1);
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }

    if let Some(row) = word_arr.get(bottom_left.1 as usize) {
        if let Some(character) = row.get(bottom_left.0 as usize) {
            if *character != *right_left.get(0).unwrap() {
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }

    1
}

fn find_xmas(x: usize, y: usize, word_arr: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    let y: i32 = y as i32;
    let x: i32 = x as i32;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            let m_x_index = dx + x;
            let m_y_index = dy + y;

            let a_x_index = x + 2 * dx;
            let a_y_index = y + 2 * dy;

            let s_x_index = x + 3 * dx;
            let s_y_index = y + 3 * dy;

            if m_x_index < 0
                || m_y_index < 0
                || a_x_index < 0
                || a_y_index < 0
                || s_x_index < 0
                || s_y_index < 0
            {
                continue;
            }

            let mut letter_count = 0;

            let letters = vec!['M', 'A', 'S'];
            let coordinates = [
                (m_x_index, m_y_index),
                (a_x_index, a_y_index),
                (s_x_index, s_y_index),
            ];

            for (index, (char_x, char_y)) in coordinates.iter().enumerate() {
                if let Some(row) = word_arr.get(*char_y as usize) {
                    if let Some(character) = row.get(*char_x as usize) {
                        if character == letters.get(index).unwrap() {
                            letter_count += 1;
                        }
                    }
                }
            }

            if letter_count == 3 {
                count += 1;
            }
        }
    }

    count
}
