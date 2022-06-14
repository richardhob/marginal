
#[cfg(test)]
mod test_mkdir {
    use crate::mkdir;
    use std::path::Path;
    use std::fs;

    fn do_test(make_path: &'static str,
               remove_path: &'static str) {
        let path = Path::new(make_path);
        let rm = Path::new(remove_path);

        mkdir(path).unwrap();

        let info = match fs::metadata(path) {
            Ok(meta) => {
                fs::remove_dir_all(&rm).unwrap(); 
                meta},
            Err(e) => panic!("{}", e)
        };

        assert!(info.is_dir());
    }

    #[test]
    fn test_make_dir1() {
        do_test("./some/test/dir", "some");
    }

    #[test]
    fn test_make_dir2() {
        do_test("./some_test_dir", "some_test_dir");
    }
}
