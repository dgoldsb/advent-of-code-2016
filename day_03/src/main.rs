use aoc::parse_lines;
use regex::Regex;
use std::str::FromStr;

struct Shape {
    a: isize,
    b: isize,
    c: isize,
}

impl Shape {
    fn is_triangle(&self) -> bool {
        (self.a < (self.b + self.c)) && (self.b < (self.a + self.c)) && (self.c < (self.a + self.b))
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Shape, Self::Err> {
        let re = Regex::new(r"(\d+)\D?").unwrap();
        let ints: Vec<isize> = re
            .captures_iter(input)
            .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
            .collect();

        return Ok(Shape {
            a: ints[0],
            b: ints[1],
            c: ints[2],
        });
    }
}

fn part_a(inputs: &Vec<String>) -> usize {
    inputs
        .iter()
        .map(|l| Shape::from_str(l).unwrap())
        .filter(|s| s.is_triangle())
        .count()
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", part_a(&inputs));
}
