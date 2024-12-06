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

fn add_move_if_distincts(moves: &mut Vec<Move>, new_move: &Move) {
    if !moves.contains(new_move) {
        moves.push(Move {
            point: Point {
                x: new_move.point.x,
                y: new_move.point.y,
            },
            dir: new_move.dir,
        });
    }
}

fn get_char(lines: &Vec<&str>, x: usize, y: usize) -> char {
    lines[y].chars().nth(x).unwrap()
}

fn step1(input: &String) -> Vec<Point> {
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

    return cases;
}

#[derive(Debug, PartialEq)]

struct Move {
    point: Point,
    dir: i32,
}

fn point_to_index(point: &Point, len: usize) -> usize {
    (point.y * len) + point.x
}

fn check_if_loop(moves: &Vec<Move>, new_move: &Move) -> bool {
    for m in moves {
        if m.point.x == new_move.point.x && m.point.y == new_move.point.y && m.dir == new_move.dir {
            return true;
        }
    }
    return false;
}

fn step2(path: &Vec<Point>, input: &String) {
    let new_input = input.clone().trim().replace("\r", "");
    let lines: Vec<_> = new_input.split("\n").collect();
    let y_len = lines.len();

    if y_len <= 0 {
        panic!("Invalid input");
    }
    let x_len = lines[0].len();

    let i = new_input
        .chars()
        .position(|c| "^>v<".contains(c))
        .expect("Expect ^>v< in input");
    let start_position = index_to_point(i, x_len);

    let c = new_input.chars().nth(i).unwrap();

    let start_direction = if c == '^' {
        0
    } else if c == '>' {
        1
    } else if c == 'v' {
        2
    } else {
        3
    };

    let mut answer = 0;

    for (i, case) in path.into_iter().enumerate() {
        if i != 0 {
            println!("Progress: {i}/{}", path.len());
            let mut current_directions = start_direction;
            let mut guard_position = Point {
                x: start_position.x,
                y: start_position.y,
            };
            let new_obstacle_index = point_to_index(case, x_len + 1);
            let mut new_input = input.clone().trim().replace("\r", "").to_string();
            new_input.replace_range(new_obstacle_index..new_obstacle_index + 1, "#");
            let map: Vec<_> = new_input.split("\n").collect();

            let mut moves: Vec<Move> = vec![Move {
                point: Point {
                    x: guard_position.x,
                    y: guard_position.y,
                },
                dir: current_directions,
            }];

            while (guard_position.y > 0 && current_directions == 0)
                || (guard_position.x < x_len - 1 && current_directions == 1)
                || (guard_position.y < y_len - 1 && current_directions == 2)
                || (guard_position.x > 0 && current_directions == 3)
            {
                if current_directions == 0 {
                    let c = get_char(&map, guard_position.x, guard_position.y - 1);

                    if c == '#' {
                        current_directions = (current_directions + 1) % 4;
                    } else {
                        guard_position.y -= 1;
                        let new_move = Move {
                            point: Point {
                                x: guard_position.x,
                                y: guard_position.y,
                            },
                            dir: current_directions,
                        };
                        if check_if_loop(&moves, &new_move) {
                            answer += 1;
                            break;
                        }
                        add_move_if_distincts(&mut moves, &new_move)
                    }
                }
                if current_directions == 1 {
                    let c = get_char(&map, guard_position.x + 1, guard_position.y);

                    if c == '#' {
                        current_directions = (current_directions + 1) % 4;
                    } else {
                        guard_position.x += 1;
                        let new_move = Move {
                            point: Point {
                                x: guard_position.x,
                                y: guard_position.y,
                            },
                            dir: current_directions,
                        };
                        if check_if_loop(&moves, &new_move) {
                            answer += 1;
                            break;
                        }
                        add_move_if_distincts(&mut moves, &new_move)
                    }
                }
                if current_directions == 2 {
                    let c = get_char(&map, guard_position.x, guard_position.y + 1);

                    if c == '#' {
                        current_directions = (current_directions + 1) % 4;
                    } else {
                        guard_position.y += 1;
                        let new_move = Move {
                            point: Point {
                                x: guard_position.x,
                                y: guard_position.y,
                            },
                            dir: current_directions,
                        };
                        if check_if_loop(&moves, &new_move) {
                            answer += 1;
                            break;
                        }
                        add_move_if_distincts(&mut moves, &new_move)
                    }
                }
                if current_directions == 3 {
                    let c = get_char(&map, guard_position.x - 1, guard_position.y);

                    if c == '#' {
                        current_directions = (current_directions + 1) % 4;
                    } else {
                        guard_position.x -= 1;
                        let new_move = Move {
                            point: Point {
                                x: guard_position.x,
                                y: guard_position.y,
                            },
                            dir: current_directions,
                        };
                        if check_if_loop(&moves, &new_move) {
                            answer += 1;
                            break;
                        }
                        add_move_if_distincts(&mut moves, &new_move)
                    }
                }
            }
        }
    }
    println!("Step 2 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    let cases = step1(&input.clone());
    println!("Step 1 : {}", cases.len());
    step2(&cases, &input.clone());
}
