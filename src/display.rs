pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pixel_states: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    pub dirty: bool
}

impl Default for Display {
    fn default() -> Self {
        Display {
            pixel_states: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
            dirty: false
        }
    }
}

impl Display {
    pub fn clear(&mut self) {
        for pixel in self.pixel_states.iter_mut().flatten() {
            *pixel = false;
        }
        self.dirty = true;
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite_data: &[u8]) -> bool {
        let x = x as i16;
        let y = y as i16;

        let mut collision = false;
        for py in y..y+sprite_data.len() as i16 {
            let mut sprite_row = sprite_data[(py-y) as usize];
            for px in x..x+8 {
                let px_wrapped = px as usize % DISPLAY_WIDTH;
                let py_wrapped = py as usize % DISPLAY_HEIGHT;
                let display_pixel = &mut self.pixel_states[px_wrapped][py_wrapped];
                let sprite_pixel = sprite_row & 128 == 128;

                if *display_pixel && sprite_pixel {
                    collision = true;
                }
                *display_pixel ^= sprite_pixel;

                sprite_row <<= 1;
            }
        }
        self.dirty = true;
        collision
    }

    pub fn borrow_display(&self) -> &[[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] {
        &self.pixel_states
    }
}