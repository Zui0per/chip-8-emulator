use fastrand;

pub const DISPLAY_WIDTH: u8 = 64;
pub const DISPLAY_HEIGHT: u8 = 32;
const FONT_START_ADDRESS: u8 = 0;
const FONT_CHAR_SIZE_IN_BYTES: u8 = 5;
const TIMER_DECREMENT_FEQUENCY: u8 = 60;

const OCTAJAM_TITLE: &[u8] = include_bytes!("./roms/octojam1title.ch8");
const PUZZLE_15: &[u8] = include_bytes!("./roms/15PUZZLE");
const INVADERS: &[u8] = include_bytes!("./roms/INVADERS");
const GUESS: &[u8] = include_bytes!("./roms/GUESS");
const PONG: &[u8] = include_bytes!("./roms/PONG");
const PONG2: &[u8] = include_bytes!("./roms/PONG2");
const TANK: &[u8] = include_bytes!("./roms/TANK");

const RED_OCTOBER: &[u8] = include_bytes!("./roms/redOctober.ch8");
const BLINKY: &[u8] = include_bytes!("./roms/BLINKY");
const BLITZ: &[u8] = include_bytes!("./roms/BLITZ");
const BRIX: &[u8] = include_bytes!("./roms/BRIX");
const CONNECT: &[u8] = include_bytes!("./roms/CONNECT4");
const HIDDEN: &[u8] = include_bytes!("./roms/HIDDEN");
const KALEID: &[u8] = include_bytes!("./roms/KALEID");
const MAZE: &[u8] = include_bytes!("./roms/MAZE");
const MERLIN: &[u8] = include_bytes!("./roms/MERLIN");
const MISSILE: &[u8] = include_bytes!("./roms/MISSILE");
const PUZZLE: &[u8] = include_bytes!("./roms/PUZZLE");
const SYZYGY: &[u8] = include_bytes!("./roms/SYZYGY");
const TETRIS: &[u8] = include_bytes!("./roms/TETRIS");
const TICTAC: &[u8] = include_bytes!("./roms/TICTAC");
const UFO: &[u8] = include_bytes!("./roms/UFO");
const VBRIX: &[u8] = include_bytes!("./roms/VBRIX");
const VERS: &[u8] = include_bytes!("./roms/VERS");
const WIPEOFF: &[u8] = include_bytes!("./roms/WIPEOFF");

