use std::{collections::HashSet, fs};

fn main() {
    let bindings = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let bindings = bindings.replace("\r\n", "\n");

    let grids = bindings.split("\n\n");

    let mut locks: HashSet<[i32; 5]> = HashSet::new();
    let mut keys: HashSet<[i32; 5]> = HashSet::new();

    for grid in grids {
        let mut split: Vec<&str> = grid.lines().collect();
        let is_key = split[0].contains(".");
        let mut item = [0; 5];

        if is_key {
            split.reverse();
        }

        for line in split.into_iter().skip(1) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    item[i] += 1;
                }
            }
        }

        if is_key {
            keys.insert(item);
        } else {
            locks.insert(item);
        }
    }

    let mut answer = 0;

    for lock in locks {
        for key in &keys {
            if !lock
                .into_iter()
                .enumerate()
                .any(|(i, value)| key[i] + value >= 6)
            {
                answer += 1;
            }
        }
    }

    println!("Answer: {}", answer);
}
