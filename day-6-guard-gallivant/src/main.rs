use std::fs;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn index_to_point(index: usize, len: usize) -> Point {
    Point {
        x: index % (len + 1),
        y: index / (len + 1),
    }
}

fn add_pos_if_distincts(cases: &mut Vec<Point>, pos: &Point) {
    if !cases.contains(pos) {
        cases.push(Point { x: pos.x, y: pos.y });
    }
}

fn get_char(lines: &Vec<&str>, x: usize, y: usize) -> char {
    lines[y].chars().nth(x).unwrap()
}

fn step1(input: &String) {
    let lines: Vec<_> = input.trim().split("\n").collect();
    let y_len = lines.len();

    if y_len <= 0 {
        panic!("Invalid input");
    }
    let x_len = lines[0].len();

    let i = input
        .chars()
        .position(|c| "^>v<".contains(c))
        .expect("Expect ^>v< in input");
    let c = input.chars().nth(i).unwrap();

    let mut guard_position = index_to_point(i, x_len);
    let mut current_directions = if c == '^' {
        0
    } else if c == '>' {
        1
    } else if c == 'v' {
        2
    } else {
        3
    };

    let mut cases: Vec<Point> = vec![Point {
        x: guard_position.x,
        y: guard_position.y,
    }];

    while (guard_position.y > 0 && current_directions == 0)
        || (guard_position.x < x_len - 1 && current_directions == 1)
        || (guard_position.y < y_len - 1 && current_directions == 2)
        || (guard_position.x > 0 && current_directions == 3)
    {
        if current_directions == 0 {
            let c = get_char(&lines, guard_position.x, guard_position.y - 1);

            if c == '#' {
                current_directions = (current_directions + 1) % 4;
            } else {
                guard_position.y -= 1;
                add_pos_if_distincts(&mut cases, &guard_position)
            }
        }
        if current_directions == 1 {
            let c = get_char(&lines, guard_position.x + 1, guard_position.y);

            if c == '#' {
                current_directions = (current_directions + 1) % 4;
            } else {
                guard_position.x += 1;
                add_pos_if_distincts(&mut cases, &guard_position)
            }
        }
        if current_directions == 2 {
            let c = get_char(&lines, guard_position.x, guard_position.y + 1);

            if c == '#' {
                current_directions = (current_directions + 1) % 4;
            } else {
                guard_position.y += 1;
                add_pos_if_distincts(&mut cases, &guard_position)
            }
        }
        if current_directions == 3 {
            let c = get_char(&lines, guard_position.x - 1, guard_position.y);

            if c == '#' {
                current_directions = (current_directions + 1) % 4;
            } else {
                guard_position.x -= 1;
                add_pos_if_distincts(&mut cases, &guard_position)
            }
        }
    }

    println!("Step 1: {}", cases.len());
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
}
