

mod cli {
    use std::process::Command;

    static EXE_PATH: &'static str = "./target/debug/sed";

    #[test]
    fn test_sed_fail_pass() {
        let out = Command::new(EXE_PATH)
            .arg("'s/fail/pass/'")
            .arg("'fail'")
            .output()
            .expect("Failed to execute sed");
        assert!(String::from_utf8_lossy(&out.stdout).contains("pass"));
    }
}
