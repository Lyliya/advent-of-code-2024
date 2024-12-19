use std::{collections::HashMap, fs};

fn parse_input(input: &String) -> (Vec<String>, Vec<String>) {
    let bindings = input.replace("\r\n", "\n");
    let first_split: Vec<_> = bindings.split("\n\n").collect();

    let available_towel_str = first_split[0];
    let mut available_towels = vec![];

    for towel in available_towel_str.split(", ") {
        available_towels.push(String::from(towel));
    }

    let patterns_str = first_split[1];
    let mut patterns = vec![];
    for pattern in patterns_str.lines() {
        patterns.push(String::from(pattern));
    }

    (available_towels, patterns)
}

fn is_pattern_possible(towels: &Vec<String>, pattern: &String, index: usize, cache: &mut HashMap<usize, bool>) -> bool {
    if index == pattern.len() {
        return true; // Done
    }

    if let Some(&result) = cache.get(&index) {
        return result;
    }

    for towel in towels {
        if pattern[index..].starts_with(towel) {
                if is_pattern_possible(towels, pattern, index + towel.len(), cache) {
                    cache.insert(index, true);
                    return true;
                }
        }
    }

    cache.insert(index, false);
    false
}

fn step1(input: &String) {
    let (available_towels, patterns) = parse_input(input);

    let mut answer = 0;
    for pattern in patterns {
        let mut cache: HashMap<usize, bool> = HashMap::new();

        let is_possible = is_pattern_possible(&available_towels, &pattern, 0, &mut cache);
        if is_possible {
            answer += 1;
        } else {
        }
    }

    println!("Step 1 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
}
