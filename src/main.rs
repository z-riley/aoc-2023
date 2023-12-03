/*main.rs
Run files using `cargo run n` where n is the AoC day number
*/ 

use std::env;

mod common;
mod day_template;
mod day1;
mod day1_2;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1]; 

    match day.as_str() {
        "0" => day_template::main(),
        "1" => day1::main(),
        "1.2" => day1_2::main(),
        "2" => day2::main(),
        "3" => day3::main(),
        
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
}