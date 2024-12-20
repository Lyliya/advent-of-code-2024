use std::collections::{BinaryHeap, HashMap, HashSet};
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

fn parse_input(input: &String) -> (((usize, usize), (usize, usize)), Vec<Vec<char>>) {
    let bindings = input.replace("\r\n", "\n");
    let grid: Vec<Vec<char>> = bindings
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    (find_start_end(&grid), grid)
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
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    distances.insert(start, 0);

    while let Some(State { cost, y, x }) = heap.pop() {
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

            if new_y >= 0
                && new_y <= y_max as isize
                && new_x >= 0
                && new_x <= x_max as isize
                && grid[new_y as usize][new_x as usize] != '#'
            {
                let new_cost = cost + 1;
                let new_position = (new_y as usize, new_x as usize);

                match distances.get(&(new_position)) {
                    Some(&old) if new_cost > old => (),
                    Some(&old) if new_cost == old => {
                        came_from.entry(new_position).or_insert((y, x));
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

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (isize::abs(b.1 as isize - a.1 as isize) + isize::abs(b.0 as isize - a.0 as isize)) as usize
}

fn step2(input: &String) {
    let ((start, end), grid) = parse_input(input);
    let save_time = 100;

    match dijkstra(&grid, start, end) {
        Some((_cost, path)) => {
            let mut current = 0;
            let mut answer = 0;

            // For each pos that can save me save_time
            while current < path.len() - save_time {
                // Start from the end
                let mut jump_index = path.len() - 1;

                // While I can save enough time
                while jump_index >= current + save_time {
                    // Check the distance use to skip
                    let distance = manhattan_distance(path[current], path[jump_index]);
                    // If less than 20 -> valid shortcut
                    if distance <= 20 {
                        let mut c = path.clone();
                        let remove: Vec<_> = c.drain(current + 1..jump_index).collect();
                        if remove.len() - distance + 1 >= save_time {
                            answer += 1;
                        }
                    }
                    jump_index -= 1;
                }
                current += 1;
            }
            println!("Total skip of {}: {}", save_time, answer);
        }
        None => {}
    }
}

fn step1(input: &String) {
    let ((start, end), grid) = parse_input(input);
    let save_time = 100 + 2;

    match dijkstra(&grid, start, end) {
        Some((_cost, path)) => {
            let mut current = 0;
            let mut answer = 0;
            while current < path.len() - save_time {
                let mut skipped: HashSet<(usize, usize)> = HashSet::new();

                for jump_index in current + save_time..path.len() {
                    let distance = manhattan_distance(path[current], path[jump_index]);
                    if distance == 2 {
                        if skipped.contains(&path[jump_index]) {
                            continue;
                        }

                        let mut c = path.clone();
                        let remove: Vec<_> = c.drain(current + 1..jump_index).collect();
                        skipped.extend(remove);
                        answer += 1;
                    }
                }
                current += 1;
            }
            println!("Total skip of {}: {}", save_time - 2, answer);
        }
        None => {}
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
