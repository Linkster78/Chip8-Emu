use crate::{Keyboard, RAM};
use crate::instructions::Instruction;

pub struct CPU {
    v_reg: [u8; 16],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: Vec<u16>
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            v_reg: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: Vec::with_capacity(16)
        }
    }

    pub fn execute(&mut self, ram: &RAM, keyboard: &Keyboard, instruction: Instruction) -> () {

    }
}