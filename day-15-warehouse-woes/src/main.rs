use std::fs;

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

fn check_next(grid: &mut Vec<Vec<u8>>, pos: (usize, usize), dir: char) {
    if grid[pos.0][pos.1] == b'#' {
        return;
    }
    if grid[pos.0][pos.1] == b'O' {
        match dir {
            '<' => check_next(grid, (pos.0, pos.1 - 1), dir),
            '^' => check_next(grid, (pos.0 - 1, pos.1), dir),
            'v' => check_next(grid, (pos.0 + 1, pos.1), dir),
            '>' => check_next(grid, (pos.0, pos.1 + 1), dir),
            _ => {}
        }
    }
    if grid[pos.0][pos.1] == b'.' {
        match dir {
            '<' => {
                if grid[pos.0][pos.1 + 1] == b'O' || grid[pos.0][pos.1 + 1] == b'@' {
                    grid[pos.0][pos.1] = grid[pos.0][pos.1 + 1];
                    grid[pos.0][pos.1 + 1] = b'.';
                }
            }
            '^' => {
                if grid[pos.0 + 1][pos.1] == b'O' || grid[pos.0 + 1][pos.1] == b'@' {
                    grid[pos.0][pos.1] = grid[pos.0 + 1][pos.1];
                    grid[pos.0 + 1][pos.1] = b'.';
                }
            }
            'v' => {
                if grid[pos.0 - 1][pos.1] == b'O' || grid[pos.0 - 1][pos.1] == b'@' {
                    grid[pos.0][pos.1] = grid[pos.0 - 1][pos.1];
                    grid[pos.0 - 1][pos.1] = b'.';
                }
            }
            '>' => {
                if grid[pos.0][pos.1 - 1] == b'O' || grid[pos.0][pos.1 - 1] == b'@' {
                    grid[pos.0][pos.1] = grid[pos.0][pos.1 - 1];
                    grid[pos.0][pos.1 - 1] = b'.';
                }
            }
            _ => {}
        }
    }
}

fn display_map(grid: &Vec<Vec<u8>>) {
    for line in grid {
        for c in line {
            print!("{}", *c as char);
        }
        println!();
    }
    println!();
}

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
            '<' => check_next(&mut grid, (robot.0, robot.1 - 1), c),
            '^' => check_next(&mut grid, (robot.0 - 1, robot.1), c),
            'v' => check_next(&mut grid, (robot.0 + 1, robot.1), c),
            '>' => check_next(&mut grid, (robot.0, robot.1 + 1), c),
            _ => {}
        }
        robot = find_robot(&grid).expect("Expect robot");
        // display_map(&grid);
    }

    println!("Step 1 : {}", compute_score(&grid));
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    // step2(&input);
}
