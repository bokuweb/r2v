#[macro_use]
extern crate bitfield;

mod cpu;
mod instruction;

use cpu::*;
// pub use instruction::*;

fn main() {
    let mut c = Cpu::new();
    c.init(0, &[0]);
}
