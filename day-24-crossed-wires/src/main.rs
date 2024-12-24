use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs,
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

fn print_gates(gates: &HashMap<String, i8>) {
    for (key, value) in gates.iter() {
        println!("{}: {}", key, value);
    }
}

fn output_z(gates: &HashMap<String, i8>) {
    let mut z_keys: Vec<&String> = gates
        .keys()
        .filter(|key| key.starts_with('z')) // Filter keys that start with 'z'
        .collect();

    z_keys.sort();
    z_keys.reverse();

    let mut binary = String::new();

    for key in z_keys {
        if let Some(value) = gates.get(key) {
            binary.push_str(&value.to_string());
        }
    }

    let decimal = usize::from_str_radix(&binary, 2).unwrap();
    println!("Binary: {}", binary);
    println!("Decimal: {}", decimal);
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

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
}
