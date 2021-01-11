use aoc::read_file;
use md5;

struct PasswordGenerator {
    base: String,
    cycle: usize
}

impl Iterator for PasswordGenerator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let zeroes: String = "00000".to_string();
        loop {
            let undigested = format!("{}{}", self.base, self.cycle);
            let digest = format!("{:x}", md5::compute(undigested));
            self.cycle += 1;
            if digest[..5] == zeroes {
                return digest.chars().nth(5);
            }
        }
    }
}

fn part_a(base: &String) -> String {
    PasswordGenerator{ base: base.clone(), cycle: 0 }.take(8).collect()
}

fn main() {
    let input = read_file();
    println!("A: {}", part_a(&input));
}
