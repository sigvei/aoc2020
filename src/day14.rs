use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl Mask {
    fn from_str(mask: &str) -> Mask {
        // we or with the 1s and and with the inverse of
        // the 0s
        let or_mask = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();

        let and_mask =
            u64::from_str_radix(&format!("{:1>64}", &mask.replace('X', "1")), 2).unwrap();

        Mask { or_mask, and_mask }
    }

    fn mask(&self, num: u64) -> u64 {
        (num | self.or_mask) & self.and_mask
    }
}

fn part1(input: &str) {
    let mut current_mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let memset_re = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let maskset_re = Regex::new(r"^mask = ([X01]{36})$").unwrap();

    for line in input.split('\n') {
        if let Some(m) = maskset_re.captures(line) {
            current_mask = Mask::from_str(m.get(1).unwrap().as_str());
        } else if let Some(m) = memset_re.captures(line) {
            let addr = usize::from_str_radix(m.get(1).unwrap().as_str(), 10).unwrap();
            let n = u64::from_str_radix(m.get(2).unwrap().as_str(), 10).unwrap();
            mem.insert(addr, current_mask.mask(n));
        }
    }

    println!("Sum of written values: {}", mem.values().sum::<u64>());
}

struct Memory {
    store: HashMap<u64, u64>,
    mask: String,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            store: HashMap::new(),
            mask: format!("{:36}", "0"),
        }
    }

    fn store_from_str(&mut self, addr: u64, value: u64) {
        self.store_inner(
            self.mask.clone().as_str(),
            &value,
            "".to_owned(),
            &format!("{:036b}", addr),
        );
    }

    fn store_inner(&mut self, mask: &str, value: &u64, completed: String, remain: &str) {
        if remain.is_empty() {
            let addr = u64::from_str_radix(&completed, 2).unwrap();
            self.store.insert(addr, *value);
        } else {
            let (this_mask, remain_mask) = mask.split_at(1);
            let (this_addr, remain_addr) = remain.split_at(1);
            match this_mask {
                "1" => self.store_inner(remain_mask, value, completed + "1", remain_addr),
                "0" => self.store_inner(remain_mask, value, completed + this_addr, remain_addr),
                "X" => {
                    self.store_inner(remain_mask, value, completed.clone() + "1", remain_addr);
                    self.store_inner(remain_mask, value, completed + "0", remain_addr);
                }
                _ => (),
            }
        }
    }

    fn sum(&self) -> u64 {
        self.store.values().sum()
    }
}

fn part2(input: &str) {
    let mut mem = Memory::new();

    let memset_re = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let maskset_re = Regex::new(r"^mask = ([X01]{36})$").unwrap();

    for line in input.split('\n') {
        if let Some(m) = maskset_re.captures(line) {
            mem.mask = m.get(1).unwrap().as_str().to_owned();
        } else if let Some(m) = memset_re.captures(line) {
            let addr = u64::from_str_radix(m.get(1).unwrap().as_str(), 10).unwrap();
            let n = u64::from_str_radix(m.get(2).unwrap().as_str(), 10).unwrap();
            mem.store_from_str(addr, n);
        }
    }

    println!("Sum of written values: {}", mem.sum());
}

pub fn calculate(input: &str) {
    part1(input);
    part2(input);
}
