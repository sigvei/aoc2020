use std::collections::HashSet;

#[derive(Debug, Clone)]
struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn from_bstring(bstr: &str) -> BoardingPass {
        let mut row_r = 128;
        let mut col_r = 8;
        let mut row_acc = 0;
        let mut col_acc = 0;

        bstr.chars().for_each(|cmd| match cmd {
            'F' => row_r /= 2,
            'B' => {
                row_r /= 2;
                row_acc += row_r
            }
            'L' => col_r /= 2,
            'R' => {
                col_r /= 2;
                col_acc += col_r
            }
            _ => (),
        });

        BoardingPass {
            row: row_acc,
            column: col_acc,
        }
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn find_missing(passes: &[BoardingPass], max_id: &usize) -> HashSet<usize> {
    let ids: HashSet<usize> = passes.iter().map(|p| p.seat_id()).collect();
    let all_ids: HashSet<usize> = (0..*max_id).collect();

    all_ids.difference(&ids).copied().collect::<HashSet<_>>()
}

pub fn calculate(input: &str) {
    let passes: Vec<BoardingPass> = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|bstr| BoardingPass::from_bstring(bstr))
        .collect();

    let max = passes
        .clone()
        .into_iter()
        .map(|p| p.seat_id())
        .max()
        .unwrap();
    println!("Max seat-ID: {}", max.clone());
    println!("Missing seat IDs: {:?}", find_missing(&passes, &max));
}
