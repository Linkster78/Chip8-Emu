use std::{env, fs};
use chip8::{Chip8, memory};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid Syntax: {} <rom_path>", args[0]);
        return;
    }

    let rom_data = fs::read(&args[1]);
    if rom_data.is_err() {
        println!("ERROR: Couldn't read ROM \"{}\"", rom_data.err().unwrap());
        return;
    }

    let rom_data = rom_data.unwrap();
    if rom_data.len() > memory::PROGRAM_MEMORY_SIZE as usize {
        println!("ERROR: This file is too big for the chip8 RAM");
        return;
    }

    let mut system = Chip8::new();
    system.load_program(&rom_data[..]);
    println!("Loaded ROM \"{}\" into memory. ({} bytes)", args[1], rom_data.len());

    loop {
        println!("{:?}", system.cpu);
        system.step();
    }
}