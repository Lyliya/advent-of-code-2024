use std::fs;

fn generate_disk(input: &String) -> Vec<isize> {
    let mut disk: Vec<isize> = vec![];
    let mut id = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_digit(10).expect("Expect number");
        for _ in 0..n {
            disk.push(if i % 2 == 0 { id } else { -1 });
        }
        if i % 2 == 0 {
            id += 1
        }
    }
    disk
}

fn step1(input: &String) {
    let disk = generate_disk(input);
    let mut stop = disk.len() - 1;
    let mut it: usize = 0;
    let mut answer: usize = 0;

    while it <= stop {
        if disk[it] == -1 {
            while disk[stop] == -1 {
                stop -= 1;
            }
            let value: usize = disk[stop].try_into().unwrap();
            answer += it * value;
            stop -= 1;
        } else {
            let value: usize = disk[it].try_into().unwrap();
            answer += it * value;
        }
        it += 1;
    }
    println!("Step 1 : {}", answer);
}

fn generate_disk_block(input: &String) -> Vec<(usize, isize)> {
    let mut disk: Vec<(usize, isize)> = vec![];
    let mut id = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_digit(10).expect("Expect number");
        disk.push((n as usize, if i % 2 == 0 { id } else { -1 }));
        if i % 2 == 0 {
            id += 1
        }
    }
    disk
}

fn step2(input: &String) {
    let mut disk = generate_disk_block(input);
    let mut cursor = 0;

    while cursor < disk.len() {
        let (len, value) = disk[cursor];
        if value == -1 {
            let mut it = disk.len() - 1;
            while it > cursor {
                let (nl, nv) = disk[it];
                if nv != -1 {
                    if nl == len {
                        disk[it] = (nl, -1);
                        disk[cursor] = (nl, nv);
                        break;
                    } else if nl < len {
                        disk[it] = (nl, -1);
                        disk[cursor] = (nl, nv);
                        disk.insert(cursor + 1, (len - nl, -1));
                        break;
                    }
                }
                it -= 1;
            }
        }
        cursor += 1;
    }

    let mut answer = 0;
    let mut factor = 0;

    for (i, n) in disk {
        for _ in 0..i {
            if n == -1 {
            } else {
                answer += factor * n as usize;
            }
            factor += 1;
        }
    }

    println!("Step 2 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
