
use lazy_static::lazy_static;
use regex::Regex;

pub struct FunctionPrototype <'a> {
    return_type: &'a str,
    name: &'a str,
    args: [&'a str; 1],
}

pub fn get_prototype(line: &str) -> Result<FunctionPrototype, &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\w\d]+) ([\w\d]+) ?\((.+)\);").unwrap();
    }
    
    if false == RE.is_match(line) { 
        return Err("No match");
    }

    let captures = RE.captures(line).unwrap();

    let return_type = captures.get(1).map_or("", |m| m.as_str());
    let name = captures.get(2).map_or("", |m| m.as_str());
    let args = captures.get(3).map_or("", |m| m.as_str());

    let result = FunctionPrototype { 
        return_type:return_type,
        name:name,
        args:[args],
    };

    return Ok(result); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prototype_something() {
        let result = get_prototype("void something(void);").unwrap();

        assert_eq!(result.return_type, "void");
        assert_eq!(result.name, "something");
    }

    #[test]
    fn parse_prototype_test() {
        let result = get_prototype("int test(void);").unwrap();

        assert_eq!(result.return_type, "int");
        assert_eq!(result.name, "test");
    }

    #[test]
    fn parse_result_ok() {
        let result = get_prototype("int test(void);");
        assert!(result.is_ok());
    }
}
