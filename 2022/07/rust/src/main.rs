use std::fs;
use std::str::Split;
use substring::Substring;

#[derive(Clone)]
struct Dir {
    name: String,

    size: i32,
    file_size: i32,

    sub_dirs: Vec<Dir>,
}

impl Dir {
    fn add_sub_dir(&mut self, new_dir: Dir) {
        self.sub_dirs.push(new_dir);
    }
}

fn main() {
    let mut root: Dir = Dir {
        name: "/".to_owned(),
        size: 0,
        file_size: 0,
        sub_dirs: vec![],
    };

    let mut current_dir: &mut Dir = &mut root;

    handle_dir(
        &mut fs::read_to_string("../input.txt").unwrap().split("\n"),
        &mut current_dir,
    );

    count_sizes(&mut root);

    let space_to_free: i32 = 30_000_000 - (70_000_000 - root.size);

    println!("{}", find_smallest_of_size(&mut root, &space_to_free),)
}

fn handle_dir(line_iter: &mut Split<&str>, mut current_dir: &mut Dir) {
    while let Some(line) = line_iter.next() {
        if line.substring(0, 3) == "dir" {
            let new_dir: Dir = Dir {
                name: line.split(" ").collect::<Vec<&str>>()[1 as usize].to_owned(),

                size: 0,
                file_size: 0,

                sub_dirs: vec![],
            };

            current_dir.add_sub_dir(new_dir);
        } else if line.chars().nth(0).unwrap().is_numeric() {
            current_dir.file_size += line.split(" ").next().unwrap().parse::<i32>().unwrap();
        } else if line.substring(0, 4) == "$ cd" {
            let target_dir = line.split(" ").collect::<Vec<&str>>()[2 as usize].to_owned();

            if target_dir == ".." {
                return;
            }

            let target_dir_idx = current_dir
                .sub_dirs
                .iter()
                .position(|dir| dir.name == target_dir)
                .unwrap();

            handle_dir(line_iter, &mut current_dir.sub_dirs[target_dir_idx]);
        }
    }

    return;
}

fn count_sizes(current_dir: &mut Dir) {
    for sub_dir in &mut current_dir.sub_dirs {
        count_sizes(sub_dir);
    }

    current_dir.size = current_dir.file_size
        + current_dir
            .sub_dirs
            .iter()
            .map(|dir| dir.size)
            .reduce(|x, acc| x + acc)
            .unwrap_or(0);
}

fn find_smallest_of_size(current_dir: &mut Dir, min_size: &i32) -> i32 {
    let mut current_smallest = current_dir.size;

    for i in 0..current_dir.sub_dirs.len() {
        let smallest_in_subdir = find_smallest_of_size(&mut current_dir.sub_dirs[i], min_size);

        if smallest_in_subdir <= current_smallest && smallest_in_subdir >= min_size.to_owned() {
            current_smallest = smallest_in_subdir;
        }
    }

    return current_smallest;
}
