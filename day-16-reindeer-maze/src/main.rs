use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    y: usize,
    x: usize,
    dir: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Directions;

impl Directions {
    const RIGHT: (isize, isize) = (0, 1);
    const DOWN: (isize, isize) = (1, 0);
    const LEFT: (isize, isize) = (0, -1);
    const UP: (isize, isize) = (-1, 0);
}

// Right Down Left Up
const DIRECTIONS: [(isize, isize); 4] = [
    Directions::LEFT,
    Directions::UP,
    Directions::RIGHT,
    Directions::DOWN,
];

fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = (i, j);
            } else if *c == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end)
}

fn dijkstra(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, Vec<((usize, usize), (isize, isize))>, usize)> {
    let mut heap = BinaryHeap::from([State {
        cost: 0,
        y: start.0,
        x: start.1,
        dir: Directions::RIGHT,
    }]);
    let mut distances = HashMap::new();
    let mut came_from: HashMap<
        ((usize, usize), (isize, isize)),
        Vec<((usize, usize), (isize, isize))>,
    > = HashMap::new();
    let mut copy_grid = grid.clone();

    distances.insert((start, Directions::RIGHT), 0);

    while let Some(State {
        cost,
        y,
        x,
        dir: current_dir,
    }) = heap.pop()
    {
        if let Some(&old) = distances.get(&((y, x), current_dir)) {
            if cost > old {
                continue;
            }
        }

        if (y, x) == end {
            let mut stack = vec![((y, x), current_dir)];
            let mut unique_pos = HashSet::new();

            while let Some(pos) = stack.pop() {
                unique_pos.insert((pos.0 .0, pos.0 .1));
                copy_grid[pos.0 .0][pos.0 .1] = 'O';
                if let Some(prev) = came_from.get(&pos) {
                    stack.extend(prev.iter().copied());
                }
            }

            display_map(&copy_grid);
            return Some((cost, stack, unique_pos.len()));
        }

        for dir in DIRECTIONS.into_iter() {
            let new_y = y as isize + dir.0;
            let new_x = x as isize + dir.1;

            if grid[new_y as usize][new_x as usize] != '#' {
                let new_cost = match current_dir {
                    Directions::RIGHT | Directions::LEFT => {
                        if dir == Directions::UP || dir == Directions::DOWN {
                            cost + 1001
                        } else {
                            cost + 1
                        }
                    }
                    Directions::UP | Directions::DOWN => {
                        if dir == Directions::LEFT || dir == Directions::RIGHT {
                            cost + 1001
                        } else {
                            cost + 1
                        }
                    }
                    _ => panic!("what are you doing here"),
                };
                let new_position = (new_y as usize, new_x as usize);

                match distances.get(&(new_position, dir)) {
                    Some(&old) if new_cost > old => (),
                    Some(&old) if new_cost == old => {
                        came_from
                            .entry((new_position, dir))
                            .or_default()
                            .push(((y, x), current_dir));
                    }
                    _ => {
                        came_from.insert((new_position, dir), vec![((y, x), current_dir)]);
                        distances.insert((new_position, dir), new_cost);
                        heap.push(State {
                            cost: new_cost,
                            y: new_position.0,
                            x: new_position.1,
                            dir,
                        });
                    }
                }
            }
        }
    }

    None
}

fn display_map_with_path(grid: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            if path.contains(&(y, x)) {
                print!("O");
            } else {
                print!("{}", *c);
            }
        }
        println!();
    }
    println!();
}

fn display_map(grid: &Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{}", *c);
        }
        println!();
    }
    println!();
}

fn steps(input: &String) {
    let bindings = input.replace("\r\n", "\n");
    let grid: Vec<Vec<char>> = bindings
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (start, end) = find_start_end(&grid);

    match dijkstra(&grid, start, end) {
        Some((cost, path, unique_pos)) => {
            println!("Step 1: {}", cost);
            println!("Step 2: {}", unique_pos);
        }
        None => println!("No path found."),
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    steps(&input);
}
