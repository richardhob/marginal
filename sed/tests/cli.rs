

mod sed {
    use std::str;
    use std::process::{Command, Stdio};
    use std::io::{Read, Write};
    use std::fs::canonicalize;

    static EXE_PATH: &'static str = "./target/debug/sed";

    // Check out:
    //     https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html
    fn pipe_pattern(input: &str, pattern: &str, expected: &str) {
        let out = match Command::new(EXE_PATH)
                                .arg(pattern)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
            Err(msg) => panic!("Did not spawn {}: {}", EXE_PATH, msg),
            Ok(out) => out,
        };

        match out.stdin.unwrap().write_all(input.as_bytes()) {
            Err(msg) => panic!("Could not write to STDIN: {}", msg),
            Ok(_) => (),
        }

        let mut output = String::new();
        match out.stdout.unwrap().read_to_string(&mut output) {
            Err(msg) => panic!("Could not read STDOUT: {}", msg),
            Ok(_) => (),
        }

        if !output.eq(expected) {
            panic!("Error:\nCommand: {} > sed {}\nExpected: {}\nActual: {}", 
                   input, pattern, expected, output);
        }
    }

    #[test]
    fn test_default() {
        pipe_pattern("fail", "s/fail/pass/g", "pass\n");
    }

    #[test]
    fn test_word_char() {
        pipe_pattern("fail", "s/\\w+/pass/g", "pass\n");
    }

    #[test]
    fn test_single_replace() {
        pipe_pattern("fail fail fail", "s/fail/pass/", "pass fail fail\n");
    }

    #[test]
    fn test_bad_separator() {
        let out = Command::new(EXE_PATH)
            .arg("'s/fail/'")
            .status()
            .expect("Bad Separator");
        assert!(out.code().unwrap() != 0);
    }

    #[test]
    fn test_file_1() {
        let out = Command::new(EXE_PATH)
            .arg("s/test/pass/g")
            .arg(canonicalize("./tests/files/example01.txt").unwrap())
            .output()
            .expect("Could not create Process for cli");
        assert_eq!(str::from_utf8(&out.stdout).unwrap(), "pass\n\n");
    }
}