pub struct Chip8 { registers: [u8; 16],
    i_register: u16,
    sound_timer: u8,
    delay_timer: u8,
    accumulator_timer: f64,
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
    display: [[u8; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
    keyboard: [bool; 16]
}

pub struct RegistersSnapshot {
    pub V0: u8,
    pub V1: u8,
    pub V2: u8,
    pub V3: u8,
    pub V4: u8,
    pub V5: u8,
    pub V6: u8,
    pub V7: u8,
    pub V8: u8,
    pub V9: u8,
    pub VA: u8,
    pub VB: u8,
    pub VC: u8,
    pub VD: u8,
    pub VE: u8,
    pub VF: u8,
    pub I: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub programm_counter: u16,
    pub stack_pointer: u8,
}

impl Chip8 {

    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            registers: [0; 16],
            i_register: 0,
            sound_timer: 0, 
            accumulator_timer: 0.0,
            delay_timer: 0,
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
            display: [[0; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
            keyboard: [false; 16]
        };

        chip8.fill_reserved_memory();
        chip8.load_rom_in_memory("octajam_title".to_string());
        chip8
    }

    fn fill_reserved_memory(&mut self) {
        let fontset: [u8; 80] = [
            // 0
            0xF0, 0x90, 0x90, 0x90, 0xF0,
            // 1
            0x20, 0x60, 0x20, 0x20, 0x70,
            // 2
            0xF0, 0x10, 0xF0, 0x80, 0xF0,
            // 3
            0xF0, 0x10, 0xF0, 0x10, 0xF0,
            // 4
            0x90, 0x90, 0xF0, 0x10, 0x10,
            // 5
            0xF0, 0x80, 0xF0, 0x10, 0xF0,
            // 6
            0xF0, 0x80, 0xF0, 0x90, 0xF0,
            // 7
            0xF0, 0x10, 0x20, 0x40, 0x40,
            // 8
            0xF0, 0x90, 0xF0, 0x90, 0xF0,
            // 9
            0xF0, 0x90, 0xF0, 0x10, 0xF0,
            // A
            0xF0, 0x90, 0xF0, 0x90, 0x90,
            // B
            0xE0, 0x90, 0xE0, 0x90, 0xE0,
            // C
            0xF0, 0x80, 0x80, 0x80, 0xF0,
            // D
            0xE0, 0x90, 0x90, 0x90, 0xE0,
            // E
            0xF0, 0x80, 0xF0, 0x80, 0xF0,
            // F
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for (i, &byte) in fontset.iter().enumerate() {
          self.memory[i] = byte;
        }
    }

    pub fn get_display(&self) -> *const u8 {
        self.display.as_ptr() as *const u8 
    }

    pub fn get_register_snapshot(&self) -> RegistersSnapshot {
        RegistersSnapshot {
            V0: self.registers[0],
            V1: self.registers[1],
            V2: self.registers[2],
            V3: self.registers[3],
            V4: self.registers[4],
            V5: self.registers[5],
            V6: self.registers[6],
            V7: self.registers[7],
            V8: self.registers[8],
            V9: self.registers[9],
            VA: self.registers[10],
            VB: self.registers[11],
            VC: self.registers[12],
            VD: self.registers[13],
            VE: self.registers[14],
            VF: self.registers[15],
            I: self.i_register,
            delay_timer: self.delay_timer,
            sound_timer: self.sound_timer,
            programm_counter: self.position_in_memory as u16,
            stack_pointer: self.stack_pointer as u8
        }
    }

    pub fn set_key(&mut self, key: u8, is_pressed: bool)
    {
        self.keyboard[key as usize] = is_pressed;
    } 

    pub fn execute_step(&mut self) -> u16 {
        
        let  opcode = self.read_opcode();
        self.position_in_memory += 2;

        match opcode {
            0x00E0 => { self.cls(); } // Clear the display
            0x00EE => { self.ret(); }, // Return from a subroutine
            0x0000..=0x0FFF => {}, // Jump to machine code routine at nnn
            0x1000..=0x1FFF => { self.jp_addr(opcode); }, // Jump to location nnn
            0x2000..=0x2FFF => { self.call_addr(opcode); }, // Call subroutine at nnn
            0x3000..=0x4FFF => { self.se_and_sne_vx_byte(opcode)} // Skip next instruction if Vx == kk
            0x5000..=0x5FF0 => { self.se_vx_vy(opcode) }, // Skip next instruction if Vx == Vy
            0x6000..=0x6FFF => { self.ld_vx_byte(opcode) }, // Set Vx = kk
            0x7000..=0x7FFF => { self.add_vx_byte(opcode) }, // set Vx = Vx + kk
            0x8000..=0x8FFF => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                let operation_type = (opcode & 0x000F) as u8;

                match (operation_type)
                {
                    0x0 => self.ld_vx_vy(x, y),
                    0x1 => self.or_vx_vy(x, y),
                    0x2 => self.and_vx_vy(x,y),
                    0x3 => self.xor_vx_vy(x,y),
                    0x4 => self.add_vx_vy(x,y),
                    0x5 => self.sub_vx_vy(x,y),
                    0x6 => self.shr_vx_vy(x,y),
                    0x7 => self.subn_vx_vy(x, y),
                    0xE => self.shl_vx_vy(x, y),
                    _ => todo!("opcode {:04x}", opcode),
                }
            }
            0x9000..=0x9FFF => { self.sne_vx_vy(opcode); }
            0xA000..=0xAFFF => { self.ld_i_addr(opcode); }
            0xB000..=0xBFFF => { self.jp_v0_addr(opcode); }
            0xC000..=0xCFFF => { self.rnd_vx_byte(opcode); }
            0xD000..=0xDFFF => { self.drw_vx_vy_nibble(opcode); }
            0xE000..=0xEFFF => { 
                let operation_type = (opcode & 0x00FF)  as u16;
                
                match (operation_type)
                {
                    0x9E => self.skp_vx(opcode),
                    0xA1 => self.sknp_vx(opcode),
                    _ => todo!("opcode {:04x}", opcode)
                }
            }
            0xF000..=0xFFFF => { 
                let operation_type = (opcode & 0x00FF) as u16;
                
                match (operation_type)
                {
                    0x07 => self.ld_vx_dt(opcode),
                    0x0A => self.ld_vx_k(opcode),
                    0x15 => self.ld_dt_vx(opcode),
                    0x18 => self.ld_st_vx(opcode),
                    0x1E => self.add_i_vx(opcode),
                    0x29 => self.ld_f_vx(opcode),
                    0x33 => self.ld_b_vx(opcode),
                    0x55 => self.ld_i_fx(opcode),
                    0x65 => self.ld_vx_i(opcode),
                    _ => todo!("opcode {:04x}", opcode)
                }
            }
             _ => todo!("opcode {:04x}", opcode),
        }
        opcode
    }

    pub fn update_timers(self: &mut Self, elapsed_ms: u16)
    {
        self.accumulator_timer += elapsed_ms as f64;
        
        let duration_ms_between_decrements = 1000.0 / TIMER_DECREMENT_FEQUENCY as f64;
        
        if (self.accumulator_timer > duration_ms_between_decrements)
        {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);
        }
    }

    pub fn is_sound_active(self: &Self) -> bool
    {
        self.sound_timer > 0
    }

    pub fn load_rom_in_memory(self: &mut Self, name: String)
    {
        let rom = Chip8::get_rom(name.as_str())
            .ok_or_else( || format!("ROM not found: '{}'", name)).unwrap();

        let start = 0x200;
        let end = start + rom.len();
        self.memory[start..end].copy_from_slice(rom);
        
        self.position_in_memory = 0x200;
    }

    fn get_rom(name: &str) -> Option<&[u8]> {
        match name {
            "octajam_title" => Some(OCTAJAM_TITLE),
            "red_october" => Some(RED_OCTOBER),
            "puzzle_15" => Some(PUZZLE_15),
            "blinky" => Some(BLINKY),
            "blitz" => Some(BLITZ),
            "brix" => Some(BRIX),
            "connect" => Some(CONNECT),
            "guess" => Some(GUESS),
            "hidden" => Some(HIDDEN),
            "invaders" => Some(INVADERS),
            "kaleid" => Some(KALEID),
            "maze" => Some(MAZE),
            "merlin" => Some(MERLIN),
            "missile" => Some(MISSILE),
            "pong" => Some(PONG),
            "pong2" => Some(PONG2),
            "puzzle" => Some(PUZZLE),
            "syzygy" => Some(SYZYGY),
            "tank" => Some(TANK),
            "tetris" => Some(TETRIS),
            "tictac" => Some(TICTAC),
            "ufo" => Some(UFO),
            "vbrix" => Some(VBRIX),
            "vers" => Some(VERS),
            "wipeoff" => Some(WIPEOFF),
            _ => None,
        }
    }

    fn read_opcode(self: &Self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | op_byte2
    }

    fn cls(self: &mut Self) {
        for row in self.display.iter_mut() {
            for pixel in row.iter_mut()
            {
                *pixel = 0;
            }
        }
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn jp_addr(&mut self, opcode: u16) {
        let addr = opcode & 0x0FFF;
        self.position_in_memory = addr as usize;
    }

    fn call_addr(&mut self, opcode: u16) {
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


    fn se_and_sne_vx_byte(&mut self, opcode: u16) {
        let should_be_equal = ((opcode & 0xF000) >> 12) == 0x3;

        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        let is_equal = self.registers[x as usize] == kk;
        
        if (should_be_equal == is_equal) {
            self.position_in_memory += 2;
        }
    }

    fn se_vx_vy(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        if (self.registers[x as usize] == self.registers[y as usize]) {
            self.position_in_memory += 2;
        }
    }

    fn ld_vx_byte(&mut self, opcode: u16) {
        let kk = (opcode & 0x00FF) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.registers[x as usize] = kk;
    }

    fn add_vx_byte(&mut self, opcode: u16) {
        let kk = (opcode & 0x00FF) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;

       self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk); 
    }

    fn ld_vx_vy(&mut self, x: u8, y: u8)
    {
        self.registers[x as usize] = self.registers[y as usize];
    }
   
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  |= self.registers[y as usize];
    }
    
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  &= self.registers[y as usize];
    }
    
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.registers[x as usize]  ^= self.registers[y as usize];
    }
    
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let (sum, carry) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[0xF] = if carry { 1 } else { 0 };
        self.registers[x as usize] = sum;
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        let (res, borrow) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
        self.registers[0xF] = if borrow { 0 } else { 1 };
        self.registers[x as usize] = res;
    }
    
    fn shr_vx_vy(&mut self, x: u8, y: u8) {
        let lsb = self.registers[x as usize] & 1;
        self.registers[0xF] = lsb;

        self.registers[x as usize] /= 2;
    }
    
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        if (self.registers[x as usize] < self.registers[y as usize])
        {
            self.registers[0xF] = 1;
        }
        else 
        {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize]  = self.registers[y as usize].wrapping_sub(self.registers[x as usize]);
    }
    
    fn shl_vx_vy(&mut self, x: u8, y: u8) {
        let msb = self.registers[x as usize] >> 7;
        self.registers[0xF] = msb;
        
        self.registers[x as usize] = self.registers[x as usize].wrapping_mul(2);
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

        let number: u8 = fastrand::u8(0..=255);
        self.registers[x as usize] = (number & kk);
    }

    fn drw_vx_vy_nibble(&mut self, opcode: u16)
    {
        let x= ((opcode & 0x0F00) >> 8) as u8;
        let y= ((opcode & 0x00F0) >> 4) as u8;
        let n= (opcode & 0x000F) as u8;
        
        let x_pos = self.registers[x as usize];
        let y_pos = self.registers[y as usize];
        let mut is_switched_off = false;
        
        for byte_row_index in 0..n {
            let byte = self.memory[(self.i_register) as usize + byte_row_index as usize];

            for bit_index in 0..8 {
                let pixel_x = (x_pos + bit_index) % DISPLAY_WIDTH; 
                let pixel_y = (y_pos + byte_row_index) % DISPLAY_HEIGHT; 
                
                let bit_value = (byte >> (7 - bit_index)) & 1;
                let old_pixel = self.display[pixel_y as usize][pixel_x as usize];
                self.display[pixel_y as usize][pixel_x as usize] ^= bit_value; 
            
                if old_pixel == 1 && self.display[pixel_y as usize][pixel_x as usize] == 0
                {
                   is_switched_off = true; 
                }
            } 
        }

        self.registers[0xF] = if is_switched_off { 1 } else { 0 };
    }

    fn skp_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        if self.keyboard[self.registers[x as usize] as usize]
        {
            self.position_in_memory +=2;
        }
    }

    fn sknp_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        if !self.keyboard[self.registers[x as usize] as usize]
        {
            self.position_in_memory +=2;
        }
    }

    fn ld_vx_dt(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.registers[x as usize] = self.delay_timer
    }

    fn ld_vx_k(&mut self, opcode: u16) 
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        for (index, pressed) in self.keyboard.iter().enumerate()
        {
            if *pressed
            {
                self.registers[x as usize] = index as u8;
                return;
            }
        }

        self.position_in_memory -= 2;
    }

    fn ld_dt_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.delay_timer = self.registers[x as usize];
    }

    fn ld_st_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.sound_timer = self.registers[x as usize];
    }

    fn add_i_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        self.i_register += self.registers[x as usize] as u16;
    }

    fn ld_f_vx(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        let digit = self.registers[x as usize] as u16;
        self.i_register = FONT_START_ADDRESS as u16 + FONT_CHAR_SIZE_IN_BYTES as u16 * digit;
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
        
        for n in 0..=x 
        {
            self.memory[i as usize] = self.registers[n as usize];
            i += 1;
        }

        self.i_register += (x + 1) as u16
    }

    fn ld_vx_i(&mut self, opcode: u16)
    {
        let x = ((opcode & 0x0F00) >> 8) as u8;  
        let mut i = self.i_register;
        
        for n in 0..=x 
        {
            self.registers[n as usize] = self.memory[i as usize];
            i += 1;
        }
 
        self.i_register += (x + 1) as u16
    }
}

