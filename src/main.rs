mod chip8;

use chip8::CPU;
use std::sync::{Arc, atomic::{AtomicU8}};

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        i_register: 0,
        sound_timer_register: Arc::new(AtomicU8::new(0)),
        delay_timer_register: Arc::new(AtomicU8::new(0)),
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // 
    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00; 
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14; 
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + 10 + 10 + 10 + 10 = {0}", cpu.registers[0]); 
}
