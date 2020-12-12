#[macro_use]
extern crate lazy_static;
extern crate regex;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        }
        "day2" => {
            day2::run(input).unwrap();
        }
        "day3" => {
            day3::run(input).unwrap();
        }
        "day4" => {
            day4::run(input).unwrap();
        }
        "day5" => {
            day5::run(input).unwrap();
        }
        "day6" => {
            day6::run(input).unwrap();
        }
        "day7" => {
            day7::run(input).unwrap();
        }
        "day8" => {
            day8::run(input).unwrap();
        }
        "day9" => {
            day9::run(input).unwrap();
        }
        "day10" => {
            day10::run(input).unwrap();
        }
        "day11" => {
            day11::run(input).unwrap();
        }
        day => {
            println!("Unknown day: {}", day);
        }
    }
}
