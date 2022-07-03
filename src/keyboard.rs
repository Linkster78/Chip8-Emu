pub struct Keyboard {
    key_states: [bool; 16]
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard { key_states: [false; 16] }
    }
}

impl Keyboard {
    pub fn is_pressed(&self, key: usize) -> bool {
        self.key_states[key]
    }
}