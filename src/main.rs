mod registers;
mod cpu;
mod utils;
mod memory;

use std::io::{stdin,Read};

fn main() {
    let mut memory: memory::Memory<200> = memory::Memory::new();

    let mut cpu = cpu::Intel8080::new();

    let mut byte = [0u8];
    while let Ok(_) = stdin().read(&mut byte) {
        println!("CHAR {:?}", byte[0] as char);
    }
}
