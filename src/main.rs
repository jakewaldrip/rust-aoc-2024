mod days;
mod utils;

use days::{day01, day02, day03};
use std::env;
use std::fs::read_to_string;
use std::time::Instant;
use utils::solution::Solution;

pub type SolutionPair = (Solution, Solution);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        let time = Instant::now();

        let input_path = format!("inputs/day{}.txt", day);
        let input = read_to_string(input_path).expect("Failed to read input file");

        let (p1, p2) = func(&input);
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("  · Elapsed: {:.4} ms", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn(&str) -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        _ => unimplemented!(),
    }
}
