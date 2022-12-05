use std::collections::VecDeque;
use std::fs;

struct Storage {
    stacks: Vec<VecDeque<String>>,
}

impl Storage {
    fn move_crates(&mut self, instructions: Vec<i32>) {
        let mut moved_crates: VecDeque<String> = VecDeque::new();

        for e in (0..instructions[0]).map(|_| {
            self.stacks[instructions[1] as usize - 1]
                .pop_front()
                .unwrap()
        }) {
            moved_crates.push_front(e);
        }

        for _ in 0..instructions[0] {
            self.stacks[instructions[2] as usize - 1].push_front(moved_crates.pop_front().unwrap());
        }
    }
}

fn main() {
    let mut storage = Storage {
        stacks: vec![
            VecDeque::from([
                "N".to_owned(),
                "W".to_owned(),
                "F".to_owned(),
                "R".to_owned(),
                "Z".to_owned(),
                "S".to_owned(),
                "M".to_owned(),
                "D".to_owned(),
            ]),
            VecDeque::from([
                "S".to_owned(),
                "G".to_owned(),
                "Q".to_owned(),
                "P".to_owned(),
                "W".to_owned(),
            ]),
            VecDeque::from([
                "C".to_owned(),
                "J".to_owned(),
                "N".to_owned(),
                "F".to_owned(),
                "Q".to_owned(),
                "V".to_owned(),
                "R".to_owned(),
                "W".to_owned(),
            ]),
            VecDeque::from([
                "L".to_owned(),
                "D".to_owned(),
                "G".to_owned(),
                "C".to_owned(),
                "P".to_owned(),
                "Z".to_owned(),
                "F".to_owned(),
            ]),
            VecDeque::from(["S".to_owned(), "P".to_owned(), "T".to_owned()]),
            VecDeque::from([
                "L".to_owned(),
                "R".to_owned(),
                "W".to_owned(),
                "F".to_owned(),
                "D".to_owned(),
                "H".to_owned(),
            ]),
            VecDeque::from([
                "C".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
                "Z".to_owned(),
            ]),
            VecDeque::from([
                "Q".to_owned(),
                "J".to_owned(),
                "S".to_owned(),
                "V".to_owned(),
                "F".to_owned(),
                "R".to_owned(),
                "N".to_owned(),
                "W".to_owned(),
            ]),
            VecDeque::from([
                "V".to_owned(),
                "W".to_owned(),
                "Z".to_owned(),
                "G".to_owned(),
                "S".to_owned(),
                "M".to_owned(),
                "R".to_owned(),
            ]),
        ],
    };

    for line in fs::read_to_string("../input.txt").unwrap().split("\n") {
        println!("{}", line);
        storage.move_crates(line_to_instructions(line));
    }

    for mut stack in storage.stacks {
        print!("{}", stack.pop_front().unwrap());
    }
}

fn line_to_instructions(line: &str) -> Vec<i32> {
    let letters: Vec<&str> = line.split(" ").collect();

    vec![
        letters[1].parse().unwrap(),
        letters[3].parse().unwrap(),
        letters[5].parse().unwrap(),
    ]
}
