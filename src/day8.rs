#[derive(Clone, Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Instruction {
    fn from_str(s: &str) -> Option<Instruction> {
        if !s.contains(' ') {
            return None;
        }

        let words: Vec<&str> = s.split_whitespace().collect();
        let val = words[1].parse::<i64>().unwrap();
        match words[0] {
            "acc" => Some(Instruction::Acc(val)),
            "jmp" => Some(Instruction::Jmp(val)),
            "nop" => Some(Instruction::Nop(val)),
            &_ => None,
        }
    }
}

struct Computer {
    pc: usize,
    instructions: Vec<Instruction>,
    acc: i64,
    visited_instructions: Vec<usize>,
    exception: bool,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            instructions: Vec::new(),
            acc: 0,
            visited_instructions: Vec::new(),
            exception: false,
        }
    }

    fn run(&mut self) {
        while self.pc < self.instructions.len() && !self.exception {
            self.execute();
        }
    }

    fn execute(&mut self) {
        if self.visited_instructions.contains(&self.pc) {
            self.exception = true;
            return;
        }
        let old_pc = self.pc;
        let instr = self.instructions.get(self.pc).unwrap();
        match instr {
            Instruction::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }
            Instruction::Jmp(val) => {
                if *val < 0 {
                    self.pc -= val.abs() as usize
                } else {
                    self.pc += *val as usize
                }
            }
            Instruction::Nop(_val) => self.pc += 1,
        };
        self.visited_instructions.push(old_pc);
    }
}

fn find_loop(input: &str) {
    let mut comp = Computer::new();
    for line in input.split('\n') {
        let instr = Instruction::from_str(line);
        match instr {
            Some(i) => comp.instructions.push(i),
            None => eprintln!("Warning: '{}' is not an instruction", line),
        }
    }
    comp.run();
    if comp.exception {
        println!("Loop detected at instruction {}", comp.pc);
    }
    println!("Acc: {}", comp.acc);
}

fn find_mistake(input: &str) {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.split('\n') {
        let instr = Instruction::from_str(line);
        match instr {
            Some(i) => instructions.push(i),
            None => eprintln!("Warning: '{}' is not an instruction", line),
        }
    }

    for i in 0..instructions.len() {
        let mut our_instructions = instructions.clone();
        let replacement = match &instructions[i] {
            Instruction::Acc(val) => Instruction::Acc(*val),
            Instruction::Jmp(val) => Instruction::Nop(*val),
            Instruction::Nop(val) => Instruction::Jmp(*val),
        };
        our_instructions[i] = replacement;
        let mut c = Computer::new();
        c.instructions = our_instructions;
        c.run();
        if !c.exception {
            println!("Found it! Flipped instr {}, got {}", i, c.acc);
            break;
        }
    }
}

pub fn calculate(input: &str) {
    find_loop(input);
    find_mistake(input);
}
