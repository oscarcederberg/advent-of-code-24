use std::collections::{HashMap, HashSet};

const TEST: &'static str = include_str!("../../input/23/test.txt");
const INPUT: &'static str = include_str!("../../input/23/input.txt");

fn part_1(input: &str) -> usize {
    let mut connections = HashMap::new();
    let mut t_computers = HashSet::new();

    input.lines().for_each(|line| {
        let a = line[0..2].to_string();
        let b = line[3..5].to_string();
        connections
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        connections
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
        if a.starts_with('t') {
            t_computers.insert(a.clone());
        }
        if b.starts_with('t') {
            t_computers.insert(b.clone());
        }
    });

    let mut inter_connections = Vec::new();
    for a in t_computers {
        let a_connections = connections.get(&a).unwrap();
        for b in a_connections {
            let b_connections = connections.get(b).unwrap();
            for c in b_connections {
                if !a_connections.contains(c) || !b_connections.contains(c) {
                    continue;
                }

                let mut connection = HashSet::new();
                connection.insert(a.to_string());
                connection.insert(b.to_string());
                connection.insert(c.to_string());
                if inter_connections.contains(&connection) {
                    continue;
                }

                inter_connections.push(connection);
            }
        }
    }

    inter_connections.len()
}

fn bron_kerbhosch(
    connections: &mut HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    } else if !p.is_empty() {
        let vertices = p.iter().cloned().collect::<HashSet<String>>();
        vertices.iter().for_each(|vertex| {
            let neighbours: &HashSet<String> = &connections[vertex];
            let mut r = r.clone();
            r.insert(vertex.clone());
            bron_kerbhosch(
                connections,
                r,
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                cliques,
            );
            p.remove(vertex);
            x.insert(vertex.clone());
        });
    }
}

fn part_2(input: &str) -> usize {
    let mut connections = HashMap::new();
    let mut vertices = HashSet::new();

    input.lines().for_each(|line| {
        let a = line[0..2].to_string();
        let b = line[3..5].to_string();
        connections
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        connections
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
        vertices.insert(a.clone());
        vertices.insert(b.clone());
    });

    let mut cliques = Vec::new();

    bron_kerbhosch(
        &mut connections,
        HashSet::new(),
        vertices,
        HashSet::new(),
        &mut cliques,
    );

    cliques.sort_unstable_by_key(|clique| clique.len());
    let mut clique: Vec<String> = cliques.pop().unwrap().into_iter().collect::<Vec<String>>();
    clique.sort();
    println!("{}", clique.join(","));
    clique.len()
}

fn main() {
    println!("day 23");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
