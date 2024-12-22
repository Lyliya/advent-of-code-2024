use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn mix(num: usize, a: usize) -> usize {
    num ^ a
}

fn get_next_secret_number(num: usize) -> usize {
    let mut res = mix(num, num * 64) % 16777216;
    res = mix(res, res / 32) % 16777216;
    res = mix(res, res * 2048) % 16777216;

    res
}

fn find_n_secret_number(num: usize, steps: usize, cost: &mut HashMap<[isize; 4], usize>) {
    let mut sequence = [0; 4];
    let mut exist: HashSet<[isize; 4]> = HashSet::new();

    let mut t = num;
    for step in 0..steps {
        let tmp = t;
        t = get_next_secret_number(t);

        sequence.rotate_left(1);
        sequence[3] = (tmp % 10) as isize - (t % 10) as isize;
        if step >= 3 {
            if exist.insert(sequence) {
                cost.entry(sequence)
                    .and_modify(|e| *e += t % 10)
                    .or_insert(t % 10);
            }
        }
    }
}

fn step2(input: &String) {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Expect number"))
        .collect();

    let mut cost = HashMap::new();
    for line in lines {
        find_n_secret_number(line, 2000, &mut cost);
    }

    println!(
        "Step 2 : {}",
        cost.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
    );
}

fn step1(input: &String) {
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Expect number"))
        .collect();

    let mut answer = 0;

    for line in lines {
        let mut t = line;
        for _i in 0..2000 {
            t = get_next_secret_number(t);
        }
        answer += t;
    }

    println!("Step 1 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
