struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard {
    fn new() -> Self {
        Self { keys: [ false; 16] }
    } 

    fn set_key(&mut self, key: u8, value: bool)
    {
        self.keys[key as usize] = value;
    }

    fn is_key_pressed(&mut self, key: u8) -> bool
    {
        self.keys[key as usize]
    }
}