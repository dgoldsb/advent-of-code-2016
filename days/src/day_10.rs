use aoc::parse_lines;
use std::collections::HashMap;
use std::vec::Vec;

struct Bin {
    chips: Vec<usize>,
}

impl Bin {
    fn new() -> Bin {
        return Bin { chips: Vec::new() }
    }
}

struct Bot {
    bin: Bin,
    low_bot_id: String,
    high_bot_id: String,
}

impl Bot {
    fn new(low: String, high: String) -> Bot {
        return Bot { bin: Bin::new(), low_bot_id: low, high_bot_id: high}
    }

    fn divide(&mut self) -> (usize, usize) {
        let mut result = (0, 0);
        if self.bin.len() != 2 {
            panic!("Cannot divide more or less than 2 chips...");
        }
        else {
            if self.bin[0] < self.bin[1] {
                result = (self.bin[0], self.bin[1]);
            }
            result = (self.bin[1], self.bin[0]);
        }
        self.bin = Vec::new();
        return result;
    }
}

fn part_a(input: &Vec<String>) -> usize {
    // We keep track of bots here.
    let mut bots: HashMap<String, Bot> = HashMap::new();

    // We keep track of starting instructions here.
    let mut start: Vec<(String, usize)> = Vec::new();

    // TODO: Parse input.

    // Seed the system with chips.
    for seed in start {
        bots[seed.0].bin.push(seed.1);
    }

    // TODO: Run until all output bins have a chip.

    return 0;
}

pub fn day_10() {
    let input = parse_lines("day_10".to_string());
    println!("A: {}", part_a(&input));
}
