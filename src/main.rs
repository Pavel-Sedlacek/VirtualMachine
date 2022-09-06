use std::thread;
use VirtualMachine::cpu::cpu::CPU;
use VirtualMachine::mem::ram::RAM;

pub mod lib;
fn main() {

    // 4294967296 * 8 ~ 32GB
    // 0x0000'0000 <-> 0xffff'ffff
    let mut ram = RAM::new(4_294_967_296);

    let mut cpu = CPU::new();

    thread::spawn(move || {
        cpu.launch(&mut ram)
    });

    // thread::spawn(move || {
    //     cpu.launch(&mut ram)
    // });

}
