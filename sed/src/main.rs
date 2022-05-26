
mod one {
    pub fn something() {
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[path="./test_main.rs"]
mod test_main;
