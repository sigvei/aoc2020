use num::clamp;
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

    fn ch(&self) -> char {
        if self.occupied {
            '#'
        } else {
            'L'
        }
    }

    fn prepare_state(&mut self, occupied_neighbours: usize) {
        if !self.occupied && occupied_neighbours == 0 {
            self.next_state = true;
        } else if self.occupied && occupied_neighbours >= 4 {
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

    fn neighbors_of(&self, row: &usize, col: &usize) -> Vec<&Seat> {
        let mut nbs = Vec::new();
        let rows = &self.rows;
        let cols = &self.cols;
        let rmin = clamp(*row, 1, *rows) - 1;
        let rmax = clamp(*row, 0, *rows - 2) + 1;
        let cmin = clamp(*col, 1, *cols) - 1;
        let cmax = clamp(*col, 0, *cols - 2) + 1;
        for r in rmin..=rmax {
            for c in cmin..=cmax {
                if !(r == *row && c == *col) {
                    if let Some(seat) = self.seats.get(&(r, c)) {
                        nbs.push(seat);
                    }
                }
            }
        }
        nbs
    }

    fn tick(&mut self) {
        let prev_state = self.clone();
        for ((row, col), seat) in self.seats.iter_mut() {
            let n = prev_state
                .neighbors_of(row, col)
                .iter()
                .filter(|nb| nb.occupied)
                .count();
            seat.prepare_state(n);
            seat.commit_state();
        }
        self.ticks += 1;
    }

    fn print(&self) {
        println!("Grid: {}x{}, tick {}", self.cols, self.rows, self.ticks);
        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch: char = match self.seats.get(&(row, col)) {
                    Some(seat) => seat.ch(),
                    None => '.',
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

pub fn calculate(input: &str) {
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
        room.tick();
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
