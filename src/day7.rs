use regex::Regex;
use std::collections::HashMap;

fn bags_containing<'a>(input: &'a str, bag: &str) -> Vec<&'a str> {
    let mut bags: HashMap<&str, Vec<&str>> = HashMap::new();
    let re = Regex::new(r"\d+ (\w+ \w+) bags?[,.]").unwrap();

    for bag in input.split('\n') {
        let parts = bag.split(" bags contain ").collect::<Vec<&str>>();
        bags.insert(parts[0], Vec::new());
    }

    for bag in input.split('\n') {
        let parts = bag.split(" bags contain ").collect::<Vec<&str>>();

        if parts.len() >= 2 {
            for cap in re.captures_iter(parts[1]) {
                let containee = bags.get_mut(&cap[1]).unwrap();
                containee.push(&parts[0]);
            }
        }
    }

    let mut candidate_bags = bags[bag].clone();
    let mut visited_bags: Vec<&str> = Vec::new();

    while !candidate_bags.is_empty() {
        let candidate = candidate_bags.pop().unwrap();
        if !visited_bags.contains(&candidate) {
            visited_bags.push(&candidate);
            candidate_bags.extend(bags[&candidate].iter().copied());
        }
    }
    visited_bags
}

fn bags_within(input: &str, bag: &str) -> usize {
    let r = format!("(?m)^{} bags contain .*$", bag);
    let re = Regex::new(&r).unwrap();
    let rec = re.find(input).unwrap();

    let re = Regex::new(r"\d+ \w+ \w+").unwrap();
    let next_item_re = Regex::new(r"(\d+) (\w+ \w+)").unwrap();

    re.captures_iter(&rec.as_str())
        .map(|cap| {
            let caps = next_item_re.captures(&cap[0]).unwrap();
            let n: usize = caps[1].parse().unwrap();
            n + n * bags_within(input, &caps[2])
        })
        .sum()
}

pub fn calculate(input: &str) {
    println!(
        "shiny gold might be in {} different bags",
        bags_containing(input, "shiny gold").len()
    );

    println!(
        "Shiny gold contains {} other bags",
        bags_within(input, "shiny gold")
    )
}
