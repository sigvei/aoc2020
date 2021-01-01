use std::collections::HashMap;

#[derive(Clone)]
struct Seat {
    occupied: bool,
    next_state: bool,
}

impl Seat {
    fn new() -> Seat {
        Seat {
            occupied: false,
            next_state: false,
        }
    }

    // fn ch(&self) -> char {
    //     if self.occupied {
    //         '#'
    //     } else {
    //         'L'
    //     }
    // }

    fn prepare_state(&mut self, occupied_neighbours: usize, death_rule: usize) {
        if !self.occupied && occupied_neighbours == 0 {
            self.next_state = true;
        } else if self.occupied && occupied_neighbours >= death_rule {
            self.next_state = false;
        } else {
            self.next_state = self.occupied;
        }
    }

    fn commit_state(&mut self) {
        self.occupied = self.next_state;
    }
}

#[derive(Clone)]
struct WaitingRoom {
    seats: HashMap<(usize, usize), Seat>,
    ticks: usize,
    rows: usize,
    cols: usize,
}

impl WaitingRoom {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    fn new() -> WaitingRoom {
        WaitingRoom {
            rows: 0,
            cols: 0,
            seats: HashMap::new(),
            ticks: 0,
        }
    }

    fn add_seat(&mut self, row: usize, col: usize) {
        let s = Seat::new();
        self.seats.insert((row, col), s);
        if row + 1 > self.rows {
            self.rows = row + 1
        }
        if col + 1 > self.cols {
            self.cols = col + 1
        }
    }

    fn taken(&self) -> usize {
        self.seats.values().filter(|seat| seat.occupied).count()
    }

    fn out_of_bounds(&self, row: i32, col: i32) -> bool {
        // i32 means this will silently fail if rows or cols > 2**31
        row >= self.rows as i32 || row < 0 || col >= self.cols as i32 || col < 0
    }

    fn nearest_in_lines_from(&self, row: i32, col: i32, max_dist: Option<i32>) -> Vec<&Seat> {
        let mut nbs = Vec::new();
        for direction in &WaitingRoom::DIRECTIONS {
            let mut i: i32 = 0;
            loop {
                i += 1;
                if let Some(max) = max_dist {
                    if i > max {
                        break;
                    }
                }

                let pos = (row + direction.0 * i, col + direction.1 * i);
                if self.out_of_bounds(pos.0, pos.1) {
                    break;
                }
                if let Some(seat) = self.seats.get(&(pos.0 as usize, pos.1 as usize)) {
                    nbs.push(seat);
                    break;
                }
            }
        }
        nbs
    }

    fn tick(&mut self, distance_limit: Option<i32>, death_rule: usize) {
        let prev_state = self.clone();
        for ((row, col), seat) in self.seats.iter_mut() {
            let n = prev_state
                .nearest_in_lines_from(*row as i32, *col as i32, distance_limit)
                .iter()
                .filter(|nb| nb.occupied)
                .count();
            seat.prepare_state(n, death_rule);
            seat.commit_state();
        }
        self.ticks += 1;
    }

    // fn print(&self) {
    //     println!("Grid: {}x{}, tick {}", self.cols, self.rows, self.ticks);
    //     for row in 0..self.rows {
    //         for col in 0..self.cols {
    //             let ch: char = match self.seats.get(&(row, col)) {
    //                 Some(seat) => seat.ch(),
    //                 None => '.',
    //             };
    //             print!("{}", ch);
    //         }
    //         println!();
    //     }
    // }
}

fn run(input: &str, distance_limit: Option<i32>, death_rule: usize) {
    let mut room = WaitingRoom::new();

    for (row, line) in input.split('\n').enumerate() {
        if !line.is_empty() {
            for (col, c) in line.chars().enumerate() {
                if let 'L' = c {
                    room.add_seat(row, col);
                }
            }
        }
    }

    let mut prev_num;
    let mut this_num: usize = 0;
    loop {
        prev_num = this_num;
        room.tick(distance_limit, death_rule);
        this_num = room.taken();
        if prev_num == this_num {
            break;
        }
    }
    println!(
        "Stabilised at {} taken seats after {} ticks",
        this_num, room.ticks
    );
}

pub fn calculate(input: &str) {
    println!("Calculating for max_distance=1 and death rule=4:");
    run(input, Some(1), 4);
    println!("Calculating for max_distance=Inf and death rule=5:");
    run(input, None, 5);
}
