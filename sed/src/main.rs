
use std::env;

pub mod parse {
    use regex::Regex;

    pub fn replace<'a> (input: Vec<&str>, pattern: &str, replacement: &str) -> Vec<String> {
        let re_pattern = Regex::new(pattern).unwrap();
        let mut output: Vec<String> = vec![];

        for line in input.iter() {
            output.push(re_pattern.replace_all(line, replacement).to_string());
        }

        output
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let separator = pattern.chars().nth(1).unwrap();
    let split: Vec<&str> = pattern.split(separator).collect();

    println!("pass");
    println!("{}, {}, {}", separator, split[1], split[2]);
}

#[cfg(test)]
#[path="./test_main.rs"]
mod test_main;
