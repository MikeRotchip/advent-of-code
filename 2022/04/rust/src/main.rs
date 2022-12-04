use std::fs;

struct Interval {
    start: i32,
    end: i32,
}

struct Elf {
    input: String,
}

impl Elf {
    fn to_interval(&self) -> Interval {
        let coordinates: Vec<&str> = self.input.split("-").collect();

        return Interval {
            start: coordinates[0].parse::<i32>().unwrap(),
            end: coordinates[1].parse::<i32>().unwrap(),
        };
    }
}

fn main() {
    let input: String = fs::read_to_string("../input.txt").unwrap();

    let mut output: i32 = 0;

    for line in input.split("\n") {
        let intervals: Vec<Interval> = line_to_interval(line);

        if overlaps(&intervals[0], &intervals[1]) {
            output += 1;
        }
    }

    println!("{}", output);
}

fn overlaps(first: &Interval, second: &Interval) -> bool {
    return first.start <= second.end && first.end >= second.start;
}

fn line_to_interval(line: &str) -> Vec<Interval> {
    line.split(",")
        .map(|elf_input| Elf {
            input: elf_input.to_owned(),
        })
        .map(|elf| elf.to_interval())
        .collect()
}
