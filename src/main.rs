#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Usage example: adventofcode2020 day1 < input.txt");
        panic!();
    }

    let input = std::io::BufReader::new(std::io::stdin());

    match args[1].as_str() {
        "day1" => {
            let input = day1::read_input(input).unwrap();
            day1::day1(input.as_slice());
        },
        "day2" => {
            day2::run(input).unwrap();
        },
        day => {
            println!("Unknown day: {}", day);
        }
    }
}
