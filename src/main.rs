use VirtualMachine::cpu::cpu::CPU;
use VirtualMachine::mem::ram::RAM;

pub mod lib;
fn main() {

    // 4294967296 * 8 ~ 32MB
    // 0x0000'0000 <-> 0xffff'ffff
    let mut ram = RAM::new(4_294_967_296);

    let mut cpu = CPU::new();


}
