use std::fmt;
use std::fmt::Formatter;

pub struct InstructionReadError(u16);

impl fmt::Debug for InstructionReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "couldn't read instruction {:#06x}", self.0)
    }
}

impl fmt::Display for InstructionReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "couldn't read instruction {:#06x}", self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    SYS    (u16),
    CLS    ,
    RET    ,
    JP     (u16),
    CALL   (u16),
    SE_RV  (u8, u8),
    SNE_RV (u8, u8),
    SE_RR  (u8, u8),
    LD_RV  (u8, u8),
    ADD_RV (u8, u8),
    LD_RR  (u8, u8),
    OR     (u8, u8),
    AND    (u8, u8),
    XOR    (u8, u8),
    ADD_RR (u8, u8),
    SUB    (u8, u8),
    SHR    (u8, u8),
    SUBN   (u8, u8),
    SHL    (u8, u8),
    SNE_RR (u8, u8),
    LD_IV  (u16),
    JP_RV  (u16),
    RND    (u8, u8),
    DRW    (u8, u8, u8),
    SKP    (u8),
    SKNP   (u8),
    LD_RD  (u8),
    LD_RK  (u8),
    LD_DR  (u8),
    LD_SR  (u8),
    ADD_IR (u8),
    LD_RF  (u8),
    LD_BR  (u8),
    LD_IRR (u8),
    LD_RRI (u8)
}

impl Instruction {
    pub fn read(x: u16) -> Result<Instruction, InstructionReadError> {
        match x >> 12 {
            0x0 => match x {
                0xE0 => Ok(Instruction::CLS),
                0xEE => Ok(Instruction::RET),
                _ => Ok(Instruction::SYS(x & 0xFFF))
            },
            0x1 => Ok(Instruction::JP(x & 0xFFF)),
            0x2 => Ok(Instruction::CALL(x & 0xFFF)),
            0x3 => Ok(Instruction::SE_RV(((x & 0xF00) >> 8) as u8, (x & 0xFF) as u8)),
            0x4 => Ok(Instruction::SNE_RV(((x & 0xF00) >> 8) as u8, (x & 0xFF) as u8)),
            0x5 => Ok(Instruction::SE_RR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
            0x6 => Ok(Instruction::LD_RV(((x & 0xF00) >> 8) as u8, (x & 0xFF) as u8)),
            0x7 => Ok(Instruction::ADD_RV(((x & 0xF00) >> 8) as u8, (x & 0xFF) as u8)),
            0x8 => match x & 0xF {
                0x0 => Ok(Instruction::LD_RR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x1 => Ok(Instruction::OR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x2 => Ok(Instruction::AND(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x3 => Ok(Instruction::XOR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x4 => Ok(Instruction::ADD_RR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x5 => Ok(Instruction::SUB(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x6 => Ok(Instruction::SHR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0x7 => Ok(Instruction::SUBN(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                0xE => Ok(Instruction::SHL(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
                _ => Err(InstructionReadError(x))
            },
            0x9 => Ok(Instruction::SNE_RR(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8)),
            0xA => Ok(Instruction::LD_IV(x & 0xFFF)),
            0xB => Ok(Instruction::JP_RV(x & 0xFFF)),
            0xC => Ok(Instruction::RND(((x & 0xF00) >> 8) as u8, (x & 0xFF) as u8)),
            0xD => Ok(Instruction::DRW(((x & 0xF00) >> 8) as u8, ((x & 0xF0) >> 4) as u8, (x & 0xF) as u8)),
            0xE => match x & 0xFF {
                0x9E => Ok(Instruction::SKP(((x & 0xF00) >> 8) as u8)),
                0xEA => Ok(Instruction::SKNP(((x & 0xF00) >> 8) as u8)),
                _ => Err(InstructionReadError(x))
            },
            0xF => match x & 0xFF {
                0x07 => Ok(Instruction::LD_RD(((x & 0xF00) >> 8) as u8)),
                0x0A => Ok(Instruction::LD_RK(((x & 0xF00) >> 8) as u8)),
                0x15 => Ok(Instruction::LD_DR(((x & 0xF00) >> 8) as u8)),
                0x18 => Ok(Instruction::LD_SR(((x & 0xF00) >> 8) as u8)),
                0x1E => Ok(Instruction::ADD_IR(((x & 0xF00) >> 8) as u8)),
                0x29 => Ok(Instruction::LD_RF(((x & 0xF00) >> 8) as u8)),
                0x33 => Ok(Instruction::LD_BR(((x & 0xF00) >> 8) as u8)),
                0x55 => Ok(Instruction::LD_IRR(((x & 0xF00) >> 8) as u8)),
                0x65 => Ok(Instruction::LD_RRI(((x & 0xF00) >> 8) as u8)),
                _ => Err(InstructionReadError(x))
            },
            _ => Err(InstructionReadError(x))
        }
    }
}