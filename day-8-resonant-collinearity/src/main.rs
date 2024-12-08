use std::collections::{HashMap, HashSet};
use std::fs;

fn get_symmetry(a: &(usize, usize), b: &(usize, usize)) -> Result<(usize, usize), ()> {
    let y = 2 * a.0 as isize - b.0 as isize;
    let x = 2 * a.1 as isize - b.1 as isize;

    if y < 0 || x < 0 {
        return Err(());
    }

    return Ok((y as usize, x as usize));
}

fn get_symmetry_recursive(
    a: &(usize, usize),
    b: &(usize, usize),
    len: (usize, usize),
    antinodes: &mut HashSet<(usize, usize)>,
) {
    antinodes.insert(*a);
    let y = 2 * a.0 as isize - b.0 as isize;
    let x = 2 * a.1 as isize - b.1 as isize;

    if y < 0 || x < 0 || y as usize >= len.0 || x as usize >= len.1 {
        return;
    }

    let (y, x) = (y as usize, x as usize);
    get_symmetry_recursive(&(y, x), a, len, antinodes);
}

fn step2(input: &String) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in grid.clone().into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if c != b'.' {
                let positions = antennas.entry(c).or_insert(vec![(y, x)]);
                if !positions.contains(&(y, x)) {
                    positions.push((y, x));
                }
            }
        }
    }

    for (_, set) in &antennas {
        for (i, coord) in set.into_iter().enumerate() {
            for other in 0..set.len() {
                if other != i {
                    get_symmetry_recursive(coord, &set[other], (y_len, x_len), &mut antinodes);
                }
            }
        }
    }

    println!("Step 2 : {:?}", antinodes.len());
}

fn step1(input: &String) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let y_len = grid.len();
    let x_len = grid[0].len();
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if c != b'.' {
                let positions = antennas.entry(c).or_insert(vec![(y, x)]);
                if !positions.contains(&(y, x)) {
                    positions.push((y, x));
                }
            }
        }
    }

    for (_, set) in &antennas {
        for (i, coord) in set.into_iter().enumerate() {
            for other in 0..set.len() {
                if other != i {
                    match get_symmetry(coord, &set[other]) {
                        Ok(val) => {
                            if val.0 < y_len && val.1 < x_len {
                                antinodes.insert(val);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }

    println!("Step 1 : {}", antinodes.len());
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
