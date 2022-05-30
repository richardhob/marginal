
use std::env;

mod parse {
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
    println!("{:?}", args);
    println!("pass");
}

#[cfg(test)]
#[path="./test_main.rs"]
mod test_main;
