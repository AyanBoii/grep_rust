use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => input_line.chars().any(|c| c.is_digit(10)),
        "\\w" => input_line.chars().any(|c| c.is_alphanumeric()),
        _ if pattern.chars().count() == 1 => input_line.contains(pattern),
        _ if pattern.starts_with('[') && pattern.ends_with(']') => {
            if pattern[1..].starts_with("^") {
                let chars_inside: Vec<char> = pattern[2..pattern.len() - 1].chars().collect();
                input_line.chars().any(|c| !chars_inside.contains(&c))
            } else {
                let chars_inside: Vec<char> = pattern[1..pattern.len() - 1].chars().collect();
                input_line.chars().any(|c| chars_inside.contains(&c))
            }
        }
        _ => panic!("Unhandled Pattern {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
        if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        println!("0");
        process::exit(0)
    } else {
        println!("1");
        process::exit(1)
    }
}