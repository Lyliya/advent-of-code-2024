use std::{collections::HashSet, fs};

fn check_pos_uniq(grid: &Vec<Vec<u8>>, pos: (usize, usize), end: &mut HashSet<(usize, usize)>) {
    let y_len = grid.len();
    let x_len = grid[0].len();

    if grid[pos.0][pos.1] == b'9' {
        end.insert(pos);
        return;
    }

    // Check up
    if pos.0 > 0 && grid[pos.0 - 1][pos.1] == grid[pos.0][pos.1] + 1 {
        check_pos_uniq(grid, (pos.0 - 1, pos.1), end);
    }
    // Check down
    if pos.0 < y_len - 1 && grid[pos.0 + 1][pos.1] == grid[pos.0][pos.1] + 1 {
        check_pos_uniq(grid, (pos.0 + 1, pos.1), end);
    }

    // Check left
    if pos.1 > 0 && grid[pos.0][pos.1 - 1] == grid[pos.0][pos.1] + 1 {
        check_pos_uniq(grid, (pos.0, pos.1 - 1), end);
    }
    // Check right
    if pos.1 < x_len - 1 && grid[pos.0][pos.1 + 1] == grid[pos.0][pos.1] + 1 {
        check_pos_uniq(grid, (pos.0, pos.1 + 1), end);
    }
}

fn step1(input: &String) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut answer = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == b'0' {
                let mut end: HashSet<(usize, usize)> = HashSet::new();
                check_pos_uniq(&grid, (y, x), &mut end);
                answer += end.len();
            }
        }
    }

    println!("Step 1 : {}", answer);
}

fn check_pos(grid: &Vec<Vec<u8>>, pos: (usize, usize), end: &mut Vec<(usize, usize)>) {
    let y_len = grid.len();
    let x_len = grid[0].len();

    if grid[pos.0][pos.1] == b'9' {
        end.push(pos);
        return;
    }

    // Check up
    if pos.0 > 0 && grid[pos.0 - 1][pos.1] == grid[pos.0][pos.1] + 1 {
        check_pos(grid, (pos.0 - 1, pos.1), end);
    }
    // Check down
    if pos.0 < y_len - 1 && grid[pos.0 + 1][pos.1] == grid[pos.0][pos.1] + 1 {
        check_pos(grid, (pos.0 + 1, pos.1), end);
    }

    // Check left
    if pos.1 > 0 && grid[pos.0][pos.1 - 1] == grid[pos.0][pos.1] + 1 {
        check_pos(grid, (pos.0, pos.1 - 1), end);
    }
    // Check right
    if pos.1 < x_len - 1 && grid[pos.0][pos.1 + 1] == grid[pos.0][pos.1] + 1 {
        check_pos(grid, (pos.0, pos.1 + 1), end);
    }
}

fn step2(input: &String) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut answer = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[y][x] == b'0' {
                let mut end: Vec<(usize, usize)> = vec![];
                check_pos(&grid, (y, x), &mut end);
                answer += end.len();
            }
        }
    }

    println!("Step 2 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}