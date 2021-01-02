extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let mut day = String::new();
    let mut input = String::new();
    let mut debug: bool = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of code for a certain day");
        ap.refer(&mut debug)
            .add_option(&["--debug", "-d"], StoreTrue, "Debug output");

        ap.refer(&mut day)
            .add_argument("day", Store, "Day to run")
            .required();

        ap.refer(&mut input)
            .add_argument("input", Store, "File with input data")
            .required();

        ap.parse_args_or_exit();
    }

    let nday: u8 = day.parse().unwrap();
    let input = fs::read_to_string(input).expect("Could not read input file");

    dispatch(&nday, &input);

    std::process::exit(0);
}

fn dispatch(day: &u8, input: &str) {
    match day {
        1 => day1::calculate(input),
        2 => day2::calculate(input),
        3 => day3::calculate(input),
        4 => day4::calculate(input),
        5 => day5::calculate(input),
        6 => day6::calculate(input),
        7 => day7::calculate(input),
        8 => day8::calculate(input),
        9 => day9::calculate(input),
        10 => day10::calculate(input),
        11 => day11::calculate(input),
        12 => day12::calculate(input),
        13 => day13::calculate(input),
        14 => day14::calculate(input),
        15 => day15::calculate(input),
        16..=24 => unimplemented!(),
        _ => {
            eprintln!("Day must be in range 1-24");
            std::process::exit(1);
        }
    }
}
