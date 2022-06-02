
use regex::Regex;

pub fn replace(line: &str, pattern: &str, replacement: &str) -> String {
    let re_pattern = Regex::new(pattern).unwrap();
    re_pattern.replace_all(line, replacement).to_string()
}

pub fn check(args: &Vec<String>) -> Result<(), &'static str> {
    if args.len() < 2 {
        return Err("Expected at least one argument (Pattern)");
    }

    if args[1].len() < 2 {
        return Err("Expected Pattern argument to contain at least 2 characters");
    }

    Ok(())
}

#[cfg(test)]
#[path="./test_utils.rs"]
mod test_utils;
