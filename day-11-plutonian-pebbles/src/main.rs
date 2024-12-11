use std::{collections::HashMap, fs};

fn split_number(num: usize) -> (usize, usize) {
    let len = num.ilog10() + 1;

    (num % 10usize.pow(len/2), num / 10usize.pow(len/2))
}


fn blink(num: Vec<usize>) -> Vec<usize> {
    let mut next: Vec<usize> = vec![];

    for n in num {
        if n == 0 {
            next.push(1);
        } else {
            let n_str = n.to_string();
            if n_str.len() % 2 == 0 {
                let (left, right) = split_number(n);
                next.push(left);
                next.push(right);
            } else {
                next.push(n * 2024);
            }
        }
    }

    next
}

fn blink_but_better(num: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut stones = HashMap::with_capacity(num.len());

    for (&key, &value) in num {
        if key == 0 {
            *stones.entry(1).or_default() += value;
        } else {
            let len = key.ilog10() + 1;
            if len % 2 == 0 {
                let (left, right) = split_number(key);
                *stones.entry(left).or_default() += value;
                *stones.entry(right).or_default() += value;
            } else {
                *stones.entry(key * 2024).or_default() += value;
            }

        }
    }

    stones
}

fn step1(input: &String) {
    let mut num: Vec<usize> = input.trim().split_whitespace().map(|n| n.parse::<usize>().expect("Expect number")).collect();
    for _i in 0..25 {
        num = blink(num);
    }

    println!("Step 1 : {}", num.len());
}

fn step2(input: &String) {
    let mut num: HashMap<usize, usize> = input.trim().split_whitespace().map(|n| (n.parse::<usize>().expect("Expect number"), 1)).collect();

    for _i in 0..75 {
        num = blink_but_better(&num);
    }

    println!("Step 2 : {}", num.values().sum::<usize>());
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}