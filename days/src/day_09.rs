use aoc::parse_chars;
use std::collections::VecDeque;
use std::vec::Vec;

const MARKER_START: char = '(';
const MARKER_END: char = ')';
const MARKER_SEP: char = 'x';

enum MarkerState {
    Unstarted,
    WidthDefining,
    MultiplierDefining,
    Finished,
}

struct Marker {
    width: Vec<char>,
    multiplier: Vec<char>,
    phase: MarkerState,
}

impl Marker {
    fn new() -> Marker {
        Marker {
            width: Vec::new(),
            multiplier: Vec::new(),
            phase: MarkerState::Unstarted,
        }
    }
}

fn part_a(input: &Vec<char>) -> usize {
    // Turn into a dequeue of characters.
    let mut queue: VecDeque<char> = VecDeque::new();
    queue.extend(input);

    // Make an empty uncompressed vector.
    let mut uncompressed: Vec<char> = Vec::new();

    // Reserve a bit of memory for the current marker.
    let mut current_marker: Marker = Marker::new();

    while queue.len() > 0 {
        let char = queue.pop_front().unwrap();
        match char {
            MARKER_START => current_marker.phase = MarkerState::WidthDefining,
            MARKER_SEP => current_marker.phase = MarkerState::MultiplierDefining,
            MARKER_END => {
                current_marker.phase = MarkerState::Finished;

                // Process the marker.
                let mut buffer: Vec<char> = Vec::new();
                for _ in 0..current_marker
                    .width
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap()
                {
                    buffer.push(queue.pop_front().unwrap());
                }
                for _ in 0..current_marker
                    .multiplier
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap()
                {
                    for c in &buffer {
                        uncompressed.push(*c);
                    }
                }

                current_marker = Marker::new();
            }
            _ => match current_marker.phase {
                MarkerState::Unstarted => uncompressed.push(char),
                MarkerState::WidthDefining => current_marker.width.push(char),
                MarkerState::MultiplierDefining => current_marker.multiplier.push(char),
                MarkerState::Finished => uncompressed.push(char),
            },
        }
    }

    return uncompressed.len();
}

fn find_length_recursively(mut queue: VecDeque<char>) -> usize {
    let mut counter: usize = 0;

    // Reserve a bit of memory for the current marker.
    let mut current_marker: Marker = Marker::new();

    while queue.len() > 0 {
        let char = queue.pop_front().unwrap();
        match char {
            MARKER_START => current_marker.phase = MarkerState::WidthDefining,
            MARKER_SEP => current_marker.phase = MarkerState::MultiplierDefining,
            MARKER_END => {
                current_marker.phase = MarkerState::Finished;

                // Process the marker.
                let mut subqueue: VecDeque<char> = VecDeque::new();
                for _ in 0..current_marker
                    .width
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap()
                {
                    subqueue.push_back(queue.pop_front().unwrap());
                }
                let multiplier: usize = current_marker
                    .multiplier
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                counter += find_length_recursively(subqueue) * multiplier;

                current_marker = Marker::new();
            }
            _ => match current_marker.phase {
                MarkerState::Unstarted => counter += 1,
                MarkerState::WidthDefining => current_marker.width.push(char),
                MarkerState::MultiplierDefining => current_marker.multiplier.push(char),
                MarkerState::Finished => counter += 1,
            },
        }
    }

    return counter;
}

fn part_b(input: &Vec<char>) -> usize {
    // Turn into a dequeue of characters.
    let mut queue: VecDeque<char> = VecDeque::new();
    queue.extend(input);

    return find_length_recursively(queue);
}

pub fn day_09() {
    let input = parse_chars("day_09".to_string());
    println!("A: {}", part_a(&input));
    println!("B: {}", part_b(&input));
}
