use itertools::Itertools;

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

pub fn calculate(input: &str) {
    let mut inputs = Vec::new();

    for line in input.lines() {
        inputs.push(line.parse::<u64>().unwrap());
    }

    println!("The product of 2 numbers: {}", find_product(&inputs, 2));
    println!("The product of 3 numbers: {}", find_product(&inputs, 3));
}
