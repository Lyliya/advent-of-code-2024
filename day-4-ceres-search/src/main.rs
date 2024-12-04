use std::fs;

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

fn check_vert(lines: &Vec<&str>, pos: &Point) -> usize {
    let mut occurences = 0;

    if pos.y >= 3 
    && lines[pos.y - 1].chars().nth(pos.x).unwrap() == 'M'
    && lines[pos.y - 2].chars().nth(pos.x).unwrap() == 'A'
    && lines[pos.y - 3].chars().nth(pos.x).unwrap() == 'S' {
        occurences += 1;
    }
    if pos.y < lines.len() - 3
    && lines[pos.y + 1].chars().nth(pos.x).unwrap() == 'M'
    && lines[pos.y + 2].chars().nth(pos.x).unwrap() == 'A'
    && lines[pos.y + 3].chars().nth(pos.x).unwrap() == 'S' {
        occurences += 1;
    }
    occurences
}

fn check_hor(lines: &Vec<&str>, pos: &Point) -> usize {
    let mut occurences = 0;

    if pos.x >= 3
    && lines[pos.y].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y].chars().nth(pos.x - 2).unwrap() == 'A'
    && lines[pos.y].chars().nth(pos.x - 3).unwrap() == 'S' {
        occurences += 1;
    }
    if pos.x < lines[0].len() - 3
    && lines[pos.y].chars().nth(pos.x + 1).unwrap() == 'M'
    && lines[pos.y].chars().nth(pos.x + 2).unwrap() == 'A'
    && lines[pos.y].chars().nth(pos.x + 3).unwrap() == 'S' {
        occurences += 1;
    }
    occurences
}

fn check_diag(lines: &Vec<&str>, pos: &Point) -> usize {
    let mut occurences = 0;

    if pos.x >= 3 && pos.y >= 3
    && lines[pos.y - 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y - 2].chars().nth(pos.x - 2).unwrap() == 'A'
    && lines[pos.y - 3].chars().nth(pos.x - 3).unwrap() == 'S' {
        occurences += 1;
    }
    if pos.x < lines[0].len() - 3 && pos.y < lines.len() - 3
    && lines[pos.y + 1].chars().nth(pos.x + 1).unwrap() == 'M'
    && lines[pos.y + 2].chars().nth(pos.x + 2).unwrap() == 'A'
    && lines[pos.y + 3].chars().nth(pos.x + 3).unwrap() == 'S' {
        occurences += 1;
    }

    if pos.x < lines[0].len() - 3 && pos.y >= 3
    && lines[pos.y - 1].chars().nth(pos.x + 1).unwrap() == 'M'
    && lines[pos.y - 2].chars().nth(pos.x + 2).unwrap() == 'A'
    && lines[pos.y - 3].chars().nth(pos.x + 3).unwrap() == 'S' {
        occurences += 1;
    }
    if pos.x >= 3 && pos.y < lines.len() - 3
    && lines[pos.y + 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y + 2].chars().nth(pos.x - 2).unwrap() == 'A'
    && lines[pos.y + 3].chars().nth(pos.x - 3).unwrap() == 'S' {
        occurences += 1;
    }
    occurences
}

fn step1(input: &String) {
    let lines: Vec<_> = input.split("\n").collect();
    if lines.len() <= 0 {
        panic!("Invalid input file!")
    };
    let mut occurences = 0;
    for (i, c) in input.chars().enumerate() {
        if c == 'X' {
            let pos = index_to_point(i, lines[0].len());
            occurences += check_vert(&lines, &pos);
            occurences += check_hor(&lines, &pos);
            occurences += check_diag(&lines, &pos);
        }
    }
    println!("Step 1 : {}", occurences);
}

fn check_x_mas(lines: &Vec<&str>, pos: &Point) -> usize {
    let mut occurences = 0;
    let left_boundary = lines[0].len() - 1;
    let bottom_boundary = lines.len() - 1;

    if pos.x < 1 || pos.y < 1 || pos.x >= left_boundary || pos.y >= bottom_boundary {
        return 0
    }
    // M.S
    // .A.
    // M.S
    if lines[pos.y - 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y - 1].chars().nth(pos.x + 1).unwrap() == 'S'
    && lines[pos.y + 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y + 1].chars().nth(pos.x + 1).unwrap() == 'S' {
        occurences += 1;
    }

    // S.M
    // .A.
    // S.M
    if lines[pos.y - 1].chars().nth(pos.x - 1).unwrap() == 'S'
    && lines[pos.y - 1].chars().nth(pos.x + 1).unwrap() == 'M'
    && lines[pos.y + 1].chars().nth(pos.x - 1).unwrap() == 'S'
    && lines[pos.y + 1].chars().nth(pos.x + 1).unwrap() == 'M' {
        occurences += 1;
    }

    // M.M
    // .A.
    // S.S
    if lines[pos.y - 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y - 1].chars().nth(pos.x + 1).unwrap() == 'M'
    && lines[pos.y + 1].chars().nth(pos.x - 1).unwrap() == 'S'
    && lines[pos.y + 1].chars().nth(pos.x + 1).unwrap() == 'S' {
        occurences += 1;
    }

    // S.S
    // .A.
    // M.M
    if lines[pos.y - 1].chars().nth(pos.x - 1).unwrap() == 'S'
    && lines[pos.y - 1].chars().nth(pos.x + 1).unwrap() == 'S'
    && lines[pos.y + 1].chars().nth(pos.x - 1).unwrap() == 'M'
    && lines[pos.y + 1].chars().nth(pos.x + 1).unwrap() == 'M' {
        occurences += 1;
    }
    occurences
}

fn step2(input: &String) {
    let lines: Vec<_> = input.split("\n").collect();
    if lines.len() <= 0 {
        panic!("Invalid input file!")
    };
    let mut occurences = 0;
    for (i, c) in input.chars().enumerate() {
        if c == 'A' {
            let pos = index_to_point(i, lines[0].len());
            occurences += check_x_mas(&lines, &pos);
        }
    }
    println!("Step 2 : {}", occurences);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
