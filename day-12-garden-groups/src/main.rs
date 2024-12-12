use std::{collections::HashSet, fs};

fn check_area(grid: &Vec<Vec<u8>>, pos: (usize, usize), checked: &mut HashSet<(usize, usize)>, area: &mut Vec<(usize, usize)>) {
    let y_len = grid.len();
    let x_len = grid[0].len();

    if !checked.insert(pos) {
        return;
    }

    let dir = [
        pos.0 > 0 && grid[pos.0 - 1][pos.1] == grid[pos.0][pos.1],
        pos.0 < y_len - 1 && grid[pos.0 + 1][pos.1] == grid[pos.0][pos.1],
        pos.1 > 0 && grid[pos.0][pos.1 - 1] == grid[pos.0][pos.1],
        pos.1 < x_len - 1 && grid[pos.0][pos.1 + 1] == grid[pos.0][pos.1],
    ];

    let mut perimeter = 4;
    for d in dir {
        if d {
            perimeter -= 1;
        }
    }


    let last = area.last_mut().unwrap();
    last.0 += 1;
    last.1 += perimeter;


    // Check up
    if dir[0] {
        check_area(grid, (pos.0 - 1, pos.1), checked, area);
    }
    // Check down
    if dir[1] {
        check_area(grid, (pos.0 + 1, pos.1), checked, area);
    }
    // Check left
    if dir[2] {
        check_area(grid, (pos.0, pos.1 - 1), checked, area);
    }
    // Check right
    if dir[3] {
        check_area(grid, (pos.0, pos.1 + 1), checked, area);
    }
}

fn step1(input: &String) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let y_len = grid.len();
    let x_len = grid[0].len();

    let mut area: Vec<(usize, usize)> = vec![];
    let mut checked: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..y_len {
        for x in 0..x_len {
            if !checked.contains(&(y, x)) {
                area.push((0, 0));
                check_area(&grid, (y, x), &mut checked, &mut area);
            }
        }
    }

    let mut total = 0;
    for value in area.clone().into_iter() {
        total += value.0 * value.1;
    }
    println!("Step 1 : {}", total);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    // step2(&input);
}