use regex::Regex;
use std::collections::HashSet;
use std::{
    collections::{HashMap, VecDeque},
    fs::{self},
};

fn parse_input(input: &String) -> (HashMap<String, i8>, Vec<String>) {
    let bindings = input.trim().replace("\r\n", "\n");
    let (init, op) = bindings.split_once("\n\n").unwrap();

    let mut gates: HashMap<String, i8> = HashMap::new();

    for i in init.lines() {
        let (gate, value) = i.split_once(": ").unwrap();

        gates.insert(
            gate.to_string(),
            value.parse::<i8>().expect("Expect number"),
        );
    }

    let ops = op.lines().map(|l| l.to_string()).collect();

    (gates, ops)
}

fn do_op(a: i8, b: i8, op: &str, out: &str, gates: &mut HashMap<String, i8>) {
    match op {
        "AND" => {
            gates.insert(out.to_string(), if a == 1 && b == 1 { 1 } else { 0 });
        }
        "OR" => {
            gates.insert(out.to_string(), if a == 1 || b == 1 { 1 } else { 0 });
        }
        "XOR" => {
            gates.insert(out.to_string(), if a != b { 1 } else { 0 });
        }
        _ => (),
    }
}

fn get_representation(gates: &HashMap<String, i8>, pat: char) -> (String, usize) {
    let mut keys: Vec<&String> = gates.keys().filter(|key| key.starts_with(pat)).collect();

    keys.sort();
    keys.reverse();

    let mut binary = String::new();

    for key in keys {
        if let Some(value) = gates.get(key) {
            binary.push_str(&value.to_string());
        }
    }

    let decimal = usize::from_str_radix(&binary, 2).unwrap();

    (binary, decimal)
}

fn output_z(gates: &HashMap<String, i8>) {
    let (binary, _decimal) = get_representation(gates, 'z');

    let decimal = usize::from_str_radix(&binary, 2).unwrap();
    println!("Step 1 : {} - {}", binary, decimal);
}

fn step1(input: &String) {
    let (mut gates, ops) = parse_input(input);
    let mut ops = VecDeque::from(ops);

    let re = Regex::new(r"(?<a>.*) (?<op>.*) (?<b>.*) -> (?<out>.*)").unwrap();

    while let Some(op) = ops.pop_front() {
        if let Some(captures) = re.captures(&op) {
            let a = captures.name("a").unwrap().as_str();
            let operator = captures.name("op").unwrap().as_str();
            let b = captures.name("b").unwrap().as_str();
            let out = captures.name("out").unwrap().as_str();

            if !gates.contains_key(a) || !gates.contains_key(b) {
                ops.push_back(op.to_string());
            } else {
                let a_val = *gates.get(a).unwrap();
                let b_val = *gates.get(b).unwrap();
                do_op(a_val, b_val, operator, out, &mut gates);
            }
        }
    }

    output_z(&gates);
}

fn step2(input: &String) {
    let (mut _gates, ops) = parse_input(input);

    let re = Regex::new(r"(?<a>.*) (?<op>.*) (?<b>.*) -> (?<out>.*)").unwrap();

    let mut broken = HashSet::new();

    let mut entry: HashMap<String, usize> = HashMap::new();

    for op in &ops {
        if let Some(captures) = re.captures(&op) {
            let a = captures.name("a").unwrap().as_str();
            let b = captures.name("b").unwrap().as_str();

            *entry.entry(a.to_string()).or_default() += 1;
            *entry.entry(b.to_string()).or_default() += 1;
        }
    }

    for op in &ops {
        if let Some(captures) = re.captures(&op) {
            let a = captures.name("a").unwrap().as_str();
            let operator = captures.name("op").unwrap().as_str();
            let b = captures.name("b").unwrap().as_str();
            let out = captures.name("out").unwrap().as_str();

            // You cannot connect XOR output to more than 2 nodes
            if operator == "XOR" && !out.starts_with('z') && *entry.get(out).unwrap() != 2 {
                broken.insert(out.to_string());
            }

            // Same for AND, no more than 1, except when no carry
            if operator == "AND"
                && !out.starts_with('z')
                && *entry.get(out).unwrap() != 1
                && !((a == "x00" && b == "y00") || (a == "y00" && b == "x00"))
            {
                broken.insert(out.to_string());
            }

            // Z can only be outputed in XOR
            if out.starts_with('z') && out != "z45" && operator != "XOR" {
                broken.insert(out.to_string());
            }

            // non-z XOR should only be X or Y
            if !out.starts_with('z')
                && operator == "XOR"
                && !((a.starts_with('x') && b.starts_with('y'))
                    || (a.starts_with('y') && b.starts_with('x')))
            {
                broken.insert(out.to_string());
            }
        }
    }

    let mut sorted: Vec<String> = broken.into_iter().collect();
    sorted.sort();

    print!("Step 2 : {}", sorted.join(","));
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
