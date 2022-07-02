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

impl Chip8 {
    pub fn new() -> Chip8 {
        let cpu = CPU::new();
        let ram = RAM::new();
        let keyboard = Keyboard::new();
        let display = Display::new();
        Chip8 { cpu, ram, keyboard, display }
    }

    pub fn execute(&mut self, instruction: Instruction) -> () {
        self.cpu.execute(&mut self.ram, &mut self.keyboard, &mut self.display, instruction)
    }

    pub fn step(&mut self) -> () {
        let inst = self.ram.read_instruction(self.cpu.program_counter as usize)
            .expect(&format!("Memory contained invalid instruction at position {}", self.cpu.program_counter));
        println!("-> {:?}", inst);
        self.cpu.execute(&mut self.ram, &self.keyboard, &mut self.display, inst);
    }

    pub fn load_program(&mut self, program_data: &[u8]) -> () {
        let memory = &mut self.ram;
        let program_offset = memory.copy_program(program_data);
        self.cpu.program_counter = program_offset as u16;
    }
}