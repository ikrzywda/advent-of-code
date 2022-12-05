use scanf::sscanf;
use std::io::{self, BufRead};

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn new_move(move_code: &str) -> Result<Move, &'static str> {
    match move_code {
        "A" | "X" => return Ok(Move::Rock),
        "B" | "Y" => return Ok(Move::Paper),
        "C" | "Z" => return Ok(Move::Scissors),
        _ => return Err("Invalid input"),
    };
}

fn get_result(rps_match: (Move, Move)) -> u32 {
    let (my_move, opponent_move) = rps_match;
    let result: u32;

    match my_move {
        Move::Rock => match opponent_move {
            Move::Paper => result = 0,
            Move::Rock => result = 3,
            Move::Scissors => result = 6,
        },
        Move::Paper => match opponent_move {
            Move::Scissors => result = 0,
            Move::Paper => result = 3,
            Move::Rock => result = 6,
        },
        Move::Scissors => match opponent_move {
            Move::Rock => result = 0,
            Move::Scissors => result = 3,
            Move::Paper => result = 6,
        },
    }
    return result + my_move as u32;
}

fn get_score() -> u32 {
    let mut net_score: u32 = 0;
    let mut my_input: String = String::new();
    let mut opponent_input: String = String::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if sscanf!(&line, "{} {}", opponent_input, my_input).is_ok() {
                    if let Some(my_move) = new_move(&my_input).ok().take() {
                        if let Some(opponent_move) = new_move(&opponent_input).ok().take() {
                            net_score += get_result((my_move, opponent_move));
                        }
                    }
                }
            }
            Err(_) => eprintln!("Invalid input"),
        }
    }
    return net_score;
}

fn main() {
    println!("Result: {}", get_score());
}
