use regex::Regex;
use std::fs;

fn read_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

pub fn parse_chars() -> Vec<char> {
    read_file().chars().collect()
}

pub fn parse_ints() -> Vec<isize> {
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();
    let input = read_file();
    re.captures_iter(&input)
        .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
        .collect()
}

pub fn parse_items(delimiter: String) -> Vec<String> {
    read_file().split(&delimiter).map(|i| i.to_string()).collect()
}

pub fn parse_lines() -> Vec<String> {
    read_file().split("\n").map(|i| i.to_string()).collect()
}
