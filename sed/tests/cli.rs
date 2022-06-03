

mod cli {
    use std::process::{Command, Stdio};
    use std::io::{Read, Write};

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
    fn test_sed_default() {
        pipe_pattern("fail", "s/fail/pass/g", "pass");
    }

    #[test]
    fn test_sed_single_replace() {
        pipe_pattern("fail fail fail", "s/fail/pass/", "pass fail fail");
    }

    #[test]
    fn test_sed_bad_separator() {
        let out = Command::new(EXE_PATH)
            .arg("'s/fail/'")
            .status()
            .expect("Bad Separator");
        assert!(out.code().unwrap() != 0);
    }
}
