mod chip8;
use chip8::Chip8;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator { 
    chip8: Chip8
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Emulator {
            chip8: Chip8::new()
        }
    }

    pub fn set_key(&mut self, key: u8, is_pressed: bool) {
        self.chip8.set_key(key, is_pressed);
    }

    pub fn get_framebuffer_ptr(&self) -> *const [u8; 64] {
        self.chip8.get_display().as_ptr()
    }

    pub fn get_framebuffer_len(&self) -> usize {
        self.chip8.get_display().len()
    }

    pub fn execute_instruction(&mut self) {
        self.chip8.execute_step();
    }
}