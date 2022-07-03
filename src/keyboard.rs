pub enum KeyEvent {
    Pressed(u8),
    Released(u8)
}

#[derive(Default)]
pub struct Keyboard {
    key_states: [bool; 16],
    pub last_pressed: Option<u8>
}

impl Keyboard {
    pub fn is_pressed(&self, key: usize) -> bool {
        self.key_states[key]
    }

    pub fn update_key_states(&mut self, events: Vec<KeyEvent>) {
        self.last_pressed = None;

        for event in events {
            match event {
                KeyEvent::Pressed(k) => {
                    self.key_states[k as usize] = true;
                    self.last_pressed = Some(k);
                }
                KeyEvent::Released(k) => {
                    self.key_states[k as usize] = false;
                }
            }
        }
    }
}