use std::{collections::HashSet, fs};

fn find_robot(grid: &Vec<Vec<u8>>) -> Result<(usize, usize), ()> {
    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if *c == b'@' {
                return Ok((y, x));
            }
        }
    }
    Err(())
}

fn check_next(grid: &mut Vec<Vec<u8>>, pos: (usize, usize), dir: char, robot: &mut (usize, usize)) {
    if grid[pos.0][pos.1] == b'#' {
        return;
    }
    if grid[pos.0][pos.1] == b'O' {
        match dir {
            '<' => check_next(grid, (pos.0, pos.1 - 1), dir, robot),
            '^' => check_next(grid, (pos.0 - 1, pos.1), dir, robot),
            'v' => check_next(grid, (pos.0 + 1, pos.1), dir, robot),
            '>' => check_next(grid, (pos.0, pos.1 + 1), dir, robot),
            _ => {}
        }
    }
    if grid[pos.0][pos.1] == b'.' {
        match dir {
            '<' => {
                if grid[pos.0][pos.1 + 1] == b'O' || grid[pos.0][pos.1 + 1] == b'@' {
                    if grid[pos.0][pos.1 + 1] == b'@' {
                        *robot = (pos.0, pos.1);
                    }
                    grid[pos.0][pos.1] = grid[pos.0][pos.1 + 1];
                    grid[pos.0][pos.1 + 1] = b'.';
                }
            }
            '^' => {
                if grid[pos.0 + 1][pos.1] == b'O' || grid[pos.0 + 1][pos.1] == b'@' {
                    if grid[pos.0 + 1][pos.1] == b'@' {
                        *robot = (pos.0, pos.1);
                    }
                    grid[pos.0][pos.1] = grid[pos.0 + 1][pos.1];
                    grid[pos.0 + 1][pos.1] = b'.';
                }
            }
            'v' => {
                if grid[pos.0 - 1][pos.1] == b'O' || grid[pos.0 - 1][pos.1] == b'@' {
                    if grid[pos.0 - 1][pos.1] == b'@' {
                        *robot = (pos.0, pos.1);
                    }
                    grid[pos.0][pos.1] = grid[pos.0 - 1][pos.1];
                    grid[pos.0 - 1][pos.1] = b'.';
                }
            }
            '>' => {
                if grid[pos.0][pos.1 - 1] == b'O' || grid[pos.0][pos.1 - 1] == b'@' {
                    if grid[pos.0][pos.1 - 1] == b'@' {
                        *robot = (pos.0, pos.1);
                    }
                    grid[pos.0][pos.1] = grid[pos.0][pos.1 - 1];
                    grid[pos.0][pos.1 - 1] = b'.';
                }
            }
            _ => {}
        }
    }
}

// fn display_map(grid: &Vec<Vec<u8>>) {
//     for line in grid {
//         for c in line {
//             print!("{}", *c as char);
//         }
//         println!();
//     }
//     println!();
// }

fn compute_score(grid: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;

    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if *c == b'O' {
                score += 100 * y + x;
            }
        }
    }
    score
}

fn compute_score2(grid: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;

    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if *c == b'[' {
                score += 100 * y + x;
            }
        }
    }
    score
}

fn step1(input: &String) {
    let binding = input.replace("\r\n", "\n");
    let s: Vec<_> = binding.split("\n\n").collect();

    if s.len() != 2 {
        panic!("Invalid input file")
    }

    let grid_txt = s[0];
    let mut grid: Vec<Vec<u8>> = grid_txt
        .lines()
        .map(|line| line.bytes().collect())
        .collect();
    let movements = s[1];

    let mut robot: (usize, usize) = find_robot(&grid).expect("Expect robot in input");

    for c in movements.chars() {
        match c {
            '<' => check_next(&mut grid, (robot.0, robot.1 - 1), c, &mut robot),
            '^' => check_next(&mut grid, (robot.0 - 1, robot.1), c, &mut robot),
            'v' => check_next(&mut grid, (robot.0 + 1, robot.1), c, &mut robot),
            '>' => check_next(&mut grid, (robot.0, robot.1 + 1), c, &mut robot),
            _ => {}
        }
    }

    println!("Step 1 : {}", compute_score(&grid));
}

fn add_dir(pos: (usize, usize), dir: char) -> (usize, usize) {
    match dir {
        '^' => (pos.0 - 1, pos.1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        '>' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid movement"),
    }
}

fn expand_frontier(
    grid: &Vec<Vec<u8>>,
    pos: (usize, usize),
    dir: char,
) -> Option<HashSet<(usize, usize)>> {
    let mut frontier = Vec::from([pos]);
    let mut all = HashSet::from([pos]);

    while let Some(curr) = frontier.pop() {
        let next = add_dir(curr, dir);
        if all.contains(&next) {
            continue;
        }
        let c = grid[next.0][next.1];
        match c {
            b'.' => {}
            b']' | b'[' => {
                frontier.push(next);
                all.insert(next);
                if dir == '^' || dir == 'v' {
                    let other = if c == b']' {
                        (next.0, next.1 - 1)
                    } else {
                        (next.0, next.1 + 1)
                    };
                    frontier.push(other);
                    all.insert(other);
                }
            }
            _ => return None,
        }
    }
    Some(all)
}

// Based on https://github.com/syncd010/AoC2024/blob/main/src/day15.rs answer. Did not manage to make it work on my own
fn step2(input: &String) {
    let binding = input
        .replace("\r\n", "\n")
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let s: Vec<_> = binding.split("\n\n").collect();

    if s.len() != 2 {
        panic!("Invalid input file")
    }

    let grid_txt = s[0];
    let mut grid: Vec<Vec<u8>> = grid_txt
        .lines()
        .map(|line| line.bytes().collect())
        .collect();
    let mut scratch_grid = grid.clone();
    let movements = s[1].replace("\n", "");

    let mut robot: (usize, usize) = find_robot(&grid).expect("Expect robot in input");

    for m in movements.chars() {
        if let Some(frontier) = expand_frontier(&grid, robot, m) {
            for p in frontier.iter() {
                scratch_grid[p.0][p.1] = grid[p.0][p.1];
            }
            for p in frontier.iter() {
                grid[p.0][p.1] = b'.';
            }
            for p in frontier.iter() {
                let new_p = add_dir(*p, m);
                grid[new_p.0][new_p.1] = scratch_grid[p.0][p.1];
            }
            robot = add_dir(robot, m);
        }
    }

    let answer = compute_score2(&grid);

    println!("Step 2 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
