use progress_bar::color::{Color, Style};
use progress_bar::progress_bar::ProgressBar;
use std::collections::HashMap;

static MAX: u32 = 30000000;

pub fn calculate(input: &str) {
    let mut last_occurred: HashMap<u32, u32> = HashMap::new();
    let mut turn: u32 = 1;
    let mut next: u32 = 0;

    for line in input.split('\n') {
        if let Ok(num) = u32::from_str_radix(line, 10) {
            last_occurred.insert(num, turn);
            next = num;
            turn += 1;
        }
    }
    // remove last input number from hash
    last_occurred.remove(&next);
    turn -= 1;

    let mut bar = ProgressBar::new(MAX as usize);
    bar.set_action("Calculating...", Color::Green, Style::Normal);

    while turn <= MAX {
        let this = next;
        if let Some(occurrence) = last_occurred.get(&next) {
            next = turn - occurrence;
        } else {
            next = 0;
        }
        last_occurred.insert(this, turn);
        turn += 1;
        if turn % 100000 == 0 {
            bar.set_progression(turn as usize);
        }

        if turn == 2020 || turn == MAX {
            bar.print_info(
                "Success",
                &format!("{}: {}", turn, next),
                Color::Green,
                Style::Normal,
            );
        }
    }
}
