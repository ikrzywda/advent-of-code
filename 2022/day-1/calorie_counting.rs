use std::io::{self, BufRead};

fn get_calory_count() -> Vec<i32> {
    let mut result_vector: Vec<i32> = Vec::with_capacity(1024);
    let mut callorie_count: i32 = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line.unwrap().trim().parse::<i32>() {
            Ok(cal) => callorie_count += cal,
            Err(_) => {
                result_vector.push(callorie_count);
                callorie_count = 0;
            }
        }
    }
    return result_vector;
}

fn main() {
    let net_calories = get_calory_count();
    let mut max_val = 0;

    for &net_calorie in net_calories.iter() {
        if net_calorie > max_val {
            max_val = net_calorie;
        }
    }
    println!("THE RESULT IS: {}", max_val);
}
