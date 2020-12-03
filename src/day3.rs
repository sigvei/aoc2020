#[derive(Debug)]
struct Line {
    trees: Vec<bool>,
}

impl Line {
    fn from_string(input: &str) -> Line {
        let trees = input.chars().map(|c| c == '#').collect();
        Line { trees }
    }

    fn get(&self, pos: usize) -> bool {
        *self.trees.get(pos % self.trees.len()).unwrap()
    }
}

fn trees_hit(lines: &[Line], deltax: usize, deltay: usize) -> usize {
    let mut xpos = 0;
    let mut ypos = 0;
    let mut trees = 0;
    while ypos < lines.len() {
        let line = &lines[ypos];

        if line.get(xpos) {
            trees += 1;
        }
        xpos += deltax;
        ypos += deltay;
    }
    trees
}

pub fn calculate(input: &str) {
    let lines: Vec<Line> = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|line| Line::from_string(&line))
        .collect();

    let candidates = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product: usize = 1;

    for (x, y) in candidates {
        let res = trees_hit(&lines, *x, *y);
        product *= res;
        println!("Trees hit with ({}, {}): {}", x, y, res)
    }
    println!("Product is {}", product);
}
