use countmap::CountMap;
use std::collections::HashMap;
use std::hash::Hash;

fn memoize<A, K, R, F>(cache: &mut HashMap<K, R>, func: F, arg: A, key: K) -> R
where
    K: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<K, R>, A, K) -> R,
{
    match cache.get(&key).cloned() {
        Some(result) => result,
        None => {
            let result = func(cache, arg, key.clone());
            cache.insert(key, result.clone());
            result
        }
    }
}

fn count_permutations(cache: &mut HashMap<u32, u64>, adapters: &[u32], location: u32) -> u64 {
    match cache.get(&location) {
        Some(result) => *result,
        None => {
            if let Some((me, rest)) = adapters.split_first() {
                if rest.is_empty() {
                    1
                } else {
                    rest.iter()
                        .filter(|&&e| e <= me + 3)
                        .map(|p| {
                            let pos = rest.iter().position(|e| e == p).unwrap();
                            memoize(cache, count_permutations, &rest[pos..rest.len()], *p)
                        })
                        .sum()
                }
            } else {
                panic!("Error: Not a good list of adapters.");
            }
        }
    }
}

fn difference_calculation(adapters: &[u32]) {
    let mut differences: CountMap<u32> = CountMap::new();

    for (i, &adapter) in adapters[1..adapters.len()].iter().enumerate() {
        differences.insert_or_increment(adapter - adapters[i]);
    }

    eprintln!("Differences: {:?}", differences);
    eprintln!("Answer: {}", differences[&1] * differences[&3]);
}

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

    difference_calculation(&adapters);
    println!(
        "Number of permutations: {}",
        count_permutations(&mut HashMap::new(), &adapters, 0)
    );
}
