
mod test_replace_all {
    use crate::utils;

    #[test]
    fn default() {
        let input: &str = "AnotherTest";
        let output: String = utils::replace_all(input, "Test", "Pass");
        assert_eq!(output, "AnotherPass");
    }

    #[test]
    fn regex1() {
        let input: &str = "1234567890";
        let output: String = utils::replace_all(input, "\\d+", "pass");
        assert_eq!(output, "pass");
    }

    #[test]
    fn regex2() {
        let input: &str = "FailFailFailFailFail";
        let output: String = utils::replace_all(input, "Fail", "Pass");
        assert_eq!(output, "PassPassPassPassPass");
    }
}

mod test_replace {
    use crate::utils;

    #[test]
    fn default() {
        let input: &str = "AnotherTest";
        let output: String = utils::replace(input, "Test", "Pass");
        assert_eq!(output, "AnotherPass");
    }

    #[test]
    fn regex1() {
        let input: &str = "1234567890";
        let output: String = utils::replace(input, "\\d+", "pass");
        assert_eq!(output, "pass");
    }

    #[test]
    fn regex2() {
        let input: &str = "FailFailFailFailFail";
        let output: String = utils::replace(input, "Fail", "Pass");
        assert_eq!(output, "PassFailFailFailFail");
    }
}

mod test_check {
    use crate::utils;

    #[test]
    fn ok_pattern1() {
        let args: Vec<String> = vec!["sed".to_string(), 
            "s/123/234/g".to_string()];
        utils::check(&args).unwrap();
    }

    #[test]
    fn ok_pattern2() {
        let args: Vec<String> = vec!["sed".to_string(), 
            "sq222q222q".to_string()];
        utils::check(&args).unwrap();
    }

    #[test]
    fn no_pattern() {
        let args: Vec<String> = vec!["sed".to_string()];
        let result = utils::check(&args);
        assert!(result.is_err());
    }

    #[test]
    fn bad_pattern1() {
        let args: Vec<String> = vec!["sed".to_string(), 
            "".to_string()];
        let result = utils::check(&args);
        assert!(result.is_err());
    }
}
