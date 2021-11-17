
use lazy_static::lazy_static;
use regex::Regex;

pub struct FunctionPrototype <'a> {
    return_type: &'a str,
    name: &'a str,
}

pub fn parse_string(line: &str) -> Result<FunctionPrototype, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\w\d]+) ([\w\d]+) ?\((.+)\);").unwrap();
    }
    
    if RE.is_match(line) { 
        let captures = RE.captures(line).unwrap();

        let rt = captures.get(1).map_or("", |m| m.as_str());
        let name = captures.get(2).map_or("", |m| m.as_str());

        let result = FunctionPrototype { 
            return_type:rt,
            name:name,
        };
        return Ok(result); 
    }
    else { return Err("No match"); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prototype_something() {
        let result = parse_string("void something(void);").unwrap();

        assert_eq!(result.return_type, "void");
        assert_eq!(result.name, "something");
    }

    #[test]
    fn parse_prototype_test() {
        let result = parse_string("int test(void);").unwrap();

        assert_eq!(result.return_type, "int");
        assert_eq!(result.name, "test");
    }

    #[test]
    fn parse_result_ok() {
        let result = parse_string("int test(void);");
        assert!(result.is_ok());
    }
}
