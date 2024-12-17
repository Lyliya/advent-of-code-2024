use std::fs;
use regex::Regex;

fn parse_register(input: &String) -> (usize, usize, usize) {
    let register = Regex::new(r"Register (?:A|B|C): (?<n>\d*)").unwrap();

    let matches: Vec<regex::Captures<'_>> = register.captures_iter(&input).collect();
    let a = &matches[0]["n"].parse::<usize>().expect("Expect number");
    let b = &matches[1]["n"].parse::<usize>().expect("Expect number");
    let c = &matches[2]["n"].parse::<usize>().expect("Expect number");

    

    (*a,*b,*c)
}

fn get_program(input: &String) -> Vec<usize> {
    let program_regex = Regex::new(r"Program: (?<n>.*$)").unwrap();
    let matches: Vec<regex::Captures<'_>> = program_regex.captures_iter(&input).collect();
    let mut num = vec![];
    
    for n in matches[0]["n"].split(",") {
        num.push(n.parse::<usize>().expect("Expect number"));
    }

    num
}

fn get_operand(register: (usize, usize, usize), operand: usize) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => register.0,
        5 => register.1,
        6 => register.2,
        7 => panic!("Reserved operand"),
        _ => panic!("Invalid operand")
    }
}

fn adv(register: &(usize, usize, usize), operand: usize) -> usize {
    register.0 / usize::pow(2, get_operand(*register, operand) as u32)
}

fn bxl(register: &(usize, usize, usize), operand: usize) -> usize {
    register.1 ^ operand
}

fn bst(register: &(usize, usize, usize), operand: usize) -> usize {
    get_operand(*register, operand) % 8
}

// The jnz instruction (opcode 3) does nothing if the A register is 0. However,
// if the A register is not zero, it jumps by setting the
// instruction pointer to the value of its literal operand;
// if this instruction jumps, the instruction pointer is not
// increased by 2 after this instruction.
fn jnz(operand: usize) -> usize {
    // println!("Register jnz: {:?}", register);
    operand
}

// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
// then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
fn bxc(register: &(usize, usize, usize)) -> usize {
    register.1 ^ register.2
}

fn step1(input: &String) {
    let mut register = parse_register(input);
    let num = get_program(input);
    let mut out: Vec<usize> = vec![];

    let mut i = 0;

    while i < num.len() - 1 {
        let opcode = num[i];
        let operand = num[i + 1];
        match opcode {
            0 => {
                register.0 = adv(&register, operand);
                i += 2;
            },
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
                out.push(get_operand(register, operand) % 8);
                i += 2;
            }
            6 => {
                register.1 = adv(&register, operand);
                i += 2;
            },
            7 => {
                register.2 = adv(&register, operand);
                i += 2;
            },
            _ => {
                i += 2;
            }
        }
    }

    // out.reverse();
    let joined = out.into_iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",");
    println!("Step 1 : {}", joined);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
}
