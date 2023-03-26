mod registers;
mod cpu;
mod utils;

fn main() {
    let mut regs = registers::Registers::new();
    println!("Hello, world!");
    regs.set_pair_b(2000);
    println!("{}", regs.pair_b());
}
