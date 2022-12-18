use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn compute_priority(line: &str) -> u32 {
    let mut mapping_array: [u32; 53] = [0; 53];
    let mut net_priority: u32 = 0;

    let compartment_midpoint = line.len() / 2;
    let first_compartment = &line[0..compartment_midpoint];
    let second_compartment = &line[compartment_midpoint..line.len()];

    for item in first_compartment.chars().into_iter() {
        let item_priority = if item.is_ascii_lowercase() {
            item as u32 - 96
        } else {
            item as u32 - 38
        };
        mapping_array[item_priority as usize] += 1;
    }

    for item in second_compartment.chars().into_iter() {
        let item_priority = if item.is_ascii_lowercase() {
            item as u32 - 96
        } else {
            item as u32 - 38
        };

        if mapping_array[item_priority as usize] != 0 {
            net_priority += item_priority;
            mapping_array[item_priority as usize] = 0;
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
