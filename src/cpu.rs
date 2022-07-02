use rand::Rng;
use rand::rngs::ThreadRng;
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
    stack: Vec<u16>,
    rng: ThreadRng
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
            stack: Vec::with_capacity(16),
            rng: rand::thread_rng()
        }
    }

    pub fn execute(&mut self, ram: &mut RAM, keyboard: &mut Keyboard, instruction: Instruction) -> () {
        self.program_counter += 1;
        match instruction {
            Instruction::SYS(_)
                => panic!("SYS instruction not supported"),
            Instruction::CLS
                => todo!("clear display"),
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
            Instruction::SUB(r1, r2) => {
                let (v, bf) = self.v_reg[r1 as usize].overflowing_sub(self.v_reg[r2 as usize]);
                self.v_reg[r1 as usize] = v;
                self.v_reg[0xF] = (!bf).into();
            }
            Instruction::SHR(r1, _) => {
                let v = self.v_reg[r1 as usize];
                self.v_reg[0xF] = v & 0b1;
                self.v_reg[r1 as usize] = v / 2;
            }
            Instruction::SUBN(r1, r2) => {
                let (v, bf) = self.v_reg[r2 as usize].overflowing_sub(self.v_reg[r1 as usize]);
                self.v_reg[r1 as usize] = v;
                self.v_reg[0xF] = (!bf).into();
            }
            Instruction::SHL(r1, _) => {
                let v = self.v_reg[r1 as usize];
                self.v_reg[0xF] = v & 0b1000;
                self.v_reg[r1 as usize] = v * 2;
            }
            Instruction::SNE_RR(r1, r2) => {
                if self.v_reg[r1 as usize] != self.v_reg[r2 as usize] {
                    self.program_counter += 1;
                }
            }
            Instruction::LD_IV(n) => {
                self.i_reg = n - memory::RAM_INTPT_OFFSET;
            }
            Instruction::JP_RV(n) => {
                self.program_counter = n - memory::RAM_INTPT_OFFSET + self.v_reg[0] as u16;
            }
            Instruction::RND(r, v) => {
                self.v_reg[r as usize] = self.rng.gen::<u8>() & v;
            }
            Instruction::DRW(_, _, _)
                => todo!("draw sprite"),
            Instruction::SKP(r) => {
                if keyboard.is_pressed(self.v_reg[r as usize] as usize) {
                    self.program_counter += 1;
                }
            }
            Instruction::SKNP(r) => {
                if !keyboard.is_pressed(self.v_reg[r as usize] as usize) {
                    self.program_counter += 1;
                }
            }
            Instruction::LD_RD(r) => {
                self.v_reg[r as usize] = self.delay_timer;
            }
            Instruction::LD_RK(r)
                => todo!("wait for key press"),
            Instruction::LD_DR(r) => {
                self.delay_timer = self.v_reg[r as usize];
            }
            Instruction::LD_SR(r) => {
                self.sound_timer = self.v_reg[r as usize];
            }
            Instruction::ADD_IR(r) => {
                self.i_reg += self.v_reg[r as usize] as u16;
            }
            Instruction::LD_RF(_)
                => todo!("set location of sprite"),
            Instruction::LD_BR(_)
                => todo!("bcd representation"),
            Instruction::LD_IRR(tr) => {
                let mut memory = ram.borrow_memory_range(self.i_reg as usize, (tr + 1) as usize);
                for i in 0usize..=tr.into() {
                    memory[i] = self.v_reg[i];
                }
            }
            Instruction::LD_RRI(tr) => {
                let mut memory = ram.borrow_memory_range(self.i_reg as usize, (tr + 1) as usize);
                for i in 0usize..=tr.into() {
                    self.v_reg[i] = memory[i];
                }
            }
        }
    }
}