use regex::Regex;

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn coordinates(&self) -> (i64, i64) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    MoveDirection(Direction, i64),
    Rotate(i64),
    MoveForward(i64),
}

impl Instruction {
    fn parse(cmd: &str) -> Option<Instruction> {
        let re = Regex::new(r"([NESWLRF])(\d+)").unwrap();
        if let Some(cap) = re.captures(cmd) {
            let value = i64::from_str_radix(&cap[2], 10).unwrap();
            match &cap[1] {
                "N" => Some(Instruction::MoveDirection(Direction::North, value)),
                "S" => Some(Instruction::MoveDirection(Direction::South, value)),
                "E" => Some(Instruction::MoveDirection(Direction::East, value)),
                "W" => Some(Instruction::MoveDirection(Direction::West, value)),
                "L" => Some(Instruction::Rotate(-value)),
                "R" => Some(Instruction::Rotate(value)),
                "F" => Some(Instruction::MoveForward(value)),
                &_ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

struct WaypointFerry {
    ferry: Ferry,
    waypoint: Position,
}

impl WaypointFerry {
    fn new() -> WaypointFerry {
        WaypointFerry {
            ferry: Ferry::new(),
            waypoint: Position { x: 10, y: -1 },
        }
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Rotate(deg) => self.rotate_waypoint(deg),
            Instruction::MoveForward(len) => self.sail(len),
            Instruction::MoveDirection(dir, len) => self.move_waypoint(dir, len),
        }
    }

    fn sail(&mut self, len: i64) {
        self.ferry.position.x += self.waypoint.x * len;
        self.ferry.position.y += self.waypoint.y * len;
    }

    fn rotate_waypoint(&mut self, deg: i64) {
        let rotations = Ferry::deg_to_rotations(deg);
        for _ in 0..rotations {
            self.waypoint = Position {
                x: -self.waypoint.y,
                y: self.waypoint.x,
            }
        }
    }

    fn move_waypoint(&mut self, direction: Direction, len: i64) {
        let multiplier = direction.coordinates();

        self.waypoint.x += multiplier.0 * len;
        self.waypoint.y += multiplier.1 * len;
    }
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

    fn deg_to_rotations(deg: i64) -> usize {
        (deg / 90).rem_euclid(4) as usize
    }

    fn sail(&mut self, direction: Direction, length: i64) {
        let multiplier = direction.coordinates();

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
        let rotations = Ferry::deg_to_rotations(deg);

        while directions.first().unwrap() != &self.direction {
            directions.rotate_right(1);
        }
        directions.rotate_left(rotations as usize);
        self.direction = directions.first().unwrap().clone();
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rotate(deg) => self.rotate(deg),
            Instruction::MoveForward(len) => self.sail(self.direction.clone(), len),
            Instruction::MoveDirection(dir, len) => self.sail(dir, len),
        }
    }

    pub fn manhattan_distance_from_origin(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs()
    }
}

pub fn calculate(input: &str) {
    let mut ferry = Ferry::new();
    let mut waypointferry = WaypointFerry::new();

    for line in input.split('\n') {
        if let Some(instr) = Instruction::parse(line) {
            ferry.execute(instr.clone());
            waypointferry.execute(instr);
        }
    }
    println!(
        "Manhattan distance from (0,0) for regular ferry, waypoint ferry: {}, {}",
        ferry.manhattan_distance_from_origin(),
        waypointferry.ferry.manhattan_distance_from_origin()
    );
}
