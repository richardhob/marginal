
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_string(line: &str) -> Result<i32, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*").unwrap();
    }
    if RE.is_match(line) { return Ok(0); }
    else { return Err("No match"); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prototype() {
        let result = parse_string("int test(void);").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn parse_result_ok() {
        let result = parse_string("int test(void);");
        assert!(result.is_ok());
    }
}
