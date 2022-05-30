mod tests {
    use crate::parse;

    #[test]
    fn test_parse_lines_return() {
        let input: Vec<&str> = vec!["Test", "AnotherTest", "FirstTest"];
        let output: Vec<String> = parse::replace(input, "Test", "Pass");
        assert_eq!(output[0], "Pass");
        assert_eq!(output[1], "AnotherPass");
        assert_eq!(output[2], "FirstPass");
    }
}
