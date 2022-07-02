use chip8::Chip8;
use chip8::instructions::Instruction;

fn main() {
    let mut system = Chip8::new();
    system.execute(Instruction::LD_RV(0, 255));
    system.execute(Instruction::LD_RV(1, 255));
    system.execute(Instruction::ADD_RR(0, 1));
    println!("{:?}", system.cpu);
}