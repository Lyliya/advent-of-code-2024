use std::collections::HashMap;
use std::fs;

fn get_numpad_position(key: char) -> (usize, usize) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!(),
    }
}

fn get_arrowpad_position(key: char) -> (usize, usize) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!(),
    }
}

fn get_sequence_y(num: isize) -> Vec<char> {
    let mut sequence = vec![];
    let positive = num > 0;

    for _i in 0..num.abs() {
        sequence.push(if positive { 'v' } else { '^' });
    }

    sequence
}

fn get_sequence_x(num: isize) -> Vec<char> {
    let mut sequence = vec![];
    let positive = num > 0;

    for _i in 0..num.abs() {
        sequence.push(if positive { '>' } else { '<' });
    }

    sequence
}

fn generate_sequence(
    y: isize,
    x: isize,
    steps: usize,
    avoid_empty: bool,
    memo: &mut HashMap<(isize, isize, usize, bool), usize>,
) -> usize {
    if let Some(result) = memo.get(&(y, x, steps, avoid_empty)) {
        return *result;
    }

    let mut sequence = vec![];
    if avoid_empty {
        sequence.extend(get_sequence_y(y));
        sequence.extend(get_sequence_x(x));
    } else {
        sequence.extend(get_sequence_x(x));
        sequence.extend(get_sequence_y(y));
    }

    sequence.push('A');

    if steps == 0 {
        memo.insert((y, x, steps, avoid_empty), sequence.len());
        return sequence.len();
    } else {
        let mut pos = get_arrowpad_position('A');
        let mut len: usize = 0;

        for c in sequence {
            let end = get_arrowpad_position(c);
            let d = (
                end.0 as isize - pos.0 as isize,
                end.1 as isize - pos.1 as isize,
            );

            if end == (1, 0) && pos.0 == 0 {
                len += generate_sequence(d.0, d.1, steps - 1, true, memo);
            } else if pos == (1, 0) && end.0 == 0 {
                len += generate_sequence(d.0, d.1, steps - 1, false, memo);
            } else {
                // check both, add the smallest one
                let a = generate_sequence(d.0, d.1, steps - 1, false, memo);
                let b = generate_sequence(d.0, d.1, steps - 1, true, memo);

                if a <= b {
                    len += a;
                } else {
                    len += b;
                }
            }

            pos = end;
        }

        memo.insert((y, x, steps, avoid_empty), len);
        len
    }
}

fn get_keypad_code(code: &str, steps: usize) -> usize {
    let mut sequence: usize = 0;
    let mut pos = get_numpad_position('A');
    let mut memo = HashMap::new();

    for c in code.chars() {
        let end = get_numpad_position(c);

        let d = (
            end.0 as isize - pos.0 as isize,
            end.1 as isize - pos.1 as isize,
        );

        if pos.0 == 3 && end.1 == 0 {
            sequence += generate_sequence(d.0, d.1, steps, true, &mut memo);
        } else if pos.1 == 0 && end.0 == 3 {
            sequence += generate_sequence(d.0, d.1, steps, false, &mut memo);
        } else {
            // check both, add the smallest one
            let a = generate_sequence(d.0, d.1, steps, false, &mut memo);
            let b = generate_sequence(d.0, d.1, steps, true, &mut memo);

            if a <= b {
                sequence += a;
            } else {
                sequence += b;
            }
        }

        pos = end;
    }

    sequence
}

fn step1(input: &String) {
    let codes: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for code in codes {
        let sequence = get_keypad_code(code, 2);
        let code_num = code[0..code.len() - 1]
            .parse::<usize>()
            .expect("Expect number");

        total += code_num * sequence;
    }

    println!("Step 1 : {}", total);
}

fn step2(input: &String) {
    let codes: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for code in codes {
        let sequence = get_keypad_code(code, 25);
        let code_num = code[0..code.len() - 1]
            .parse::<usize>()
            .expect("Expect number");

        total += code_num * sequence;
    }

    println!("Step 2 : {}", total);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
