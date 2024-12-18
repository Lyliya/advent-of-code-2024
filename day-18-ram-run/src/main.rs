use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    y: usize,
    x: usize,
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

fn construct_grid() -> Vec<Vec<char>> {
    let size = 70;
    let mut grid: Vec<Vec<char>> = vec![];
    
    for y in 0..size + 1 {
        grid.push(vec![]);
        for _x in 0..size + 1 {
            grid[y].push('.');
        }
    }

    grid
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

fn dijkstra(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut heap = BinaryHeap::from([State {
        cost: 0,
        y: start.0,
        x: start.1,
    }]);
    let mut distances = HashMap::new();
    let mut came_from: HashMap<(usize, usize),(usize, usize)> = HashMap::new();
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;


    distances.insert(start, 0);

    while let Some(State {
        cost,
        y,
        x,
    }) = heap.pop()
    {
        if let Some(&old) = distances.get(&(y, x)) {
            if cost > old {
                continue;
            }
        }

        if (y, x) == end {
            let mut stack = vec![((y, x))];
            let mut path: Vec<(usize, usize)> = vec![];

            while let Some(pos) = stack.pop() {
                if let Some(prev) = came_from.get(&pos) {
                    path.push(*prev);
                    stack.push(*prev);
                }
            }

            path.reverse();
            path.push(end);
            return Some((cost, path));
        }

        for dir in DIRECTIONS.into_iter() {
            let new_y = y as isize + dir.0;
            let new_x = x as isize + dir.1;

            if new_y >= 0 && new_y <= y_max as isize && new_x >= 0 && new_x <= x_max as isize && grid[new_y as usize][new_x as usize] != '#' {
                let new_cost = cost + 1;
                let new_position = (new_y as usize, new_x as usize);

                match distances.get(&(new_position)) {
                    Some(&old) if new_cost > old => (),
                    Some(&old) if new_cost == old => {
                        came_from
                            .entry(new_position)
                            .or_insert((y, x));
                    }
                    _ => {
                        came_from.insert(new_position, (y, x));
                        distances.insert(new_position, new_cost);
                        heap.push(State {
                            cost: new_cost,
                            y: new_position.0,
                            x: new_position.1,
                        });
                    }
                }
            }
        }
    }

    None
}

fn step1(input: &String) {
    let lines: Vec<_> = input.trim().lines().collect();
    let mut ram: Vec<(usize, usize)> = vec![];

    let mut grid = construct_grid();
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    for line in lines {
        let s: Vec<&str> = line.split(",").collect();
        ram.push((s[1].parse::<usize>().expect("Expect number"), s[0].parse::<usize>().expect("Expect number")))
    }

    for i in 0..1024 {
        let f = ram[i];
        grid[f.0][f.1] = '#';
    }

    match dijkstra(&grid, (0,0), (y_max, x_max)) {
        Some((_cost, path)) => {
            display_map_with_path(&grid, &path);
            println!("Step 1 : {}", path.len() - 1);
        }
        None => {
            println!("No path found !");
        }
    }

    
}

fn find_mid(min: usize, max: usize) -> usize {
    (min + max) / 2
}

fn step2(input: &String) {
    let lines: Vec<_> = input.trim().lines().collect();
    let mut ram: Vec<(usize, usize)> = vec![];

    let grid = construct_grid();
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    for line in lines {
        let s: Vec<&str> = line.split(",").collect();
        ram.push((s[1].parse::<usize>().expect("Expect number"), s[0].parse::<usize>().expect("Expect number")))
    }

    let mut min = 0;
    let mut max = ram.len() - 1;
    let mut i = (ram.len() - 1) / 2;

    while min != max - 1 {
        let mut copy_grid = grid.clone();
        for c in 0..i {
            let f = ram[c];
            copy_grid[f.0][f.1] = '#';
        }
        match dijkstra(&copy_grid, (0,0), (y_max, x_max)) {
            Some((_cost, _path)) => {
                min = i;
                i = find_mid(min, max);
            }
            None => {
                max = i;
                if min != max - 1 {
                    i = find_mid(min, max)
                }
            }
        }
    }
    println!("Step 2 : {},{}", ram[i].1, ram[i].0);
    return;
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
