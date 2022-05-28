
mod one {
    pub fn something() -> i32 {
        1
    }
}

fn main() {
    println!("fail");
}

#[cfg(test)]
#[path="./test_main.rs"]
mod test_main;
