use chip8::Chip8;
use chip8::instructions::Instruction;

fn main() {
    let mut system = Chip8::new();
    let inst = Instruction::read(0x8232).unwrap();
    println!("{:?}", inst);
    system.execute(inst);
}