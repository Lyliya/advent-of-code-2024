use core::str;
use std::fs;

fn check_diff(a: i32, b: i32, is_increasing: bool) -> bool {
    let diff: i32 = a - b;

    if (is_increasing && diff > 0)
        || (!is_increasing && diff < 0)
        || diff == 0
        || (diff.abs() < 1 || diff.abs() > 3)
    {
        return false;
    }
    return true;
}

fn is_safe_step1(num: &Vec<i32>) -> bool {
    let mut is_increasing: bool = false;

    for i in 0..num.len() - 1 {
        if i == 0 {
            is_increasing = num[i] - num[i + 1] < 0
        }
        if !check_diff(num[i], num[i + 1], is_increasing) {
            return false;
        }
    }
    return true;
}

fn copy_and_remove(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    if index >= vec.len() {
        panic!("OoB");
    }
    let mut new_vec: Vec<i32> = vec.clone();
    new_vec.remove(index);
    new_vec
}

fn is_safe_step2(num: &Vec<i32>) -> bool {
    if is_safe_step1(num) {
        return true;
    }

    for i in 0..num.len() {
        if is_safe_step1(&copy_and_remove(&num, i)) {
            return true;
        }
    }

    return false;
}

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split_whitespace()
        .map(|f: &str| f.parse::<i32>().expect("Expect number"))
        .collect()
}

fn step1(lines: str::Lines<'_>) {
    let mut answer: i32 = 0;
    for line in lines {
        let num: Vec<i32> = parse_line(line);

        if is_safe_step1(&num) {
            answer += 1;
        }
    }
    println!("Step 1: {}", answer);
}

fn step2(lines: str::Lines<'_>) {
    let mut answer: i32 = 0;
    for line in lines {
        let num: Vec<i32> = parse_line(line);

        if is_safe_step2(&num) {
            answer += 1;
        }
    }
    println!("Step 2: {}", answer);
}

fn main() {
    let input1 = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(input1.lines());
    step2(input1.lines());
}
