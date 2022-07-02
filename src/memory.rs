const RAM_INTPT_OFFSET: usize = 0x200;
const RAM_SIZE: usize = 0x1000 - RAM_INTPT_OFFSET;

pub struct RAM {
    mem: [u8; RAM_SIZE]
}

impl RAM {
    pub fn new() -> RAM {
        RAM { mem: [0; RAM_SIZE] }
    }
}