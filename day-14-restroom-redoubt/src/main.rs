use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct Guard {
    pos: (isize, isize),
    velocity: (isize, isize),
}

const X: isize = 101;
const Y: isize = 103;

fn parse_guard(input: &String) -> Vec<Guard> {
    let lines = input.trim().lines();
    let guard_reg = Regex::new(r"p=(?<px>-?\d*),(?<py>-?\d*) v=(?<vx>-?\d*),(?<vy>-?\d*)").unwrap();
    let mut guards: Vec<Guard> = vec![];

    for line in lines {
        let guard_info = guard_reg.captures(&line).unwrap();

        guards.push(Guard {
            pos: (
                guard_info[1].parse::<isize>().expect("Expect Number"),
                guard_info[2].parse::<isize>().expect("Expect Number"),
            ),
            velocity: (
                guard_info[3].parse::<isize>().expect("Expect Number"),
                guard_info[4].parse::<isize>().expect("Expect Number"),
            ),
        });
    }
    guards
}

fn step1(input: &String) {
    let mut guards: Vec<Guard> = parse_guard(input);

    for _i in 0..100 {
        for guard in &mut guards {
            let x = (guard.pos.0 + guard.velocity.0) % X;
            let y = (guard.pos.1 + guard.velocity.1) % Y;
            guard.pos = (if x < 0 { X + x } else { x }, if y < 0 { Y + y } else { y });
        }
    }

    let mut q1 = 0; // Top Left
    let mut q2 = 0; // Top Right
    let mut q3 = 0; // Bottom Left
    let mut q4 = 0; // Bottom Right

    for guard in guards.into_iter() {
        if guard.pos.0 < X / 2 {
            if guard.pos.1 < Y / 2 {
                q1 += 1;
            } else if guard.pos.1 > Y / 2 {
                q3 += 1;
            }
        }
        if guard.pos.0 > X / 2 {
            if guard.pos.1 < Y / 2 {
                q2 += 1;
            } else if guard.pos.1 > Y / 2 {
                q4 += 1;
            }
        }
    }

    println!("Step 1 : {}", q1 * q2 * q3 * q4);
}

fn display_map(guards: Vec<Guard>) {
    let mut map: Vec<Vec<u8>> = vec![];
    for _y in 0..Y {
        let mut line: Vec<u8> = vec![];
        for _x in 0..X {
            line.push(b'.');
        }
        map.push(line);
    }

    for guard in guards {
        map[guard.pos.1 as usize][guard.pos.0 as usize] = b'X';
    }

    for y in 0..Y {
        for x in 0..X {
            print!("{}", map[y as usize][x as usize] as char);
        }
        println!();
    }
}

fn step2(input: &String) {
    let mut guards: Vec<Guard> = parse_guard(input);
    let mut second = 0;

    let half = guards.len() / 2;
    let mut count = 0;

    // If more than half the guard are near the center, there is a high change it draw a Xmas tree
    while count < half {
        count = 0;
        for guard in &mut guards {
            let x = (guard.pos.0 + guard.velocity.0) % X;
            let y = (guard.pos.1 + guard.velocity.1) % Y;
            guard.pos = (if x < 0 { X + x } else { x }, if y < 0 { Y + y } else { y });
        }
        for guard in guards.clone().into_iter() {
            if guard.pos.0 >= X / 4
                && guard.pos.0 <= X * 3 / 4
                && guard.pos.1 >= Y / 4
                && guard.pos.1 <= Y * 3 / 4
            {
                count += 1;
            }
        }
        second += 1
    }

    display_map(guards);

    println!("Step 2 : {}", second);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
