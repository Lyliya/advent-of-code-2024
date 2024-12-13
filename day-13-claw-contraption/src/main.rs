use std::fs;
use regex::Regex;

fn check_game(game: &str, add: isize) -> (usize, usize) {
    let button = Regex::new(r"Button (?<name>\w): X\+(?<x>\d*), Y\+(?<y>\d*)").unwrap();
    let matches: Vec<regex::Captures<'_>> = button.captures_iter(&game).collect();
    let a: (isize, isize) = (matches[0]["x"].parse::<isize>().expect("Expect number"), matches[0]["y"].parse::<isize>().expect("Expect number"));
    let b: (isize, isize) = (matches[1]["x"].parse::<isize>().expect("Expect number"), matches[1]["y"].parse::<isize>().expect("Expect number"));

    let prize = Regex::new(r"Prize: X=(?<x>\d*), Y=(?<y>\d*)").unwrap();
    let matches: Vec<regex::Captures<'_>> = prize.captures_iter(&game).collect();
    let total: (isize, isize) = (matches[0]["x"].parse::<isize>().expect("Expect number") + add, matches[0]["y"].parse::<isize>().expect("Expect number") + add);

    if a.0 == 0 || a.1 == 0 || b.0 == 0 || b.1 == 0 {
        return (0,0);
    }
    let b_count: isize = (a.1 * total.0 - a.0 * total.1) / (a.1 * b.0 - a.0 * b.1);
    let a_count: isize = (total.0 - b.0 * b_count) / a.0;
    
    if (a.0 * a_count + b.0 * b_count, a.1 * a_count + b.1 * b_count) == total {
        return (a_count as usize, b_count as usize)
    }
    return (0,0);
}

fn step1(input: &String) {
    let binding = input.replace("\r\n", "\n");
    let games: Vec<&str> = binding.split("\n\n").collect();
    let mut answer = 0;

    for game in games {
        let (a, b) = check_game(game, 0);
        if a < 100 && b < 100 {
            answer += a * 3 + b;
        }
    }

    println!("Step 1 : {}", answer);
}


fn step2(input: &String) {
    let binding = input.replace("\r\n", "\n");
    let games: Vec<&str> = binding.split("\n\n").collect();
    let mut answer = 0;

    for game in games {
        let (a, b) = check_game(game, 10000000000000);
        answer += a * 3 + b;
    }

    println!("Step 2 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}