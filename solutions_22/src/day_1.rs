use std::io::{self, BufRead};

fn get_calorie_count() -> Vec<u32> {
    let mut result_vector: Vec<u32> = Vec::with_capacity(1024);
    let mut callorie_count: u32 = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line.unwrap().trim().parse::<u32>() {
            Ok(cal) => callorie_count += cal,
            Err(_) => {
                result_vector.push(callorie_count);
                callorie_count = 0;
            }
        }
    }
    return result_vector;
}

fn solution() {
    let mut net_calories = get_calorie_count();
    net_calories.sort();
    let mut net_top_three_calories = 0;

    for x in &net_calories[net_calories.len() - 3..net_calories.len()] {
        net_top_three_calories += x;
    }

    println!("Net top three calories: {}", net_top_three_calories);
}
