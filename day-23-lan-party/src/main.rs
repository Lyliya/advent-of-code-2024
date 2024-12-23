use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_input(input: &String) -> HashMap<String, HashSet<String>> {
    let lines = input.trim().lines();
    let mut tree: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once("-").unwrap();
        tree.entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        tree.entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    }

    tree
}

fn step1(input: &String) {
    let tree = parse_input(input);

    let mut groups: HashSet<Vec<&String>> = HashSet::default();

    for (n1, set) in tree.iter() {
        for n2 in set {
            for n3 in tree.get(n2).unwrap().iter() {
                if tree.get(n3).unwrap().contains(n1) {
                    let mut group = vec![n1, n2, n3];
                    group.sort();
                    groups.insert(group);
                }
            }
        }
    }

    let answer = groups
        .iter()
        .filter(|g| g.iter().any(|n| n.starts_with("t")))
        .count();

    println!("Step 1 : {:?}", answer);
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    tree: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let p_iter = p.clone();
    for v in p_iter {
        let default = HashSet::new();
        let neighbors = tree.get(&v).unwrap_or(&default);

        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let mut new_p = p.intersection(neighbors).cloned().collect();
        let mut new_x = x.intersection(neighbors).cloned().collect();

        bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, tree, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

fn step2(input: &String) {
    let tree = parse_input(input);

    let mut r = HashSet::new();
    let mut p: HashSet<String> = tree.keys().cloned().collect();
    let mut x = HashSet::new();
    let mut cliques = vec![];

    bron_kerbosch(&mut r, &mut p, &mut x, &tree, &mut cliques);

    let mut answer = cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<String>>();

    answer.sort();

    println!("Step 2 : {}", answer.join(","));
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    step1(&input);
    step2(&input);
}
