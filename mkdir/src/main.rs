
use std::io;
use std::fs;
use std::path::Path;
use std::env;

pub fn mkdir(dir: &Path) -> io::Result<()> {
    fs::create_dir_all(dir)
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No path specified");
    }

    if args.len() > 2 {
        return Err("Please specify one path");
    }

    for path in args.iter().skip(1) {
        let directory = Path::new(&path);
        let result = mkdir(directory);

        if result.is_err() {
            return Err("Could not make dir");
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;

    fn do_test(path: &'static str) {
        let test_path = Path::new(path);
        mkdir(test_path).unwrap();

        let info = match fs::metadata(test_path) {
            Ok(meta) => {
                fs::remove_dir_all(&test_path).unwrap(); 
                meta},
            Err(e) => panic!("{}", e)
        };

        assert!(info.is_dir());
    }

    #[test]
    fn test_make_dir1() {
        do_test("./some/test/dir");
    }

    #[test]
    fn test_make_dir2() {
        do_test("./some_test_dir");
    }
}
