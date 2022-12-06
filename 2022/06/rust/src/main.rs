use std::collections::HashMap;
use std::fs;

fn main() {
    let line: String = fs::read_to_string("../input.txt").unwrap();

    let mut seen_letters: HashMap<char, usize> = HashMap::new();
    let mut interval_start: usize = 0;

    let mut i: usize = 0;
    while i < line.len() {
        let existing_index = seen_letters
            .get(&line.chars().nth(i).unwrap())
            .unwrap_or(&usize::MAX);

        if existing_index == &usize::MAX {
            seen_letters.insert(line.chars().nth(i).unwrap(), i);

            if i - interval_start + 1 >= 14 {
                break;
            }
        } else {
            i = existing_index.to_owned();
            interval_start = i + 1;
            seen_letters.drain();
        }

        i += 1;
    }

    println!("{}", i);
}
