

mod cli {
    use std::process::Command;

    static EXE_PATH: &'static str = "./target/debug/sed";

    #[test]
    fn test_sed_default() {
        let out = Command::new(EXE_PATH)
            .arg("'s/fail/pass/g'")
            .arg("'fail'")
            .output()
            .expect("Failed to execute sed");
        assert!(String::from_utf8_lossy(&out.stdout).contains("pass"));
    }

    #[test]
    fn test_sed_bad_separator() {
        let out = Command::new(EXE_PATH)
            .arg("'s/fail/'")
            .arg("'fail'")
            .status()
            .expect("Bad Separator");
        assert!(out.code().unwrap() != 0);
    }
}
