pub mod memory;
pub mod cpu;
pub mod keyboard;
pub mod instructions;

use crate::memory::RAM;
use crate::cpu::CPU;
use crate::instructions::Instruction;
use crate::keyboard::Keyboard;

pub struct Chip8 {
    pub cpu: CPU,
    pub ram: RAM,
    pub keyboard: Keyboard
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let cpu = CPU::new();
        let ram = RAM::new();
        let keyboard = Keyboard::new();
        Chip8 { cpu, ram, keyboard }
    }

    pub fn execute(&mut self, instruction: Instruction) -> () {
        self.cpu.execute(&mut self.ram, &mut self.keyboard, instruction)
    }
}