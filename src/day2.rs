use countmap::CountMap;
use regex::Regex;

pub fn calculate(input: &str) {
    first_interpretation(input);
    second_interpretation(input);
}

fn first_interpretation(input: &str) {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut valids = 0;

    for line in re.captures_iter(input) {
        let min = &line[1].parse::<u64>().unwrap();
        let max = &line[2].parse::<u64>().unwrap();
        let chr: char = line[3].chars().collect::<Vec<char>>()[0];
        let pass = &line[4];

        let mut counts: CountMap<char> = CountMap::new();

        for c in pass.chars() {
            counts.insert_or_increment(c);
        }

        let count = counts.get_count(&chr).unwrap_or(0);

        if &count < min || &count > max {
            eprintln!("Invalid password: {} {} {} {}", min, max, chr, pass);
        } else {
            valids += 1;
        }
    }
    println!("Valid passwords, interpretation 1: {}", valids);
}

fn second_interpretation(input: &str) {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut valids = 0;

    for line in re.captures_iter(input) {
        let p1 = line[1].parse::<usize>().unwrap();
        let p2 = line[2].parse::<usize>().unwrap();
        let chr: char = line[3].chars().collect::<Vec<char>>()[0];
        let pass = &line[4].chars().collect::<Vec<char>>();

        if p1 <= pass.len() && p2 <= pass.len() && ((pass[p1 - 1] == chr) ^ (pass[p2 - 1] == chr)) {
            valids += 1;
        }
    }
    println!("Valid passwords, interpretation 2: {}", valids);
}
