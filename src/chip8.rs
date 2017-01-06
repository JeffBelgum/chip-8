use std::sync::{Arc, Mutex};

use std::{thread,time};

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory_bus::MemoryBus;
use opcodes::OpCode;
use sound::Sound;
use timer::Timer;
use window::Window;

pub struct Chip8 {
    cpu: Cpu,
    mem_bus: MemoryBus,
    delay_timer: Timer,
    sound_timer: Timer,
    display: Display,
    keyboard: Keyboard,
    sound: Sound,
}

impl Chip8 {
    pub fn disassemble(rom: &[u8]) {
        for chunk in rom.chunks(2) {
            let word_1 = chunk[0];
            let instr = 
                if word_1 == 0x0A {
                    0x0A00
                } else {
                    let word_2 = chunk[1];
                    // opcodes are big-endian
                    ((word_1 as u16) << 8) | (word_2 as u16)
                };
            let opcode: OpCode = instr.into();
            println!("{:04X} {:?}", instr, opcode);
        }
    }

    pub fn run(rom: &[u8]) {
        let mut mem_bus = MemoryBus::new();
        mem_bus.load_rom(rom);
        let window = Arc::new(Mutex::new(Window::new(64, 32)));

        let mut c8 = Chip8 {
            cpu: Cpu::new(),
            mem_bus: mem_bus,
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            display: Display::new(window.clone()),
            keyboard: Keyboard::new(window.clone()),
            sound: Sound::new(),
        };

        // boot sound
        // c8.sound.emit();

        loop {
        //while c8.cpu.instruction_count() < 20 {
            c8.execute_cycle();
            if c8.should_exit() {
                break;
            }
        }

        //thread::sleep(time::Duration::from_millis(10_000));

        debug!("Shutdown");
    }

    pub fn execute_cycle(&mut self) {
        self.cpu.execute_instruction(&mut self.mem_bus,
                                     &mut self.display,
                                     &mut self.keyboard,
                                     &mut self.delay_timer,
                                     &mut self.sound_timer);
        // TODO: should this happen here or at the beginning of the cycle?
        self.delay_timer.decr();
        self.sound_timer.decr();
        if self.sound_timer.get_value() > 0 {
            self.sound.emit();
        }
    }

    pub fn should_exit(&self) -> bool {
        self.cpu.should_exit()
    }
}
