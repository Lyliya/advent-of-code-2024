use regex::Regex;
use std::fs;

fn parse_register(input: &String) -> (usize, usize, usize) {
    let register = Regex::new(r"Register (?:A|B|C): (?<n>\d*)").unwrap();

    let matches: Vec<regex::Captures<'_>> = register.captures_iter(&input.trim()).collect();
    let a = &matches[0]["n"].parse::<usize>().expect("Expect number");
    let b = &matches[1]["n"].parse::<usize>().expect("Expect number");
    let c = &matches[2]["n"].parse::<usize>().expect("Expect number");

    (*a, *b, *c)
}

fn get_program(input: &String) -> Vec<usize> {
    let program_regex = Regex::new(r"Program: (?<n>.*$)").unwrap();
    let matches: Vec<regex::Captures<'_>> = program_regex.captures_iter(&input.trim()).collect();
    let mut num = vec![];

    for n in matches[0]["n"].split(",") {
        num.push(n.parse::<usize>().expect("Expect number"));
    }

    num
}

fn get_operand(register: &(usize, usize, usize), operand: usize) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => register.0,
        5 => register.1,
        6 => register.2,
        7 => panic!("Reserved operand"),
        _ => panic!("Invalid operand"),
    }
}

fn adv(register: &(usize, usize, usize), operand: usize) -> usize {
    register.0 / usize::pow(2, get_operand(register, operand) as u32)
}

fn bxl(register: &(usize, usize, usize), operand: usize) -> usize {
    register.1 ^ operand
}

fn bst(register: &(usize, usize, usize), operand: usize) -> usize {
    get_operand(register, operand) % 8
}

// The jnz instruction (opcode 3) does nothing if the A register is 0. However,
// if the A register is not zero, it jumps by setting the
// instruction pointer to the value of its literal operand;
// if this instruction jumps, the instruction pointer is not
// increased by 2 after this instruction.
fn jnz(operand: usize) -> usize {
    operand
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
// then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(register: &(usize, usize, usize)) -> usize {
    register.1 ^ register.2
}

fn emul(num: &Vec<usize>, register: &mut (usize, usize, usize)) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];

    let mut i = 0;

    while i < num.len() - 1 {
        let opcode = num[i];
        let operand = num[i + 1];
        match opcode {
            0 => {
                register.0 = adv(&register, operand);
                i += 2;
            }
            1 => {
                register.1 = bxl(&register, operand);
                i += 2;
            }
            2 => {
                register.1 = bst(&register, operand);
                i += 2;
            }
            3 => {
                if register.0 != 0 {
                    let j = jnz(operand);
                    i = j * 2;
                } else {
                    i += 2;
                }
            }
            4 => {
                register.1 = bxc(&register);
                i += 2;
            }
            5 => {
                out.push(get_operand(&register, operand) % 8);
                i += 2;
            }
            6 => {
                register.1 = adv(&register, operand);
                i += 2;
            }
            7 => {
                register.2 = adv(&register, operand);
                i += 2;
            }
            _ => {
                i += 2;
            }
        }
    }

    out
}

fn step1(input: &String) {
    let mut register = parse_register(input);
    let num = get_program(input);

    let out = emul(&num, &mut register);

    let joined = out
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("Step 1 : {}", joined);
}

// 2,4: Take A % 8 -> Store in B
// 1,4: B ^ 4 -> Store in B
// 7,5: A / 2^B -> Store in B
// 4,1: B ^ C -> Store in B
// 1,4: B ^ 4 -> Store in B
// 5,5: B % 8 -> Output
// 0,3: A / 2^3 -> Store in A || No impact on the following
// 3,0: Restart until 0 || No impact on the following

fn prog(a: usize) -> usize {
    let mut b = a % 8; // 2,4
    b = b ^ 4; // 1,4
    let c = a / usize::pow(2, b as u32); // 7,5
    b = b ^ c; // 4,1
    b = b ^ 4; // 1,4
    b % 8
}

fn retro(out: &mut usize, num: &Vec<usize>, a: usize, i: usize) {
    let l = prog(a);
    if *out > 0 {
        return;
    }
    if l != num[i] {
        return;
    }
    if i == 0 && l == num[0] {
        *out = a;
    } else {
        for b in 0..8 {
            retro(out, num, (a * 8) + b, i - 1);
        }
    }
}

fn step2(input: &String) {
    let num = get_program(input);

    let mut out = 0;

    for a in 1..8 {
        if prog(a) == num[num.len() - 1] {
            retro(&mut out, &num, a, num.len() - 1);
        }
    }

    println!("Step 2 : {}", out);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
    step2(&input);
}
