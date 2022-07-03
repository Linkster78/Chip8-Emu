pub mod memory;
pub mod cpu;
pub mod instructions;
pub mod keyboard;
pub mod display;

use crate::memory::RAM;
use crate::cpu::CPU;
use crate::display::Display;
use crate::instructions::Instruction;
use crate::keyboard::Keyboard;

pub struct Chip8 {
    pub cpu: CPU,
    pub ram: RAM,
    pub keyboard: Keyboard,
    pub display: Display
}

impl Default for Chip8 {
    fn default() -> Self {
        let cpu = CPU::default();
        let ram = RAM::default();
        let keyboard = Keyboard::default();
        let display = Display::default();
        Chip8 { cpu, ram, keyboard, display }
    }
}

impl Chip8 {
    pub fn execute(&mut self, instruction: Instruction) {
        self.cpu.execute(&mut self.ram, &self.keyboard, &mut self.display, instruction)
    }

    pub fn step(&mut self) {
        let inst = self.ram.read_instruction(self.cpu.program_counter as usize)
            .unwrap_or_else(|_| panic!("Memory contained invalid instruction at position {}", self.cpu.program_counter));
        self.cpu.execute(&mut self.ram, &self.keyboard, &mut self.display, inst);
    }

    pub fn load_program(&mut self, program_data: &[u8]) {
        let memory = &mut self.ram;
        let program_offset = memory.copy_program(program_data);
        self.cpu.program_counter = program_offset as u16;
    }
}