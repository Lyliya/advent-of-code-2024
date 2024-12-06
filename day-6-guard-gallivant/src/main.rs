use std::collections::HashSet;
use std::fs;

fn compute_path(grid: &Vec<Vec<u8>>) -> Result<HashSet<(usize, usize)>, ()> {
    let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_move: HashSet<(usize, usize, i32)> = HashSet::new();
    let y_len = grid.len();
    let x_len = grid[0].len();

    let (mut gy, mut gx) = (0..y_len)
        .flat_map(|y| (0..x_len).map(move |x| (y, x)))
        .find(|(y, x)| b"^>v<".contains(&grid[*y][*x]))
        .expect("Unable to find guard position");

    let mut current_direction = match grid[gy][gx] {
        b'^' => 0,
        b'>' => 1,
        b'v' => 2,
        _ => 3,
    };

    visited_pos.insert((gy, gx));
    visited_move.insert((gy, gx, current_direction));

    while (gy > 0 && current_direction == 0)
        || (gx < x_len - 1 && current_direction == 1)
        || (gy < y_len - 1 && current_direction == 2)
        || (gx > 0 && current_direction == 3)
    {
        let (new_y, new_x) = match current_direction {
            0 => (gy - 1, gx),
            1 => (gy, gx + 1),
            2 => (gy + 1, gx),
            _ => (gy, gx - 1),
        };

        if grid[new_y][new_x] == b'#' {
            current_direction = (current_direction + 1) % 4;
        } else {
            if !visited_move.insert((new_y, new_x, current_direction)) {
                return Err(());
            }
            (gy, gx) = (new_y, new_x);
            visited_pos.insert((gy, gx));
        }
    }

    Ok(visited_pos)
}

fn step1(input: &str) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    println!(
        "Step 1 : {}",
        compute_path(&grid).expect("Input is looping").len()
    );
}

fn step2(input: &str) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let (gy, gx) = (0..grid.len())
        .flat_map(|y| (0..grid[0].len()).map(move |x| (y, x)))
        .find(|(y, x)| b"^>v<".contains(&grid[*y][*x]))
        .expect("Unable to find guard position");

    let path = compute_path(&grid).expect("Input is looping");
    let mut loop_count = 0;

    for (y, x) in path.into_iter() {
        if y != gy || x != gx {
            let mut new_grid = grid.clone();
            new_grid[y][x] = b'#';
            match compute_path(&new_grid) {
                Err(()) => {
                    loop_count += 1;
                }
                _ => (),
            };
        }
    }

    println!("Step 2 : {}", loop_count);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
