use std::cmp::min;
use crate::Instruction;
use crate::instructions::InstructionReadError;

pub const RAM_INTPT_OFFSET: usize = 0x200;
pub const RAM_SIZE: usize = 0x1000;
pub const PROGRAM_MEMORY_SIZE: usize = RAM_SIZE - RAM_INTPT_OFFSET;

pub const INTPT_SPRITE_LENGTH: usize = 5;
const INTPT_SPRITES: [u8; 16*5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct RAM {
    mem: [u8; RAM_SIZE]
}

impl Default for RAM {
    fn default() -> Self {
        let mut mem = [0; RAM_SIZE];
        mem[..INTPT_SPRITES.len()].copy_from_slice(&INTPT_SPRITES);
        RAM { mem }
    }
}

impl RAM {
    pub fn borrow_memory_range_mut(&mut self, address: usize, range: usize) -> &mut [u8] {
        &mut self.mem[address..address+range]
    }

    pub fn borrow_memory_range(&self, address: usize, range: usize) -> &[u8] {
        &self.mem[address..address+range]
    }

    pub fn borrow_memory_mut(&mut self) -> &mut [u8] {
        &mut self.mem
    }

    pub fn borrow_memory(&self) -> &[u8] {
        &self.mem
    }

    pub fn copy_program(&mut self, program_data: &[u8]) -> usize {
        let len = min(PROGRAM_MEMORY_SIZE, program_data.len());
        self.mem[RAM_INTPT_OFFSET..RAM_INTPT_OFFSET+len].copy_from_slice(&program_data[..len]);
        RAM_INTPT_OFFSET
    }

    pub fn read_instruction(&self, pc: usize) -> Result<Instruction, InstructionReadError> {
        let inst_data = (self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16;
        Instruction::read(inst_data)
    }
}