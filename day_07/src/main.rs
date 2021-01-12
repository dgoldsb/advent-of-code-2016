use aoc::parse_lines;

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
}

fn part_a(inputs: &Vec<String>) -> usize {
    inputs
        .iter()
        .map(|a| IpAddress { address: a.clone() })
        .filter(|ip| ip.supports_tls())
        .count()
}

fn main() {
    let input = parse_lines();
    println!("A: {}", part_a(&input));
}
