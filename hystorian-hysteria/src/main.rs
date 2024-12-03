use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in handle.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            let left_num: i32 = numbers[0].parse().unwrap();
            let right_num: i32 = numbers[1].parse().unwrap();
            left_list.push(left_num);
            right_list.push(right_num);
        }
    }

    left_list.sort();
    right_list.sort();

    let total_distance = calculate_total_distance(&left_list, &right_list);
    println!("Total distance: {}", total_distance);
}

fn calculate_total_distance(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut total_distance = 0;

    for (left, right) in left_list.iter().zip(right_list.iter()) {
        let distance = (left - right).abs(); 
        total_distance += distance; 
    }

    total_distance
}
