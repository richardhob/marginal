
use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

mod utils;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    let check_result = utils::check(&args);

    if check_result.is_err() {
        return check_result;
    }

    let pattern = &args[1];
    let separator = pattern.chars().nth(1).unwrap();
    let split: Vec<&str> = pattern.split(separator).collect();

    if split.len() != 4 {
        return Err("Input pattern does not have the expected number of separator characters (expected 3)");
    }

    // Pick the functionality of regex based on the last characters
    let util_func = match split[3] {
        "g" => utils::replace_all,
        _ => utils::replace
    };

    // Pick the stream to read
    if args.len() == 2 {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let actual_line = match line {
                Err(msg) => panic!("Error reading stdin: {}", msg),
                Ok(contents) => contents,
            };
            let fixed_line = util_func(&actual_line, split[1], split[2]);
            print!("{}", fixed_line);
        }
    } else {
        let input_file = File::open(&args[2]).unwrap();
        let buffer = BufReader::new(input_file);
        for line in buffer.lines() {
            let actual_line = match line {
                Err(msg) => panic!("Error reading stdin: {}", msg),
                Ok(contents) => contents,
            };
            let fixed_line = util_func(&actual_line, split[1], split[2]);
            print!("{}", fixed_line);
        }
    };

    Ok(())
}
