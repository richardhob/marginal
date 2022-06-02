
use std::env;

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

    utils::replace("This is a test", split[1], split[2]);

    println!("pass");
    println!("{}, {}, {}", separator, split[1], split[2]);

    Ok(())
}
