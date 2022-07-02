pub struct Keyboard {
    key_states: [bool; 16]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key_states: [false; 16] }
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.key_states[key]
    }
}