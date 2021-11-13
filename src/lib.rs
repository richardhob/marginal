
pub fn parse_string(line: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prototype() {
        let result = parse_string("int test(void);");
    }
}
