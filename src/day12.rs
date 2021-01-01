use regex::Regex;

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Instruction {
    MoveDirection(Direction, i64),
    Rotate(i64),
    MoveForward(i64),
}

impl Instruction {
    fn execute(&mut self, ferry: &mut Ferry) {
        match self {
            Instruction::Rotate(deg) => ferry.rotate(*deg),
            Instruction::MoveForward(len) => ferry.sail(&ferry.direction.clone(), len),
            Instruction::MoveDirection(dir, len) => ferry.sail(dir, len),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Ferry {
    position: Position,
    direction: Direction,
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            position: Position { x: 0, y: 0 },
            direction: Direction::East,
        }
    }

    fn sail(&mut self, direction: &Direction, length: &i64) {
        let multiplier = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        self.position.x += multiplier.0 * length;
        self.position.y += multiplier.1 * length;
    }

    fn rotate(&mut self, deg: i64) {
        let mut directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let rotations = (deg / 90).rem_euclid(4);
        println!("Rotations: {}", rotations);
        while directions.first().unwrap() != &self.direction {
            directions.rotate_right(1);
        }
        directions.rotate_left(rotations as usize);
        self.direction = directions.first().unwrap().clone();
    }

    pub fn parse_instruction(&mut self, input: &str, debug: bool) {
        let re = Regex::new(r"([NESWLRF])(\d+)").unwrap();
        if let Some(cap) = re.captures(input) {
            let value = i64::from_str_radix(&cap[2], 10).unwrap();
            let instr = match &cap[1] {
                "N" => Some(Instruction::MoveDirection(Direction::North, value)),
                "S" => Some(Instruction::MoveDirection(Direction::South, value)),
                "E" => Some(Instruction::MoveDirection(Direction::East, value)),
                "W" => Some(Instruction::MoveDirection(Direction::West, value)),
                "L" => Some(Instruction::Rotate(-value)),
                "R" => Some(Instruction::Rotate(value)),
                "F" => Some(Instruction::MoveForward(value)),
                &_ => None,
            };

            if let Some(mut ins) = instr {
                ins.execute(self);
                if debug {
                    println!("Executed instruction {:?}, status: {:?}", ins, self);
                }
            }
        }
    }

    pub fn manhattan_distance_from_origin(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs()
    }
}

pub fn calculate(input: &str) {
    let mut ferry = Ferry::new();

    for line in input.split('\n') {
        ferry.parse_instruction(line, true);
    }
    println!(
        "Manhattan distance from (0,0): {}",
        ferry.manhattan_distance_from_origin()
    );
}
