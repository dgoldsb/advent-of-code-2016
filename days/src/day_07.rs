use aoc::parse_lines;
use std::collections::HashSet;

const EMPTY: char = '\t';

struct IpAddress {
    address: String,
}

impl IpAddress {
    fn supports_tls(&self) -> bool {
        let mut layer: usize = 0;
        let mut buffer: [char; 3] = [EMPTY; 3];

        let mut found_good = false;
        let mut found_bad = false;

        for c in self.address.chars() {
            match c {
                '[' => {
                    layer += 1;
                    buffer = [EMPTY; 3];
                }
                ']' => {
                    layer -= 1;
                    buffer = [EMPTY; 3];
                }
                _ => {
                    if (layer == 0)
                        && (c == buffer[0])
                        && (buffer[1] == buffer[2])
                        && (c != buffer[1])
                    {
                        found_good = true;
                    } else if (layer > 0)
                        && (c == buffer[0])
                        && (buffer[1] == buffer[2])
                        && (c != buffer[1])
                    {
                        found_bad = true;
                    }

                    buffer[0] = buffer[1];
                    buffer[1] = buffer[2];
                    buffer[2] = c;
                }
            }
        }
        found_good && !found_bad
    }

    fn supports_ssl(&self) -> bool {
        let mut layer: usize = 0;
        let mut buffer: [char; 2] = [EMPTY; 2];

        let mut found_super = HashSet::new();
        let mut found_hyper = HashSet::new();

        for c in self.address.chars() {
            match c {
                '[' => {
                    layer += 1;
                    buffer = [EMPTY; 2];
                }
                ']' => {
                    layer -= 1;
                    buffer = [EMPTY; 2];
                }
                _ => {
                    if (layer == 0) && (c == buffer[0]) && (c != buffer[1]) {
                        found_super.insert((buffer[0], buffer[1]));
                    } else if (layer > 0) && (c == buffer[0]) && (c != buffer[1]) {
                        found_hyper.insert((buffer[1], buffer[0]));
                    }

                    buffer[0] = buffer[1];
                    buffer[1] = c;
                }
            }
        }
        found_hyper.intersection(&found_super).count() > 0
    }
}

fn part_a(inputs: &Vec<String>) -> usize {
    inputs
        .iter()
        .map(|a| IpAddress { address: a.clone() })
        .filter(|ip| ip.supports_tls())
        .count()
}

fn part_b(inputs: &Vec<String>) -> usize {
    inputs
        .iter()
        .map(|a| IpAddress { address: a.clone() })
        .filter(|ip| ip.supports_ssl())
        .count()
}

pub fn day_07() {
    let input = parse_lines("day_07".to_string());
    println!("A: {}", part_a(&input));
    println!("B: {}", part_b(&input));
}
