use aoc::parse_lines;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

const CHAR_OFFSET: u32 = 97;

struct Room {
    name: String,
    section: u32,
}

impl Room {
    fn decipher(&self) -> String {
        let mut decoded = "".to_string();
        for c in self.name.chars() {
            if c == '-' {
                decoded += &c.to_string();
            } else {
                let new_char_id: u32 =
                    (((c as u32 - CHAR_OFFSET) + self.section) % 26) + CHAR_OFFSET;
                decoded += &((new_char_id as u8) as char).to_string();
            }
        }
        return decoded;
    }

    fn generate_checksum(&self) -> String {
        // Count how often each char occurs.
        let mut count = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }

            let i = match count.get(&c) {
                Some(i) => i + 1,
                None => 1,
            };
            count.insert(c, i);
        }

        // Collect to a vector.
        let mut pairs: Vec<(i32, char)> = count.iter().map(|t| (*t.1, *t.0)).collect();

        // Sort the vector.
        pairs.sort_by(|a, b| match a.0.cmp(&b.0).reverse() {
            Ordering::Equal => a.1.cmp(&b.1),
            other => other,
        });

        // Return the first 5 characters.
        let mut checksum = "".to_string();
        for i in 0..5 {
            checksum += &pairs[i].1.to_string();
        }

        return checksum;
    }
}

impl FromStr for Room {
    type Err = ();

    fn from_str(input: &str) -> Result<Room, Self::Err> {
        let re = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)]").unwrap();
        let cap = re.captures_iter(input).next().unwrap();
        let room = Room {
            name: cap[1].to_string(),
            section: cap[2].parse().unwrap(),
        };

        if room.generate_checksum() == cap[3].to_string() {
            return Ok(room);
        }

        return Err(());
    }
}

fn part_a(inputs: &Vec<String>) -> u32 {
    inputs
        .iter()
        .map(|l| Room::from_str(l))
        .filter(|s| s.is_ok())
        .map(|r| r.unwrap().section)
        .sum()
}

fn part_b(inputs: &Vec<String>) -> u32 {
    inputs
        .iter()
        .map(|l| Room::from_str(l))
        .filter(|s| s.is_ok())
        .map(|r| r.unwrap())
        .filter(|r| r.decipher().contains(&"north".to_string()))
        .next()
        .unwrap()
        .section
}

pub fn day_04() {
    let inputs = parse_lines("day_04".to_string());
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
