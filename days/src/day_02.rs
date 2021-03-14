use aoc::parse_chars;
use std::collections::HashMap;

fn part_a(inputs: &Vec<char>) -> String {
    let mut digit: i8 = 5;
    let mut code: String = "".to_string();

    for chr in inputs {
        digit = match chr {
            'U' => {
                if digit > 3 {
                    digit - 3
                } else {
                    digit
                }
            }
            'D' => {
                if digit < 7 {
                    digit + 3
                } else {
                    digit
                }
            }
            'L' => {
                if ((digit - 1) % 3) != 0 {
                    digit - 1
                } else {
                    digit
                }
            }
            'R' => {
                if (digit % 3) != 0 {
                    digit + 1
                } else {
                    digit
                }
            }
            '\n' => {
                code += &digit.to_string();
                digit
            }
            _ => panic!("Unsupported character"),
        };
    }
    code
}

struct Key {
    up: char,
    down: char,
    left: char,
    right: char,
}

fn part_b(inputs: &Vec<char>) -> String {
    let mut keys = HashMap::new();

    // Add all keys.
    keys.insert(
        '1',
        Key {
            up: '1',
            down: '3',
            left: '1',
            right: '1',
        },
    );
    keys.insert(
        '2',
        Key {
            up: '2',
            down: '6',
            left: '2',
            right: '3',
        },
    );
    keys.insert(
        '3',
        Key {
            up: '1',
            down: '7',
            left: '2',
            right: '4',
        },
    );
    keys.insert(
        '4',
        Key {
            up: '4',
            down: '8',
            left: '3',
            right: '4',
        },
    );
    keys.insert(
        '5',
        Key {
            up: '5',
            down: '5',
            left: '5',
            right: '6',
        },
    );
    keys.insert(
        '6',
        Key {
            up: '2',
            down: 'A',
            left: '5',
            right: '7',
        },
    );
    keys.insert(
        '7',
        Key {
            up: '3',
            down: 'B',
            left: '6',
            right: '8',
        },
    );
    keys.insert(
        '8',
        Key {
            up: '4',
            down: 'C',
            left: '7',
            right: '9',
        },
    );
    keys.insert(
        '9',
        Key {
            up: '9',
            down: '9',
            left: '8',
            right: '9',
        },
    );
    keys.insert(
        'A',
        Key {
            up: '6',
            down: 'A',
            left: 'A',
            right: 'B',
        },
    );
    keys.insert(
        'B',
        Key {
            up: '7',
            down: 'D',
            left: 'A',
            right: 'C',
        },
    );
    keys.insert(
        'C',
        Key {
            up: '8',
            down: 'C',
            left: 'B',
            right: 'C',
        },
    );
    keys.insert(
        'D',
        Key {
            up: 'B',
            down: 'D',
            left: 'D',
            right: 'D',
        },
    );

    // Iterate over the instructions.
    let mut digit: char = '5';
    let mut code: String = "".to_string();

    for chr in inputs {
        digit = match chr {
            'U' => keys[&digit].up,
            'D' => keys[&digit].down,
            'L' => keys[&digit].left,
            'R' => keys[&digit].right,
            '\n' => {
                code += &digit.to_string();
                digit
            }
            _ => panic!("Unsupported character"),
        };
    }
    return code;
}

pub fn day_02() {
    let inputs = parse_chars("day_02".to_string());
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
