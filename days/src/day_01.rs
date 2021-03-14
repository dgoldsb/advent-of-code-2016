use aoc::parse_items;
use std::collections::HashSet;

fn part_a(inputs: &Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut dir: u8 = 0;

    for input in inputs {
        let c = input.chars().next().unwrap();
        let blocks: i32 = input[1..].parse().unwrap();

        match c {
            'L' => dir = (dir + 1) % 4,
            'R' => dir = (dir + 3) % 4,
            _ => panic!("Invalid character!"),
        }

        match dir {
            0 => x += blocks,
            1 => y += blocks,
            2 => x -= blocks,
            3 => y -= blocks,
            _ => panic!("Invalid direction!"),
        }
    }

    x.abs() + y.abs()
}

fn part_b(inputs: &Vec<String>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: u8 = 0;
    let mut visited = HashSet::new();

    for input in inputs {
        let c = input.chars().next().unwrap();
        let blocks: i32 = input[1..].parse().unwrap();

        match c {
            'L' => dir = (dir + 1) % 4,
            'R' => dir = (dir + 3) % 4,
            _ => panic!("Invalid character!"),
        }

        for _ in 0..blocks {
            match dir {
                0 => x += 1,
                1 => y += 1,
                2 => x -= 1,
                3 => y -= 1,
                _ => panic!("Invalid direction!"),
            }

            if visited.contains(&(x, y)) {
                return x.abs() + y.abs();
            } else {
                visited.insert((x, y));
            }
        }
    }
    panic!("No solution for part B found...")
}

pub fn day_01() {
    let inputs = parse_items("day_01".to_string(), ", ".to_string());
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
