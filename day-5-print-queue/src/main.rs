use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct PageOrder {
    page_before: Vec<usize>,
    page_after: Vec<usize>,
}

fn is_correct_order(page_map: &HashMap<usize, PageOrder>, pages_numbers: &Vec<usize>) -> bool {
    let len = pages_numbers.len();

    for (index, current_page) in pages_numbers.clone().into_iter().enumerate() {
        if page_map.contains_key(&current_page) {
            for i in index..len {
                match page_map.get(&current_page) {
                    Some(page) => {
                        if page.page_before.contains(&pages_numbers[i]) {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                }
            }
        } else {
            return false;
        }
    }
    return true;
}

fn step1(input: &String) {
    let lines: Vec<_> = input.split("\n").collect();
    let mut page_map: HashMap<usize, PageOrder> = HashMap::new();
    let mut answer = 0;

    for line in lines {
        if line.contains("|") {
            // Order declaration
            let order: Vec<_> = line.split("|").map(|f: &str| f.parse::<usize>().expect("Expect a number here")).collect();
            if order.len() != 2 {
                panic!("Invalid page order definition")
            }
            page_map.entry(order[0]).and_modify(|page| page.page_after.push(order[1])).or_insert_with(|| PageOrder { page_after: vec![order[1]], page_before: vec![]});
            page_map.entry(order[1]).and_modify(|page| page.page_before.push(order[0])).or_insert_with(|| PageOrder { page_after: vec![], page_before: vec![order[0]]});
        }
        if line.contains(",") {
            // Update print
            let pages_numbers: Vec<_> = line.split(",").map(|f: &str| f.parse::<usize>().expect("Expect a number here")).collect();
            let correct_order = is_correct_order(&page_map, &pages_numbers);

            if correct_order {
                answer += pages_numbers[pages_numbers.len() / 2];
            } else {
                // println!("Line {} is in wrong order", line);
            }
        }
    }
    println!("Step 1 : {}", answer);
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    step1(&input);
}