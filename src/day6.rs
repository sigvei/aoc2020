use std::collections::HashSet;

#[derive(Debug)]
struct Answer {
    answered: HashSet<char>,
}

impl Answer {
    fn from_str(s: &str) -> Answer {
        Answer {
            answered: s.chars().collect(),
        }
    }

    fn intersect(&self, a: &Answer) -> Answer {
        let set: HashSet<char> = self.answered.intersection(&a.answered).copied().collect();
        Answer { answered: set }
    }
}

impl Clone for Answer {
    fn clone(&self) -> Answer {
        Answer {
            answered: self.answered.clone(),
        }
    }
}

#[derive(Debug)]
struct Group {
    answers: Vec<Answer>,
}

impl Group {
    fn from_str(s: &str) -> Group {
        let ans: Vec<Answer> = s.split('\n').map(|line| Answer::from_str(line)).collect();
        Group { answers: ans }
    }

    fn unique_answers(&self) -> usize {
        let mut set: HashSet<char> = HashSet::new();
        for ans in &self.answers {
            set.extend(&ans.answered);
        }
        set.len()
    }

    fn intersection_of_answers(&self) -> HashSet<char> {
        if self.answers.len() == 1 {
            return self.answers.first().unwrap().answered.clone();
        } else {
            let mut answers = self.answers.clone();
            let first = answers.pop().unwrap();
            let second = answers.pop().unwrap();
            let mut init = first.intersect(&second);
            for ans in &self.answers {
                init = init.intersect(ans);
            }
            init.answered
        }
    }

    fn n_answers(&self) -> usize {
        self.intersection_of_answers().len()
    }
}

impl Clone for Group {
    fn clone(&self) -> Group {
        Group {
            answers: self.answers.clone(),
        }
    }
}

pub fn calculate(input: &str) {
    let groups: Vec<Group> = input
        .strip_suffix('\n')
        .unwrap()
        .split("\n\n")
        .map(|g| Group::from_str(g))
        .collect();

    for group in groups.clone() {
        eprintln!("Group: {} - {:?}", group.n_answers(), group);
    }

    let sum_of_answers: usize = groups
        .clone()
        .into_iter()
        .map(|g| g.unique_answers())
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    let n_diff_answers: usize = groups
        .into_iter()
        .map(|g| g.n_answers())
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("Sum of unique answers: {}", sum_of_answers);
    println!("Sum of unanimous answers in groups: {}", n_diff_answers);
}
