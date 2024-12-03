use std::fs;
use regex::Regex;

fn compute_mul(input: &String) -> usize {
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let matches = regex.captures_iter(&input);
    let mut answer = 0;
    for mul in matches {
        answer += &mul["a"].parse::<usize>().expect("Expect number") * &mul["b"].parse::<usize>().expect("Expect number");
    }
    answer
}

fn step2(input: &String) {
    let re = Regex::new(r"(?im)(don't\(\)|do\(\))|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let matches = re.captures_iter(&input);
    let mut enable: bool = true;
    let mut answer = 0;

    for mul in matches {
        if &mul[0] == "don't()" {
            enable = false;
        } else if &mul[0] == "do()" {
            enable = true;
        } else if enable {
            answer += &mul["a"].parse::<usize>().expect("Expect number") * &mul["b"].parse::<usize>().expect("Expect number");
        }
    }
    println!("Step 2 : {}", answer);
}

fn step1(input: &String) {
    let answer = compute_mul(input);
    println!("Step 1 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
