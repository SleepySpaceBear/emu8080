mod registers;
mod cpu;
mod utils;

use std::io::{stdin,Read};

fn main() {
    let mut memory: [u8; 200] = [0;200];

    let mut cpu = cpu::Intel8080::new(&mut memory);

    let mut byte = [0u8];
    while let Ok(_) = stdin().read(&mut byte) {
        println!("CHAR {:?}", byte[0] as char);
    }
}
