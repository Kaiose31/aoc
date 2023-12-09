use crate::helpers::{read_input, read_new};
mod helpers {
    mod reader;
    pub use reader::read_input;
    pub use reader::read_new;
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() == 2, "enter day number");

    let inputs = read_input(format!("inputs/day{}.txt", args[1]));
    match args[1].as_str() {
        "1" => {
            day1::solve1(inputs.clone());
            day1::solve2(inputs);
        }
        "2" => {
            day2::solve(inputs);
        }
        "3" => {
            day3::solve(inputs);
        }
        "4" => {
            day4::solve(inputs);
        }
        "5" => {
            day5::solve(read_new("inputs/day5.txt"));
        }
        "6" => {
            day6::solve(inputs);
        }
        "7" => {
            day7::solve(inputs);
        }
        "8" => {
            day8::solve(inputs);
        }
        "9" => {
            day9::solve(inputs);
        }
        _ => {}
    }
}
