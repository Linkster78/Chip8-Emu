use std::{env, fs, thread};
use std::path::Path;
use std::time::Duration;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use chip8::{Chip8, display, memory};
use chip8::cpu::Coordinator;
use chip8::keyboard::KeyEvent::{Pressed, Released};

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = 480;
const COLOR_CLEAR: Color = Color::RGB(0, 0, 0);
const COLOR_CONTRAST: Color = Color::RGB(255, 255, 255);

const CYCLES_PER_SECOND: u32 = 500;

const KEYPAD_TABLE: [Keycode; 16] = [
    Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Num4,
    Keycode::Q,    Keycode::W,    Keycode::E,    Keycode::R,
    Keycode::A,    Keycode::S,    Keycode::D,    Keycode::F,
    Keycode::Z,    Keycode::X,    Keycode::C,    Keycode::V,
];

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

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

    let mut system = Chip8::default();
    system.load_program(&rom_data[..]);
    println!("Loaded ROM \"{}\" into memory. ({} bytes)", args[1], rom_data.len());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    println!("Initialized the SDL2 context and video subsystem.");

    let rom_name = Path::new(&args[1]).file_name().unwrap();
    let window = video_subsystem.window(format!("chip8-emulator: {:?}", rom_name).as_str(), SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    println!("Created the SDL2 window.");

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();
    let mut is_playing_tone = false;
    println!("Opened audio playback for tone generation.");

    let mut cpu_coordinator = Coordinator::new(CYCLES_PER_SECOND);
    let mut timer_coordinator = Coordinator::new(60);

    'running: loop {
        if cpu_coordinator.should_cycle() {
            // pull keyboard events and pass them to the chip8 keyboard
            let mut key_events = Vec::new();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(kc), .. } => {
                        if KEYPAD_TABLE.contains(&kc) {
                            let index = KEYPAD_TABLE.iter().position(|&s| s == kc);
                            if let Some(index) = index {
                                key_events.push(Pressed(index as u8));
                            }
                        }
                    },
                    Event::KeyUp { keycode: Some(kc), .. } => {
                        if KEYPAD_TABLE.contains(&kc) {
                            let index = KEYPAD_TABLE.iter().position(|&s| s == kc);
                            if let Some(index) = index {
                                key_events.push(Released(index as u8));
                            }
                        }
                    },
                    _ => {}
                }
            }
            system.keyboard.update_key_states(key_events);

            // step the current instruction
            system.step();

            // if a rendering instruction was called, re-render the screen
            if system.display.dirty {
                canvas.set_draw_color(COLOR_CLEAR);
                canvas.clear();

                let display_data = system.display.borrow_display();

                canvas.set_draw_color(COLOR_CONTRAST);
                let rect_width = SCREEN_WIDTH / display::DISPLAY_WIDTH as u32;
                let rect_height = SCREEN_HEIGHT / display::DISPLAY_HEIGHT as u32;

                let mut rects = Vec::new();
                for x in 0..display::DISPLAY_WIDTH as i32 {
                    for y in 0..display::DISPLAY_HEIGHT as i32 {
                        if display_data[x as usize][y as usize] {
                            rects.push(Rect::new(x * rect_width as i32, y * rect_height as i32, rect_width, rect_height));
                        }
                    }
                }
                canvas.fill_rects(&rects[..]).ok();
                canvas.present();

                system.display.dirty = false;
            }

            // play a tone if the cpu's sound timer is positive
            if system.cpu.is_tone_on() {
                if !is_playing_tone {
                    device.resume();
                }
            } else if is_playing_tone {
                device.pause();
            }
            is_playing_tone = system.cpu.is_tone_on();
        }

        if timer_coordinator.should_cycle() {
            // countdown the timers of the cpu
            system.cpu.countdown_timers();
        }

        let coordinators = [&cpu_coordinator, &timer_coordinator];
        thread::sleep(Duration::new(0, Coordinator::smallest_delay_until_cycle(&coordinators) as u32));
    }
}