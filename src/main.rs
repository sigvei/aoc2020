extern crate argparse;
use itertools::Itertools;

use argparse::{ArgumentParser, Store};
use std::fs;

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
        1 => day1(input),
        2..=24 => unimplemented!(),
        _ => {
            eprintln!("Day must be in range 1-24");
            std::process::exit(1);
        }
    }
}

fn find_product(inputs: &[u64], n: usize) -> u64 {
    let mut result = 0;
    for perm in inputs.iter().permutations(n) {
        if perm.clone().into_iter().sum::<u64>() == 2020 {
            result = perm.clone().into_iter().product();
            break;
        }
    }
    result
}

fn day1(input: &str) {
    let mut inputs = Vec::new();

    for line in input.lines() {
        inputs.push(line.parse::<u64>().unwrap());
    }

    println!("The product of 2 numbers: {}", find_product(&inputs, 2));
    println!("The product of 3 numbers: {}", find_product(&inputs, 3));
}
