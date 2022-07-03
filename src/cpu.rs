use std::cmp::max;
use std::time::SystemTime;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::instructions::Instruction;
use crate::{Keyboard, RAM, Display, memory};

#[derive(Debug)]
pub struct CPU {
    v_reg: [u8; 16],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub program_counter: u16,
    stack: Vec<u16>,
    rng: ThreadRng
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            v_reg: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack: Vec::with_capacity(16),
            rng: rand::thread_rng()
        }
    }
}

impl CPU {
    pub fn execute(&mut self, ram: &mut RAM, keyboard: &Keyboard, display: &mut Display, instruction: Instruction) {
        self.program_counter += 2;

        match instruction {
            Instruction::SYS(_)
                => panic!("SYS instruction not supported"),
            Instruction::CLS => {
                display.clear();
            }
            Instruction::RET => {
                self.program_counter = self.stack.pop().unwrap();
            }
            Instruction::JP(n) => {
                let n = n;
                self.program_counter = n;
            }
            Instruction::CALL(n) => {
                let n = n;
                self.stack.push(self.program_counter);
                self.program_counter = n;
            }
            Instruction::SE_RV(r, v) => {
                if self.v_reg[r as usize] == v {
                    self.program_counter += 2;
                }
            }
            Instruction::SNE_RV(r, v) => {
                if self.v_reg[r as usize] != v {
                    self.program_counter += 2;
                }
            }
            Instruction::SE_RR(r1, r2) => {
                if self.v_reg[r1 as usize] == self.v_reg[r2 as usize] {
                    self.program_counter += 2;
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
                    self.program_counter += 2;
                }
            }
            Instruction::LD_IV(n) => {
                self.i_reg = n;
            }
            Instruction::JP_RV(n) => {
                self.program_counter = n - self.v_reg[0] as u16;
            }
            Instruction::RND(r, v) => {
                self.v_reg[r as usize] = self.rng.gen::<u8>() & v;
            }
            Instruction::DRW(x, y, n) => {
                let sprite_data = ram.borrow_memory_range(self.i_reg as usize, n as usize);
                self.v_reg[0xF] = display.draw_sprite(self.v_reg[x as usize], self.v_reg[y as usize], sprite_data).into();
            },
            Instruction::SKP(r) => {
                if keyboard.is_pressed(self.v_reg[r as usize] as usize) {
                    self.program_counter += 2;
                }
            }
            Instruction::SKNP(r) => {
                if !keyboard.is_pressed(self.v_reg[r as usize] as usize) {
                    self.program_counter += 2;
                }
            }
            Instruction::LD_RD(r) => {
                self.v_reg[r as usize] = self.delay_timer;
            }
            Instruction::LD_RK(r) => {
                if keyboard.last_pressed.is_some() {
                    self.v_reg[r as usize] = keyboard.last_pressed.unwrap();
                } else {
                    self.program_counter -= 2;
                }
            },
            Instruction::LD_DR(r) => {
                self.delay_timer = self.v_reg[r as usize];
            }
            Instruction::LD_SR(r) => {
                self.sound_timer = self.v_reg[r as usize];
            }
            Instruction::ADD_IR(r) => {
                self.i_reg += self.v_reg[r as usize] as u16;
            }
            Instruction::LD_RF(r) => {
                self.i_reg = (self.v_reg[r as usize] as usize * memory::INTPT_SPRITE_LENGTH) as u16;
            },
            Instruction::LD_BR(r) => {
                let val = self.v_reg[r as usize];
                let memory = ram.borrow_memory_range_mut(self.i_reg as usize, 3);
                memory[0] = val / 100;
                memory[1] = val % 100 / 10;
                memory[2] = val % 10;
            },
            Instruction::LD_IRR(tr) => {
                let memory = ram.borrow_memory_range_mut(self.i_reg as usize, (tr + 1) as usize);
                memory[..=tr as usize].copy_from_slice(&self.v_reg[..=tr as usize]);
            }
            Instruction::LD_RRI(tr) => {
                let memory = ram.borrow_memory_range(self.i_reg as usize, (tr + 1) as usize);
                self.v_reg[..=tr as usize].copy_from_slice(&memory[..=tr as usize]);
            }
        }
    }

    pub fn countdown_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn is_tone_on(&self) -> bool {
        self.sound_timer > 0
    }
}

pub struct Coordinator {
    pub rate: u32,
    offset: u128,
    last_execution: u128
}

impl Coordinator {
    pub fn new(rate: u32) -> Self {
        Coordinator {
            rate,
            offset: 1_000_000_000 / rate as u128,
            last_execution: 0
        }
    }

    pub fn should_cycle(&mut self) -> bool {
        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos();
        if timestamp_nanos - self.last_execution >= self.offset {
            self.last_execution = timestamp_nanos;
            true
        } else {
            false
        }
    }

    pub fn delay_until_cycle(&self) -> u128 {
        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos();
        max((self.last_execution + self.offset) as i128 - timestamp_nanos as i128, 0) as u128
    }

    pub fn smallest_delay_until_cycle(coordinators: &[&Coordinator]) -> u128 {
        coordinators.iter().map(|coordinator| coordinator.delay_until_cycle())
            .min().unwrap()
    }
}