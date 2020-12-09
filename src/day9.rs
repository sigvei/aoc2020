fn is_valid_for(n: u64, src: &[u64]) -> bool {
    for (index, a) in src[0..(src.len() - 1)].iter().enumerate() {
        for b in src[(index + 1)..src.len()].iter() {
            if a + b == n {
                return true;
            }
        }
    }
    false
}

fn find_invalid_number(numbers: &[u64]) -> Option<u64> {
    for (i, num) in numbers[25..numbers.len()].iter().enumerate() {
        // A bit opaque here. Enumerate numbers the items 25.. as 0..
        // so the correct slice of 25 numbers before our current num
        // is our current num..current_num + 25
        if !is_valid_for(*num, &numbers[i..i + 25]) {
            return Some(*num);
        }
    }
    None
}

fn find_consecutive_sum(numbers: &[u64], equal_to: u64) -> Option<&[u64]> {
    for (i, _num) in numbers.iter().enumerate() {
        for j in i..numbers.len() {
            let nums = &numbers[i..j];
            if nums.iter().sum::<u64>() == equal_to {
                return Some(nums);
            }
        }
    }
    None
}

pub fn calculate(input: &str) {
    let numbers: Vec<u64> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    match find_invalid_number(&numbers) {
        Some(n) => {
            println!("Invalid number found: {}", n);
            if let Some(cons) = find_consecutive_sum(&numbers, n) {
                println!("Consecutive numbers adding up to {}: {:?}", n, cons);
                println!(
                    "Sum of min and max: {}",
                    cons.iter().max().unwrap() + cons.iter().min().unwrap()
                );
            } else {
                println!("No consecutive run of integers gave this sum.");
            }
        }
        None => println!("No invalid number found. Stopping."),
    }
}
