mod sound;
mod keyboard;

use sound::SoundPlayer;
use rand::Rng;
use std::{thread, time::{self, Duration}};
use std::sync::{Arc, atomic::{AtomicU8, Ordering}};

const TIMER_DELAY: Duration = time::Duration::from_millis(16);

pub struct CPU {
    pub registers: [u8; 16],
    pub i_register: u16,
    pub sound_timer_register: Arc<AtomicU8>,
    pub delay_timer_register: Arc<AtomicU8>,
    pub position_in_memory: usize,
    pub memory: [u8; 4096],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl CPU {

    fn run_sound_timer_loop(sound_timer_register: Arc<AtomicU8>)
    {
        let mut sound = SoundPlayer::new();

        loop {
            let value = sound_timer_register.load(Ordering::Relaxed);
            if value != 0
            {
                sound.ensure_playing();
                sound_timer_register.fetch_sub(1, Ordering::Relaxed);
                thread::sleep(TIMER_DELAY);
            }

            sound.ensure_stopped();
        }
    }

    fn run_delay_timer_loop(delay_timer_register: Arc<AtomicU8>)
    {
        loop {
            let value = delay_timer_register.load(Ordering::Relaxed);
            if value != 0
            {
                delay_timer_register.fetch_sub(1, Ordering::Relaxed);
                thread::sleep(TIMER_DELAY);
            }
        }
    }

    pub fn run(&mut self) {
        
        let sound_timer_register_clone = self.sound_timer_register.clone();
        thread::spawn(move || {
            CPU::run_sound_timer_loop(sound_timer_register_clone);
        });

        let delay_timer_register_clone = self.delay_timer_register.clone();
        thread::spawn(move || {
            CPU::run_delay_timer_loop(delay_timer_register_clone);
        });

        loop {          
            let  opcode = self.read_opcode();
            self.position_in_memory += 2;

            match (opcode) {
                0x0000 => { return; } // For now: stop cpu run to go through complete memory
                0x00E0 => { } // Clear the display
                0x00EE => { self.ret(); }, // Return from a subroutine
                0x0000..=0x0FFF => {}, // Jump to machine code routine at nnn
                0x1000..=0x1FFF => { self.jp(opcode); }, // Jump to location nnn
                0x2000..=0x2FFF => { self.call(opcode); }, // Call subroutine at nnn
                0x3000..=0x4FFF => { self.se_and_sne(opcode)} // Skip next instruction if Vx == kk
                0x5000..=0x5FF0 => { self.se_register_only(opcode) }, // Skip next instruction if Vx == Vy
                0x6000..=0x6FFF => { self.ld(opcode) }, // Set Vx = kk
                0x7000..=0x7FFF => { self.add(opcode) }, // set Vx = Vx + kk
                0x8000..=0x8FFF => {
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let y = ((opcode & 0x00F0) >> 4) as u8;
                    let operation_type = (opcode & 0x000F) as u8;

                    match (operation_type)
                    {
                        0x1 => self.or(x, y),
                        0x2 => self.and(x,y),
                        0x3 => self.xor(x,y),
                        0x4 => self.add_register_only(x,y),
                        0x5 => self.sub(x,y),
                        0x6 => self.shr(x,y),
                        0x7 => self.subn(x, y),
                        0xE => self.shl(x, y),
                        _ => todo!("opcode {:04x}", opcode),
                    }
                }
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_opcode(self: &Self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn jp(&mut self, opcode: u16) {
        let addr = opcode & 0x0FFF;
        self.position_in_memory = addr as usize;
    }

    fn call(&mut self, opcode: u16) {
        let addr = opcode & 0x0FFF;

        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        
        if sp >= stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }


    fn se_and_sne(&mut self, opcode: u16) {
        let should_be_equal = ((opcode & 0xF000) >> 12) == 0x3;

        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        let is_equal = self.registers[x as usize] == kk;
        
        if (should_be_equal == is_equal) {
            self.position_in_memory += 2;
        }
    }

    fn se_register_only(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        if (self.registers[x as usize] == self.registers[y as usize]) {
            self.position_in_memory += 2;
        }
    }

    fn ld(&mut self, opcode: u16) {
        let kk = (opcode & 0x00FF) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.registers[x as usize] = kk;
    }

    fn add(&mut self, opcode: u16) {
        let kk = (opcode & 0x00FF) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;

       self.registers[x as usize] += kk; 
    }
   
    fn or(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  |= self.registers[y as usize];
    }
    
    fn and(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  &= self.registers[y as usize];
    }
    
    fn xor(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  ^= self.registers[y as usize];
    }
    
    fn add_register_only(&mut self, x: u8, y: u8) {
        // TODO: set VF = carry
        self.registers[x as usize]  += self.registers[y as usize];
    }

    fn sub(&mut self, x: u8, y: u8) {
        // TODO: set VF = NOT borrow
        self.registers[x as usize]  -= self.registers[y as usize];
    }
    
    fn shr(&mut self, x: u8, y: u8) {
        let lsb = self.registers[x as usize] & 1;
        self.registers[0xF] = lsb;

        self.registers[x as usize] /= 2;
    }
    
    fn subn(&mut self, x: u8, y: u8) {
        if (self.registers[x as usize] < self.registers[y as usize])
        {
            self.registers[0xF] = 1;
        }
        else 
        {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize]  = self.registers[y as usize] - self.registers[x as usize];
        
        // TODO: set VF = not borrow
    }
    
    fn shl(&mut self, x: u8, y: u8) {
        let msb = self.registers[x as usize] >> 7;
        self.registers[0xF] = msb;
        
        self.registers[x as usize] *= 2;
    }

    fn sne_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        if (self.registers[x as usize] != self.registers[y as usize]) {
            self.position_in_memory += 2;
        }
    }

    fn ld_i_addr(&mut self, opcode: u16)
    {
        let addr = opcode & 0x0FFF;
        self.i_register = addr;
    }

    fn jp_v0_addr(&mut self, opcode: u16)
    {
        let addr = opcode & 0x0FFF;
        self.position_in_memory = (addr + self.registers[0 as usize] as u16) as usize
    }

    fn rnd_vx_byte(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        let number: u8 = rand::rng().random_range(0..=255);
        self.registers[x as usize] = (number & kk);
    }

    fn drw_vx_vy_nibble(&mut self, opcode: u16)
    {

    }

    fn skip_vx(&mut self, opcode: u16)
    {
        // keyboard needed
    }

    fn sknp_vx(&mut self, opcode: u16)
    {
        // keyboard needed
    }

    fn ld_vx_dt(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.registers[x as usize] = self.delay_timer_register.load(Ordering::Relaxed);
    }

    fn lf_vx_k(&mut self, opcode: u16)
    {
        // keyboard needed
    }

    fn ld_dt_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.delay_timer_register.swap(self.registers[x as usize], Ordering::Relaxed);
    }

    fn ld_st_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.sound_timer_register.swap(self.registers[x as usize], Ordering::Relaxed);
    }

    fn add_i_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.i_register += self.registers[x as usize] as u16;
    }

    fn ld_f_vx(&mut self, opcode: u16)
    {
        // TODO: sprite location stuff
    }

    fn ld_b_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let value = self.registers[x as usize];

        let hundrets = value / 100; 
        let tens = (value % 100) / 10;
        let ones = value % 10;
    
        self.memory[self.i_register as usize] = hundrets;
        self.memory[(self.i_register + 1) as usize] = tens;
        self.memory[(self.i_register + 2) as usize] = ones;
    }

    fn ld_i_fx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        let mut i = self.i_register;
        
        for n in 0..x 
        {
            self.memory[i as usize] = self.registers[n as usize];
            i += 1;
        }
    }

    fn ld_vx_i(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        let mut i = self.i_register;
        
        for n in 0..x 
        {
            self.registers[n as usize] = self.memory[i as usize];
            i += 1;
        }
 
    }
}

