mod registers;
mod cpu;
mod utils;

fn main() {
    let mut memory: [u8; 200] = [0;200];

    let mut cpu = cpu::Intel8080::new(&mut memory);
}
