use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../input.txt").unwrap();

    let mut output: Vec<i32> = Vec::new();

    let a: Vec<&str> = input.split("\n").collect();

    for backpacks in a.chunks(3) {
        let mut group_item_counts: HashMap<char, i32> = HashMap::new();

        for backpack in backpacks {
            let mut backpack_items: HashSet<char> = HashSet::new();

            for item in backpack.chars() {
                if !backpack_items.contains(&item) {
                    backpack_items.insert(item);

                    let existing_count = group_item_counts.get(&item).unwrap_or(&0);

                    group_item_counts.insert(item, existing_count + 1);
                }
            }
        }

        output.push(get_priority(
            group_item_counts
                .keys()
                .find(|key| group_item_counts.get(key).unwrap() == &3)
                .unwrap(),
        ));
    }

    println!(
        "{}",
        output
            .iter()
            .map(|e| e.to_owned())
            .reduce(|acc, x| acc + x)
            .unwrap()
    )
}

fn get_priority(char: &char) -> i32 {
    let ascii = char.to_owned() as i32;

    let ascii_base = match ascii {
        _ if ascii <= 90 => 65 - 27,
        _ if ascii <= 122 => 97 - 1,
        _ => panic!(),
    };

    ascii - ascii_base
}
