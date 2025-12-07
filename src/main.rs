#![recursion_limit = "64"]

use std::env;
use aoc25::days;

fn main() {
    println!("Advent of Code 2025");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input day");
        std::process::exit(1);
    }
    let (day, dayargs) = (&args[1], &args[2..]);
    match day.as_ref() {
        "day1" => days::day1::day1(dayargs),
        "day2" => days::day2::day2(dayargs),
        "day3" => days::day3::day3(dayargs),
        "day4" => days::day4::day4(dayargs),
        "day5" => days::day5::day5(dayargs),
        "day6" => days::day6::day6(dayargs),
        "day7" => days::day7::day7(dayargs),
        // "day8" => days::day8::day8(dayargs),
        // "day9" => days::day9::day9(dayargs),
        // "day10" => days::day10::day10(dayargs),
        // "day11" => days::day11::day11(dayargs),
        // "day12" => days::day12::day12(dayargs),
        _ => {
            println!("Unknown day: {}", day);
        }
    };
}