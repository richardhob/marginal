
mod one {
    pub fn something() -> i32 {
        1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[path="./test_main.rs"]
mod test_main;
