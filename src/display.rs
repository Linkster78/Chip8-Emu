const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pixel_states: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH]
}

impl Display {
    pub fn new() -> Display {
        Display { pixel_states: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH] }
    }

    pub fn clear(&mut self) -> () {
        for pixel in self.pixel_states.iter_mut().flatten() {
            *pixel = false;
        }
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite_data: &[u8]) -> bool {
        let mut collision = false;
        for py in y..y+sprite_data.len() as u8 {
            let mut sprite_row = sprite_data[(py-y) as usize];
            for px in x..x+8 {
                let px_wrapped = px as usize % DISPLAY_WIDTH;
                let py_wrapped = py as usize % DISPLAY_HEIGHT;
                let display_pixel = &mut self.pixel_states[px_wrapped][py_wrapped];
                let sprite_pixel = sprite_row & 0b1 == 1;

                if *display_pixel && sprite_pixel {
                    collision = true;
                }
                *display_pixel ^= sprite_pixel;

                sprite_row >>= 1;
            }
        }
        collision
    }

    pub fn borrow_display(&self) -> &[[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] {
        &self.pixel_states
    }
}