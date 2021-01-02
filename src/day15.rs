use std::collections::HashMap;

pub fn calculate(input: &str) {
    let mut last_occurred: HashMap<u64, u64> = HashMap::new();
    let mut turn: u64 = 1;
    let mut next: u64 = 0;

    for line in input.split('\n') {
        if let Ok(num) = u64::from_str_radix(line, 10) {
            println!("{}: {}", turn, num);
            last_occurred.insert(num, turn);
            next = num;
            turn += 1;
        }
    }
    // remove last input number from hash
    last_occurred.remove(&next);
    turn -= 1;

    while turn <= 30000000 {
        let this = next;
        if let Some(occurrence) = last_occurred.get(&next) {
            next = turn - occurrence;
        } else {
            next = 0;
        }
        last_occurred.insert(this, turn);
        turn += 1;
        if turn == 2020 || turn == 30000000 || turn % 1000000 == 0 {
            println!("{}: {}", turn, next);
        }
    }
}
