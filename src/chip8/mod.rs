use fastrand;

pub const DISPLAY_WIDTH: u8 = 64;
pub const DISPLAY_HEIGHT: u8 = 32;
const FONT_START_ADDRESS: u8 = 0;
const FONT_CHAR_SIZE_IN_BYTES: u8 = 5;
const TIMER_DECREMENT_FEQUENCY: u8 = 60;

const BULLET_HELL: &[u8] = include_bytes!("./roms/danm8ku.ch8");
const OCTAJAM_TITLE: &[u8] = include_bytes!("./roms/octojam1title.ch8");

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
        chip8.load_rom();
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

    pub fn load_rom(self: &mut Self)
    {
        let start = 0x200;
        let end = start + OCTAJAM_TITLE.len();
        self.memory[start..end].copy_from_slice(OCTAJAM_TITLE);
        
        self.position_in_memory = 0x200;
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

       self.registers[x as usize] += kk; 
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

        self.registers[x as usize]  = self.registers[y as usize] - self.registers[x as usize];
    }
    
    fn shl_vx_vy(&mut self, x: u8, y: u8) {
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
 
    }
}

