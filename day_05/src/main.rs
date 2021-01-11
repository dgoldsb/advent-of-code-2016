use aoc::read_file;
use md5;

struct PasswordGenerator {
    base: String,
    cycle: usize,
}

impl Iterator for PasswordGenerator {
    type Item = (char, char);

    fn next(&mut self) -> Option<Self::Item> {
        let zeroes: String = "00000".to_string();
        loop {
            let undigested = format!("{}{}", self.base, self.cycle);
            let digest = format!("{:x}", md5::compute(undigested));
            self.cycle += 1;
            if digest[..5] == zeroes {
                return Option::Some((
                    digest.chars().nth(5).unwrap(),
                    digest.chars().nth(6).unwrap(),
                ));
            }
        }
    }
}

fn part_a(base: &String) -> String {
    PasswordGenerator {
        base: base.clone(),
        cycle: 0,
    }
    .take(8)
    .map(|i| i.0)
    .collect()
}

fn part_b(base: &String) -> String {
    let indices: Vec<usize> = (0..=7).collect();
    let mut found = Vec::new();
    let empty = '\t';
    let mut password: [char; 8] = [empty; 8];

    for t in (PasswordGenerator {
        base: base.clone(),
        cycle: 0,
    }) {
        match t.0.to_digit(10) {
            Some(i) => {
                if !found.contains(&i) && indices.contains(&(i as usize)) {
                    password[i as usize] = t.1;
                    found.push(i);
                }
            }
            _ => {}
        }
        if !password.contains(&empty) {
            break;
        }
    }
    return password.iter().collect();
}

fn main() {
    let input = read_file();
    println!("A: {}", part_a(&input));
    println!("B: {}", part_b(&input));
}
