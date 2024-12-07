use std::fs;

fn generate_permutations(
    n: usize,
    current: &mut String,
    results: &mut Vec<String>,
    ops: &Vec<char>,
) {
    if current.len() == n {
        results.push(current.clone());
        return;
    }

    for &ch in ops {
        current.push(ch);
        generate_permutations(n, current, results, ops);
        current.pop(); // Backtrack
    }
}

fn step1(input: &str) {
    let lines = input.lines();
    let mut answer = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let total = parts[0].trim().parse::<usize>().expect("Expect number");

        let nums: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Expect number"))
            .collect();

        let mut ops_perm = Vec::new();
        let mut current = String::new();
        let ops = vec!['*', '+'];

        generate_permutations(nums.len() - 1, &mut current, &mut ops_perm, &ops);

        for case in ops_perm {
            let mut t = nums[0];

            for (index, op) in case.chars().enumerate() {
                match op {
                    '*' => t *= nums[index + 1],
                    '+' => t += nums[index + 1],
                    _ => panic!("no handled"),
                }
            }
            if total == t {
                answer += total;
                break;
            }
        }
    }

    println!("Step 1 : {}", answer);
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

fn step2(input: &str) {
    let input = input.replace("||", "|");
    let lines = input.lines();
    let mut answer = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let total = parts[0].trim().parse::<usize>().expect("Expect number");

        let nums: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Expect number"))
            .collect();

        let mut ops_perm = Vec::new();
        let mut current = String::new();
        let ops = vec!['*', '+', '|'];

        generate_permutations(nums.len() - 1, &mut current, &mut ops_perm, &ops);

        for case in ops_perm {
            let mut t = nums[0];

            for (index, op) in case.chars().enumerate() {
                match op {
                    '*' => t *= nums[index + 1],
                    '+' => t += nums[index + 1],
                    '|' => t = concat(t, nums[index + 1]),
                    _ => panic!("no handled"),
                }
            }
            if total == t {
                answer += total;
                break;
            }
        }
    }

    println!("Step 1 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
