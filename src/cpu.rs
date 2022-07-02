use crate::{Keyboard, memory, RAM};
use crate::instructions::Instruction;

#[derive(Debug)]
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

    pub fn execute(&mut self, ram: &mut RAM, keyboard: &mut Keyboard, instruction: Instruction) -> () {
        self.program_counter += 1;
        match instruction {
            Instruction::SYS(_) => panic!("SYS instruction not supported"),
            Instruction::CLS => todo!("clear display"),
            Instruction::RET => {
                self.program_counter = self.stack.pop().unwrap();
                self.stack_pointer -= 1;
            }
            Instruction::JP(n) => {
                let n = n - memory::RAM_INTPT_OFFSET as u16;
                self.program_counter = n;
            }
            Instruction::CALL(n) => {
                let n = n - memory::RAM_INTPT_OFFSET;
                self.stack_pointer += 1;
                self.stack.push(self.program_counter);
                self.program_counter = n;
            }
            Instruction::SE_RV(r, v) => {
                if self.v_reg[r as usize] == v {
                    self.program_counter += 1;
                }
            }
            Instruction::SNE_RV(r, v) => {
                if self.v_reg[r as usize] != v {
                    self.program_counter += 1;
                }
            }
            Instruction::SE_RR(r1, r2) => {
                if self.v_reg[r1 as usize] == self.v_reg[r2 as usize] {
                    self.program_counter += 1;
                }
            }
            Instruction::LD_RV(r, v) => {
                self.v_reg[r as usize] = v;
            }
            Instruction::ADD_RV(r, v) => {
                self.v_reg[r as usize] = self.v_reg[r as usize].overflowing_add(v).0;
            }
            Instruction::LD_RR(r1, r2) => {
                self.v_reg[r1 as usize] = self.v_reg[r2 as usize];
            }
            Instruction::OR(r1, r2) => {
                self.v_reg[r1 as usize] |= self.v_reg[r2 as usize];
            }
            Instruction::AND(r1, r2) => {
                self.v_reg[r1 as usize] &= self.v_reg[r2 as usize];
            }
            Instruction::XOR(r1, r2) => {
                self.v_reg[r1 as usize] ^= self.v_reg[r2 as usize];
            }
            Instruction::ADD_RR(r1, r2) => {
                let (v, cf) = self.v_reg[r1 as usize].overflowing_add(self.v_reg[r2 as usize]);
                self.v_reg[r1 as usize] = v;
                self.v_reg[0xF] = cf.into();
            }
            Instruction::SUB(_, _) => {}
            Instruction::SHR(_, _) => {}
            Instruction::SUBN(_, _) => {}
            Instruction::SHL(_, _) => {}
            Instruction::SNE_RR(_, _) => {}
            Instruction::LD_IV(_) => {}
            Instruction::JP_RV(_) => {}
            Instruction::RND(_, _) => {}
            Instruction::DRW(_, _, _) => {}
            Instruction::SKP(_) => {}
            Instruction::SKNP(_) => {}
            Instruction::LD_RD(_) => {}
            Instruction::LD_RK(_) => {}
            Instruction::LD_DR(_) => {}
            Instruction::LD_SR(_) => {}
            Instruction::ADD_IR(_) => {}
            Instruction::LD_RF(_) => {}
            Instruction::LD_BR(_) => {}
            Instruction::LD_IRR(_) => {}
            Instruction::LD_RRI(_) => {}
        }
    }
}