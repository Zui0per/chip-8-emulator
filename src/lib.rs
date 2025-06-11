mod chip8;
use chip8::Chip8;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator { 
    chip8: Chip8,
}

#[wasm_bindgen]
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

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        console_error_panic_hook::set_once();
        Emulator {
            chip8: Chip8::new(),
        }
    }

    pub fn set_key(&mut self, key: u8, is_pressed: bool) {
        self.chip8.set_key(key, is_pressed);
    }

    pub fn get_display_ptr(&mut self) -> *const u8 {
        self.chip8.get_display()
    }

    pub fn get_display_width(&self) -> usize {
        chip8::DISPLAY_WIDTH as usize
    }

    pub fn get_display_height(&self) -> usize {
        chip8::DISPLAY_HEIGHT as usize
    }

    pub fn execute_instruction(&mut self) -> u16 {
        self.chip8.execute_step()
    }

    pub fn update_timers(&mut self, elapsed_ms: u16) {
        self.chip8.update_timers(elapsed_ms);
    }

    pub fn is_sound_active(&self) -> bool {
        self.chip8.is_sound_active()
    }

    pub fn get_register_snapshot(&self) -> RegistersSnapshot {
        let registers = self.chip8.get_register_snapshot();

        RegistersSnapshot {
            V0: registers.V0,
            V1: registers.V1,
            V2: registers.V2,
            V3: registers.V3,
            V4: registers.V4,
            V5: registers.V5,
            V6: registers.V6,
            V7: registers.V7,
            V8: registers.V8,
            V9: registers.V9,
            VA: registers.VA,
            VB: registers.VB,
            VC: registers.VC,
            VD: registers.VD,
            VE: registers.VE,
            VF: registers.VF,
            I: registers.I,
            delay_timer: registers.delay_timer,
            sound_timer: registers.sound_timer,
            programm_counter: registers.programm_counter,
            stack_pointer: registers.stack_pointer
        }
    }
}