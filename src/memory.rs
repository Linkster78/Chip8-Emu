pub const RAM_INTPT_OFFSET: u16 = 0x200;
pub const RAM_SIZE: u16 = 0x1000 - RAM_INTPT_OFFSET;

pub struct RAM {
    mem: [u8; RAM_SIZE as usize]
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; RAM_SIZE as usize] }
    }
}