// AI generated tests (specification used as input)
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a Chip8 instance and load a single opcode at the start address.
    fn setup_with_opcode(opcode: u16) -> Chip8 {
        let mut chip8 = Chip8::new();
        // Overwrite the default loaded ROM for a clean test environment.
        chip8.position_in_memory = 0x200;
        chip8.memory[0x200] = (opcode >> 8) as u8;
        chip8.memory[0x201] = (opcode & 0x00FF) as u8;
        chip8
    }

    #[test]
    fn test_00e0_cls() {
        let mut chip8 = setup_with_opcode(0x00E0);
        // Arrange: Dirty the display
        chip8.display[10][20] = 1;
        chip8.display[31][63] = 1;

        // Act
        chip8.execute_step();

        // Assert: The entire display should be cleared
        for y in 0..DISPLAY_HEIGHT as usize {
            for x in 0..DISPLAY_WIDTH as usize {
                assert_eq!(chip8.display[y][x], 0, "Pixel at ({}, {}) was not cleared", x, y);
            }
        }
    }

    #[test]
    fn test_00ee_ret() {
        let mut chip8 = setup_with_opcode(0x00EE);
        // Arrange: Simulate a subroutine call
        chip8.stack_pointer = 1;
        chip8.stack[0] = 0x350; // Return address

        // Act
        chip8.execute_step();

        // Assert: PC should be the return address and stack pointer decremented
        assert_eq!(chip8.position_in_memory, 0x350);
        assert_eq!(chip8.stack_pointer, 0);
    }

    #[test]
    fn test_1nnn_jp_addr() {
        let mut chip8 = setup_with_opcode(0x1ABC);
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is set to nnn, not pc + 2
        assert_eq!(chip8.position_in_memory, 0xABC);
    }

    #[test]
    fn test_2nnn_call_addr() {
        let mut chip8 = setup_with_opcode(0x2ABC);
        let initial_pc = chip8.position_in_memory;
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.position_in_memory, 0xABC, "PC should be at the new address");
        assert_eq!(chip8.stack_pointer, 1, "Stack pointer should be incremented");
        assert_eq!(chip8.stack[0] as usize, initial_pc + 2, "Return address on stack should be the next instruction");
    }

    #[test]
    fn test_3xkk_se_vx_byte_skip() {
        let mut chip8 = setup_with_opcode(0x35AB);
        // Arrange: Set V5 to the value to match
        chip8.registers[5] = 0xAB;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is incremented by 4 (2 for the instruction, 2 for the skip)
        assert_eq!(chip8.position_in_memory, 0x204);
    }
    
    #[test]
    fn test_3xkk_se_vx_byte_no_skip() {
        let mut chip8 = setup_with_opcode(0x35AB);
        // Arrange: Set V5 to a different value
        chip8.registers[5] = 0xAC;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is incremented by 2
        assert_eq!(chip8.position_in_memory, 0x202);
    }
    
    #[test]
    fn test_4xkk_sne_vx_byte_skip() {
        let mut chip8 = setup_with_opcode(0x45AB);
        // Arrange: Set V5 to a different value
        chip8.registers[5] = 0xAC;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is incremented by 4
        assert_eq!(chip8.position_in_memory, 0x204);
    }

    #[test]
    fn test_4xkk_sne_vx_byte_no_skip() {
        let mut chip8 = setup_with_opcode(0x45AB);
        // Arrange: Set V5 to the same value
        chip8.registers[5] = 0xAB;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is incremented by 2
        assert_eq!(chip8.position_in_memory, 0x202);
    }

    #[test]
    fn test_5xy0_se_vx_vy_skip() {
        let mut chip8 = setup_with_opcode(0x5120);
        // Arrange: Set V1 and V2 to be equal
        chip8.registers[1] = 0xDD;
        chip8.registers[2] = 0xDD;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is skipped
        assert_eq!(chip8.position_in_memory, 0x204);
    }
    
    #[test]
    fn test_5xy0_se_vx_vy_no_skip() {
        let mut chip8 = setup_with_opcode(0x5120);
        // Arrange: Set V1 and V2 to be different
        chip8.registers[1] = 0xDD;
        chip8.registers[2] = 0xDE;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC is not skipped
        assert_eq!(chip8.position_in_memory, 0x202);
    }
    
    #[test]
    fn test_6xkk_ld_vx_byte() {
        let mut chip8 = setup_with_opcode(0x6ABC);
        
        // Act
        chip8.execute_step();
        
        // Assert: VA now holds the value 0xBC
        assert_eq!(chip8.registers[0xA], 0xBC);
    }
    
    #[test]
    fn test_7xkk_add_vx_byte() {
        let mut chip8 = setup_with_opcode(0x7310);
        // Arrange: Put a value in V3 to add to
        chip8.registers[3] = 0x05;
        
        // Act
        chip8.execute_step();
        
        // Assert: V3 is now 0x05 + 0x10 = 0x15
        assert_eq!(chip8.registers[3], 0x15);
    }

    #[test]
    fn test_7xkk_add_vx_byte_with_wrap() {
        let mut chip8 = setup_with_opcode(0x7310);
        // Arrange: Put a value in V3 that will cause an overflow
        chip8.registers[3] = 0xFF;
        
        // Act
        chip8.execute_step();
        
        // Assert: V3 wraps around (0xFF + 0x10 = 0x10F -> 0x0F)
        // VF is NOT changed by this operation.
        assert_eq!(chip8.registers[3], 0x0F);
        assert_eq!(chip8.registers[0xF], 0); // VF untouched
    }

    #[test]
    fn test_8xy0_ld_vx_vy() {
        let mut chip8 = setup_with_opcode(0x8120);
        // Arrange
        chip8.registers[2] = 0xCC;

        // Act
        chip8.execute_step();
        
        // Assert: V1 now holds the value from V2
        assert_eq!(chip8.registers[1], 0xCC);
    }

    #[test]
    fn test_8xy1_or_vx_vy() {
        let mut chip8 = setup_with_opcode(0x8121);
        // Arrange
        chip8.registers[1] = 0b1010_1100;
        chip8.registers[2] = 0b0101_1010;

        // Act
        chip8.execute_step();

        // Assert: V1 is the bitwise OR
        assert_eq!(chip8.registers[1], 0b1111_1110);
    }
    
    #[test]
    fn test_8xy2_and_vx_vy() {
        let mut chip8 = setup_with_opcode(0x8122);
        // Arrange
        chip8.registers[1] = 0b1010_1100;
        chip8.registers[2] = 0b0101_1010;

        // Act
        chip8.execute_step();

        // Assert: V1 is the bitwise AND
        assert_eq!(chip8.registers[1], 0b0000_1000);
    }
    
    #[test]
    fn test_8xy3_xor_vx_vy() {
        let mut chip8 = setup_with_opcode(0x8123);
        // Arrange
        chip8.registers[1] = 0b1010_1100;
        chip8.registers[2] = 0b0101_1010;

        // Act
        chip8.execute_step();

        // Assert: V1 is the bitwise XOR
        assert_eq!(chip8.registers[1], 0b1111_0110);
    }

    #[test]
    fn test_8xy4_add_vx_vy_no_carry() {
        let mut chip8 = setup_with_opcode(0x8124);
        // Arrange
        chip8.registers[1] = 100;
        chip8.registers[2] = 50;
        
        // Act
        chip8.execute_step();

        // Assert
        assert_eq!(chip8.registers[1], 150);
        assert_eq!(chip8.registers[0xF], 0, "VF should be 0 for no carry");
    }
    
    #[test]
    fn test_8xy4_add_vx_vy_with_carry() {
        let mut chip8 = setup_with_opcode(0x8124);
        // Arrange
        chip8.registers[1] = 200;
        chip8.registers[2] = 100;
        
        // Act
        chip8.execute_step();

        // Assert: 200 + 100 = 300 -> 44 (with carry)
        assert_eq!(chip8.registers[1], 44);
        assert_eq!(chip8.registers[0xF], 1, "VF should be 1 for carry");
    }

    #[test]
    fn test_8xy5_sub_vx_vy_no_borrow() {
        let mut chip8 = setup_with_opcode(0x8125);
        // Arrange: Vx > Vy
        chip8.registers[1] = 100;
        chip8.registers[2] = 50;

        // Act
        chip8.execute_step();

        // Assert: VF is 1 (NOT borrow)
        assert_eq!(chip8.registers[1], 50);
        assert_eq!(chip8.registers[0xF], 1);
    }
    
    #[test]
    fn test_8xy5_sub_vx_vy_with_borrow() {
        let mut chip8 = setup_with_opcode(0x8125);
        // Arrange: Vx < Vy
        chip8.registers[1] = 50;
        chip8.registers[2] = 100;

        // Act
        chip8.execute_step();

        // Assert: VF is 0 (borrow)
        assert_eq!(chip8.registers[1], 206); // 50 - 100 wraps
        assert_eq!(chip8.registers[0xF], 0);
    }

    #[test]
    fn test_8xy6_shr_vx_lsb_one() {
        let mut chip8 = setup_with_opcode(0x8106); // y is ignored
        // Arrange: Vx has LSB of 1
        chip8.registers[1] = 0b10101011;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.registers[1], 0b01010101);
        assert_eq!(chip8.registers[0xF], 1);
    }
    
    #[test]
    fn test_8xy6_shr_vx_lsb_zero() {
        let mut chip8 = setup_with_opcode(0x8106); // y is ignored
        // Arrange: Vx has LSB of 0
        chip8.registers[1] = 0b10101010;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.registers[1], 0b01010101);
        assert_eq!(chip8.registers[0xF], 0);
    }

    #[test]
    fn test_8xy7_subn_vx_vy_no_borrow() {
        let mut chip8 = setup_with_opcode(0x8127);
        // Arrange: Vy > Vx
        chip8.registers[1] = 50;
        chip8.registers[2] = 100;

        // Act
        chip8.execute_step();

        // Assert: Vx = Vy - Vx, VF is 1 (NOT borrow)
        assert_eq!(chip8.registers[1], 50);
        assert_eq!(chip8.registers[0xF], 1);
    }
    
    #[test]
    fn test_8xy7_subn_vx_vy_with_borrow() {
        let mut chip8 = setup_with_opcode(0x8127);
        // Arrange: Vy < Vx
        chip8.registers[1] = 100;
        chip8.registers[2] = 50;

        // Act
        chip8.execute_step();

        // Assert: Vx = Vy - Vx, VF is 0 (borrow)
        assert_eq!(chip8.registers[1], 206); // 50 - 100 wraps
        assert_eq!(chip8.registers[0xF], 0);
    }

    #[test]
    fn test_8xye_shl_vx_msb_one() {
        let mut chip8 = setup_with_opcode(0x810E); // y is ignored
        // Arrange: Vx has MSB of 1
        chip8.registers[1] = 0b10101010;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.registers[1], 0b01010100);
        assert_eq!(chip8.registers[0xF], 1);
    }
    
    #[test]
    fn test_8xye_shl_vx_msb_zero() {
        let mut chip8 = setup_with_opcode(0x810E); // y is ignored
        // Arrange: Vx has MSB of 0
        chip8.registers[1] = 0b01010101;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.registers[1], 0b10101010);
        assert_eq!(chip8.registers[0xF], 0);
    }

    #[test]
    fn test_9xy0_sne_vx_vy_skip() {
        let mut chip8 = setup_with_opcode(0x9120);
        // Arrange
        chip8.registers[1] = 0xAA;
        chip8.registers[2] = 0xBB;
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.position_in_memory, 0x204);
    }

    #[test]
    fn test_annn_ld_i_addr() {
        let mut chip8 = setup_with_opcode(0xA123);
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.i_register, 0x123);
    }

    #[test]
    fn test_bnnn_jp_v0_addr() {
        let mut chip8 = setup_with_opcode(0xB300);
        // Arrange
        chip8.registers[0] = 0x50;
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.position_in_memory, 0x350);
    }

    #[test]
    fn test_cxkk_rnd_vx_byte() {
        let mut chip8 = setup_with_opcode(0xC10F);
        // Arrange: mask is 0x0F
        
        // Act
        chip8.execute_step();
        
        // Assert: We can't know the random number, but we can know it was ANDed with the mask.
        // The result in V1 must be between 0 and 15.
        let result = chip8.registers[1];
        assert!(result <= 0x0F, "RND result {} was not masked with 0x0F", result);
    }

    #[test]
    fn test_dxyn_drw_no_collision() {
        let mut chip8 = setup_with_opcode(0xD011); // Draw at (V0, V1) a 1-byte sprite
        // Arrange
        chip8.registers[0] = 10; // x-coord
        chip8.registers[1] = 20; // y-coord
        chip8.i_register = 0x300;
        chip8.memory[0x300] = 0b10101010;

        // Act
        chip8.execute_step();

        // Assert: Check pixels were drawn correctly
        assert_eq!(chip8.display[20][10], 1);
        assert_eq!(chip8.display[20][11], 0);
        assert_eq!(chip8.display[20][12], 1);
        assert_eq!(chip8.display[20][13], 0);
        // ... and so on
        assert_eq!(chip8.display[20][17], 0);

        // Assert: VF is 0 for no collision
        assert_eq!(chip8.registers[0xF], 0);
    }

    #[test]
    fn test_dxyn_drw_with_collision() {
        let mut chip8 = setup_with_opcode(0xD011);
        // Arrange
        chip8.registers[0] = 10;
        chip8.registers[1] = 20;
        chip8.i_register = 0x300;
        chip8.memory[0x300] = 0b11000000;
        // Pre-set a pixel that will be turned off
        chip8.display[20][10] = 1;

        // Act
        chip8.execute_step();

        // Assert: Pixel is turned off (1 XOR 1 = 0)
        assert_eq!(chip8.display[20][10], 0);
        // Another pixel is turned on (0 XOR 1 = 1)
        assert_eq!(chip8.display[20][11], 1);

        // Assert: VF is 1 for collision
        assert_eq!(chip8.registers[0xF], 1);
    }
    
    #[test]
    fn test_dxyn_drw_wrapping() {
        let mut chip8 = setup_with_opcode(0xD011);
        // Arrange
        chip8.registers[0] = 62; // x-coord near the edge
        chip8.registers[1] = 10; // y-coord
        chip8.i_register = 0x300;
        // This sprite is 8 pixels wide. '1's will be at x=62, 63, 0, 1
        chip8.memory[0x300] = 0b11110000;

        // Act
        chip8.execute_step();

        // Assert: Check pixels on both sides of the screen
        assert_eq!(chip8.display[10][62], 1);
        assert_eq!(chip8.display[10][63], 1);
        assert_eq!(chip8.display[10][0], 1);
        assert_eq!(chip8.display[10][1], 1);
        assert_eq!(chip8.display[10][2], 0);
    }

    #[test]
    fn test_ex9e_skp_vx_skip() {
        let mut chip8 = setup_with_opcode(0xE59E);
        // Arrange
        chip8.registers[5] = 0xA; // Key 'A' (10)
        chip8.keyboard[0xA] = true; // Key 'A' is pressed

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.position_in_memory, 0x204);
    }

    #[test]
    fn test_exa1_sknp_vx_skip() {
        let mut chip8 = setup_with_opcode(0xE5A1);
        // Arrange
        chip8.registers[5] = 0xA; // Key 'A'
        chip8.keyboard[0xA] = false; // Key 'A' is NOT pressed

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.position_in_memory, 0x204);
    }

    #[test]
    fn test_fx07_ld_vx_dt() {
        let mut chip8 = setup_with_opcode(0xF307);
        // Arrange
        chip8.delay_timer = 55;
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.registers[3], 55);
    }

    #[test]
    fn test_fx0a_ld_vx_k_wait() {
        let mut chip8 = setup_with_opcode(0xF30A);
        // Arrange: No keys are pressed
        chip8.keyboard = [false; 16];
        
        // Act
        chip8.execute_step();
        
        // Assert: PC does not advance (it is decremented after being incremented)
        assert_eq!(chip8.position_in_memory, 0x200);
    }
    
    #[test]
    fn test_fx0a_ld_vx_k_key_pressed() {
        let mut chip8 = setup_with_opcode(0xF30A);
        // Arrange
        chip8.keyboard[0xC] = true;
        
        // Act
        chip8.execute_step();
        
        // Assert: PC advances and V3 gets the key value
        assert_eq!(chip8.position_in_memory, 0x202);
        assert_eq!(chip8.registers[3], 0xC);
    }
    
    #[test]
    fn test_fx15_ld_dt_vx() {
        let mut chip8 = setup_with_opcode(0xF815);
        // Arrange
        chip8.registers[8] = 99;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.delay_timer, 99);
    }
    
    #[test]
    fn test_fx18_ld_st_vx() {
        let mut chip8 = setup_with_opcode(0xF818);
        // Arrange
        chip8.registers[8] = 123;

        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.sound_timer, 123);
    }
    
    #[test]
    fn test_fx1e_add_i_vx() {
        let mut chip8 = setup_with_opcode(0xF51E);
        // Arrange
        chip8.i_register = 0x100;
        chip8.registers[5] = 0x50;
        
        // Act
        chip8.execute_step();
        
        // Assert
        assert_eq!(chip8.i_register, 0x150);
    }
    
    #[test]
    fn test_fx29_ld_f_vx() {
        let mut chip8 = setup_with_opcode(0xF229);
        // Arrange: Character '2'
        chip8.registers[2] = 0x2;

        // Act
        chip8.execute_step();
        
        // Assert: I should point to the location of sprite '2'
        // Each font character is 5 bytes long.
        assert_eq!(chip8.i_register, (0x2 * FONT_CHAR_SIZE_IN_BYTES) as u16);
    }
    
    #[test]
    fn test_fx33_ld_b_vx() {
        let mut chip8 = setup_with_opcode(0xF733);
        // Arrange
        chip8.registers[7] = 243; // Decimal value
        chip8.i_register = 0x300;

        // Act
        chip8.execute_step();
        
        // Assert: BCD representation is stored in memory
        assert_eq!(chip8.memory[0x300], 2); // Hundreds
        assert_eq!(chip8.memory[0x301], 4); // Tens
        assert_eq!(chip8.memory[0x302], 3); // Ones
    }

    #[test]
    fn test_fx55_ld_i_vx() {
        let mut chip8 = setup_with_opcode(0xF355);
        // Arrange
        chip8.i_register = 0x400;
        chip8.registers[0] = 0x11;
        chip8.registers[1] = 0x22;
        chip8.registers[2] = 0x33;
        chip8.registers[3] = 0x44;
        
        // Act
        chip8.execute_step();
        
        // Assert: Memory contains register values
        assert_eq!(chip8.memory[0x400], 0x11);
        assert_eq!(chip8.memory[0x401], 0x22);
        assert_eq!(chip8.memory[0x402], 0x33);
        assert_eq!(chip8.memory[0x403], 0x44);

        // Assert: Check I register modification (based on your implementation)
        assert_eq!(chip8.i_register, 0x400 + (3 + 1));
    }

    #[test]
    fn test_fx65_ld_vx_i() {
        let mut chip8 = setup_with_opcode(0xF365);
        // Arrange
        chip8.i_register = 0x400;
        chip8.memory[0x400] = 0xAA;
        chip8.memory[0x401] = 0xBB;
        chip8.memory[0x402] = 0xCC;
        chip8.memory[0x403] = 0xDD;
        
        // Act
        chip8.execute_step();
        
        // Assert: Registers contain memory values
        assert_eq!(chip8.registers[0], 0xAA);
        assert_eq!(chip8.registers[1], 0xBB);
        assert_eq!(chip8.registers[2], 0xCC);
        assert_eq!(chip8.registers[3], 0xDD);
        
        // Assert: Check I register modification (based on your implementation)
        assert_eq!(chip8.i_register, 0x400 + (3 + 1));
    }
}