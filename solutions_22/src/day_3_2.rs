use std::io::{self, BufRead};

fn common_item(lines: &[String; 3]) -> u32 {
    let mut mapping_array: [[u32; 53]; 3] = [[0; 53]; 3];

    for (i, line) in lines.iter().enumerate() {
        for item in line.chars().into_iter() {
            let item_priority = if item.is_ascii_lowercase() {
                item as u32 - 96
            } else {
                item as u32 - 38
            };
            mapping_array[i][item_priority as usize] = 1;
        }
    }
    println!("{:?}", mapping_array);

    for i in 0..53 {
        if mapping_array[0][i] + mapping_array[1][i] + mapping_array[2][i] == 3 {
            print!("{} ", i);
            return i as u32;
        }
    }

    return 0;
}

fn priority_sum() -> u32 {
    let mut net_score: u32 = 0;
    let stdin = io::stdin();
    let mut current_lines: [String; 3] = Default::default();

    for (i, line) in stdin.lock().lines().enumerate() {
        match line {
            Ok(line) => current_lines[i % 3] = line,
            Err(_) => eprintln!("Invalid input"),
        }
        if (i + 1) % 3 == 0 {
            net_score += common_item(&current_lines);
            println!("{}", i);
        }
    }
    return net_score;
}

pub fn solution() {
    println!("Result: {}", priority_sum());
}
