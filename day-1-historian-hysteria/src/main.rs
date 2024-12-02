use core::str;
use std::fs;

fn step1(lines: str::Lines<'_>) {
    let mut left_vec: Vec<usize> = Vec::new();
    let mut right_vec: Vec<usize> = Vec::new();

    for line in lines {
        let parts = line.trim().split_whitespace();
        for (i, num) in parts.into_iter().enumerate() {
            if i % 2 == 0 {
                left_vec.push(num.parse().expect("Expect number"));
            } else {
                right_vec.push(num.parse().expect("Expect number"));
            }
        }
    }

    left_vec.sort();
    right_vec.sort();

    if left_vec.len() != right_vec.len() {
        panic!("Invalid input file");
    }

    let mut answer = 0;

    for i in 0..left_vec.len() {
        if left_vec[i] > right_vec[i] {
            answer += left_vec[i] - right_vec[i];
        } else if left_vec[i] < right_vec[i] {
            answer += right_vec[i] - left_vec[i];
        }
    }

    println!("Step 1 anwser: {}", answer);
}

fn step2(lines: str::Lines<'_>) {
    let mut left_vec: Vec<usize> = Vec::new();
    let mut right_vec: Vec<usize> = Vec::new();

    for line in lines {
        let parts = line.trim().split_whitespace();
        for (i, num) in parts.into_iter().enumerate() {
            if i % 2 == 0 {
                left_vec.push(num.parse().expect("Expect number"));
            } else {
                right_vec.push(num.parse().expect("Expect number"));
            }
        }
    }

    left_vec.sort();
    right_vec.sort();

    if left_vec.len() != right_vec.len() {
        panic!("Invalid input file");
    }

    let mut answer = 0;

    for i in 0..left_vec.len() {
        answer += left_vec[i] * right_vec.iter().filter(|&n| *n == left_vec[i]).count();
    }

    println!("Step 2 anwser: {}", answer);
}

fn main() {
    let input1 = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(input1.lines());
    step2(input1.lines());
}
