use aoc::parse_lines;
use regex::Regex;
use std::collections::HashSet;

const MAX_X: u8 = 50;
const MAX_Y: u8 = 6;

enum Operation {
    Rect(u8, u8),
    RotateColumn(u8, u8),
    RotateRow(u8, u8),
}

struct Display {
    pixels: HashSet<(u8, u8)>,
}

impl Display {
    fn new() -> Display {
        Display {
            pixels: HashSet::new(),
        }
    }

    fn apply(&mut self, op: &Operation) {
        match op {
            Operation::Rect(x, y) => self.create_rectangle(x, y),
            Operation::RotateColumn(c, s) => self.rotate_column(c, s),
            Operation::RotateRow(r, s) => self.rotate_row(r, s),
        };
    }

    fn create_rectangle(&mut self, x: &u8, y: &u8) {
        for x_ in 0..*x {
            for y_ in 0..*y {
                self.pixels.insert((x_, y_));
            }
        }
    }

    fn rotate_column(&mut self, column: &u8, shift: &u8) {
        let new_pixels: HashSet<(u8, u8)> = self
            .pixels
            .iter()
            .filter(|&t| t.0 == *column)
            .map(|&t| (t.0, (t.1 + *shift) % MAX_Y))
            .collect();
        self.pixels.retain(|t| t.0 != *column);
        new_pixels.iter().for_each(|&t| {
            self.pixels.insert(t);
        });
    }

    fn rotate_row(&mut self, row: &u8, shift: &u8) {
        let new_pixels: HashSet<(u8, u8)> = self
            .pixels
            .iter()
            .filter(|&t| t.1 == *row)
            .map(|&t| ((t.0 + *shift) % MAX_X, t.1))
            .collect();
        self.pixels.retain(|t| t.1 != *row);
        new_pixels.iter().for_each(|&t| {
            self.pixels.insert(t);
        });
    }

    fn print(&self) -> String {
        let mut message = "".to_string();
        for y in 0..MAX_Y {
            for x in 0..MAX_X {
                if self.pixels.contains(&(x, y)) {
                    message += &'#'.to_string();
                } else {
                    message += &' '.to_string();
                }
            }
            message += &'\n'.to_string();
        }
        message
    }
}

fn get_ints(str: &String) -> (u8, u8) {
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();
    let caps: Vec<u8> = re
        .captures_iter(&str)
        .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
        .collect();
    (caps[0], caps[1])
}

fn parse_op(str: &String) -> Operation {
    let d = get_ints(str);
    if str.contains("rect") {
        return Operation::Rect(d.0, d.1);
    } else if str.contains("row") {
        return Operation::RotateRow(d.0, d.1);
    } else if str.contains("col") {
        return Operation::RotateColumn(d.0, d.1);
    } else {
        panic!("Invalid operation!");
    };
}

fn part_a(ops: &Vec<Operation>) -> usize {
    let mut display = Display::new();
    for op in ops {
        display.apply(&op);
    }
    println!("{}", display.print());
    display.pixels.len()
}

fn main() {
    let input = parse_lines().iter().map(|s| parse_op(s)).collect();
    println!("A: {}", part_a(&input));
}
