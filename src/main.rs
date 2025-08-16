use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } 
    if pattern.starts_with('[') && pattern.ends_with(']') {
        let chars = &pattern[1..pattern.len() - 1];
        return input_line.chars().any(|c| chars.contains(c));
    }
    match pattern {
        "\\w" => input_line.chars().any(|c| c.is_ascii_alphanumeric() || c == '_'),
        "\\d" => input_line.chars().any(|c| c.is_ascii_digit()),
        "\\s" => input_line.chars().any(|c| c.is_whitespace()),
        _ => panic!("Unhandled pattern: {}", pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line.trim_end(), &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
