use aoc::parse_lines;
use std::collections::HashMap;

const EMPTY: char = '\t';

fn solve(signals: &Vec<String>, part_a: &bool) -> String {
    let mut message: [char; 8] = [EMPTY; 8];

    for i in 0..8 {
        let mut frequencies = HashMap::new();
        for signal in signals {
            let c = signal.chars().nth(i).unwrap();
            let new_frequency = match frequencies.get(&c) {
                Some(i) => i + 1,
                None => 1,
            };
            frequencies.insert(c, new_frequency);
        }
        if *part_a {
            message[i] = *frequencies.iter().max_by_key(|t| t.1).unwrap().0;
        } else {
            message[i] = *frequencies.iter().min_by_key(|t| t.1).unwrap().0;
        }
    }

    return message.iter().collect();
}

fn main() {
    let input = parse_lines();
    println!("A: {}", solve(&input, &true));
    println!("B: {}", solve(&input, &false));
}
