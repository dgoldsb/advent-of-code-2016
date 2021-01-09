use aoc::parse_chars;

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

fn main() {
    let inputs = parse_chars();
    println!("A: {}", part_a(&inputs));
}
