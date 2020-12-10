use countmap::CountMap;

pub fn calculate(input: &str) {
    let mut adapters: Vec<u32> = vec![0];

    adapters.extend(
        input
            .split('\n')
            .filter(|p| !p.is_empty())
            .map(|l| l.parse::<u32>().unwrap()),
    );

    adapters.sort_unstable();

    adapters.push(adapters.iter().max().unwrap() + 3);

    let mut differences: CountMap<u32> = CountMap::new();

    for (i, &adapter) in adapters[1..adapters.len()].iter().enumerate() {
        differences.insert_or_increment(adapter - adapters[i]);
    }

    eprintln!("Differences: {:?}", differences);
    eprintln!("Answer: {}", differences[&1] * differences[&3]);
}
