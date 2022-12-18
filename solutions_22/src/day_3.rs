use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn compute_priority(line: &str) -> u32 {
    let mut net_priority: u32 = 0;
    let first_compartment = &line[0..line.len() / 2];
    let second_compartment = &line[line.len() / 2..line.len()];

    let items_map = first_compartment
        .chars()
        .into_iter()
        .map(|c| (c, c))
        .collect::<HashMap<char, char>>();

    let mut results_map = HashMap::<char, char>::new();

    for item in second_compartment.chars().into_iter() {
        if items_map.contains_key(&item) && !results_map.contains_key(&item) {
            results_map.insert(item, item);
            let priority = if item.is_ascii_lowercase() {
                item as u32 - 96
            } else {
                item as u32 - 38
            };
            println!("char: {}, value: {}", item, priority);
            net_priority += priority;
        }
    }
    return net_priority;
}

fn priority_sum() -> u32 {
    let mut net_score: u32 = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                net_score += compute_priority(&line);
            }
            Err(_) => eprintln!("Invalid input"),
        }
    }
    return net_score;
}

pub fn solution() {
    println!("Result: {}", priority_sum());
}
