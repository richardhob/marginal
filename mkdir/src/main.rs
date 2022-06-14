
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
#[path="./test_mkdir.rs"]
mod test_mkdir;

