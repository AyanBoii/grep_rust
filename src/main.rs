use std::env;
use std::io;
use std::process;

fn main() {
    if env::args().len() < 3 || env::args().nth(1).unwrap() != "-E" {
        eprintln!("Expected first argument to be '-E'");
        eprintln!("Exiting with code 1");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line = input_line.trim_end().to_string();

    if match_pattern(&input_line, &pattern) {
        println!("Pattern matched. Exiting with code 0");
        process::exit(0);
    } else {
        println!("Pattern did not match. Exiting with code 1");
        process::exit(1);
    }
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    let mut input_iter = input.chars().peekable();
    let mut pattern_iter = pattern.chars().peekable();

    while pattern_iter.peek().is_some() {
        match pattern_iter.next() {
            Some('\\') => {
                match pattern_iter.next() {
                    Some('d') => {
                        match input_iter.next() {
                            Some(c) if c.is_ascii_digit() => {},
                            _ => return false,
                        }
                    },
                    Some('w') => {
                        match input_iter.next() {
                            Some(c) if c.is_alphanumeric() => {},
                            _ => return false,
                        }
                    },
                    Some('s') => {
                        match input_iter.next() {
                            Some(c) if c.is_whitespace() => {},
                            _ => return false,
                        }
                    },
                    Some(c) => {
                        match input_iter.next() {
                            Some(input_char) if input_char == c => {},
                            _ => return false,
                        }
                    },
                    None => return false,
                }
            },
            Some('[') => {
                let mut char_class = Vec::new();
                let mut negated = false;

                if let Some(&'^') = pattern_iter.peek() {
                    negated = true;
                    pattern_iter.next();
                }

                while let Some(&c) = pattern_iter.peek() {
                    pattern_iter.next();
                    if c == ']' {
                        break;
                    }
                    char_class.push(c);
                }

                match input_iter.next() {
                    Some(c) => {
                        let in_class = char_class.contains(&c);
                        if (negated && in_class) || (!negated && !in_class) {
                            return false;
                        }
                    },
                    None => return false,
                }
            },
            Some(pattern_char) => {
                match input_iter.next() {
                    Some(c) if c == pattern_char => {},
                    _ => return false,
                }
            },
            None => break,
        }
    }

    input_iter.next().is_none()
}