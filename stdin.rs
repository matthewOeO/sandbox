use std::io::{stdin, Read};

fn main() {
    loop {
        let mut character = [0];
        while let Ok(_) = stdin().read(&mut character) {
            println!("CHAR {:?}", character[0] as char);
        }
    }
}
