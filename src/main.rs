extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::fs;

mod day1;

fn main() {
    let mut day = String::new();
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of code for a certain day");
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
        2..=24 => unimplemented!(),
        _ => {
            eprintln!("Day must be in range 1-24");
            std::process::exit(1);
        }
    }
}
