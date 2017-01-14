use std::{fmt, io, thread};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use time;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory_bus::MemoryBus;
use opcodes::OpCode;
use sound::Sound;
use timer::Timer;
use window::Window;

const CPU_SPEED_NS: i64 = 1_000_000_000 / 500;

pub struct Chip8 {
    cpu: Cpu,
    mem_bus: MemoryBus,
    delay_timer: Timer,
    sound_timer: Timer,
    display: Display,
    keyboard: Keyboard,
    sound: Sound,
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO FIXME: if timers are ever run out of lockstep from cpu, the values will be wrong
        // here
        write!(f, "{:?} dt={:#02X} st={:#02X}", self.cpu, self.delay_timer.get_value(), self.sound_timer.get_value())
    }
}

impl Chip8 {
    pub fn disassemble(rom: &[u8]) {
        let mut count = 0x200;
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
            println!("{:#03X} {:04X}    {}", count, instr, opcode);
            count += 2;
        }
    }

    pub fn run(rom: &[u8], step: bool, cycles: Option<u64>) {
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

        let start_time = time::get_time();

        // boot sound
        c8.sound.emit();

        // either loop at most some specified number of cycles or loop infinitely until rom exit
        match cycles {
            Some(cycles) => while c8.cpu.instruction_count() < cycles {
                if c8._run(step) {
                    break
                }
            },
            None => loop { 
                if c8._run(step) { 
                    break 
                } 
            },
        }

        let end_time = time::get_time();

        info!("Shutdown -- elapsed time {:?}", end_time - start_time);
    }

    fn _run(&mut self, step: bool) -> bool {
        self.execute_cycle();

        if self.should_exit() {
            return true;
        }

        if step {
            println!("{:?}", self);
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
        } else {
            debug!("{:?}", self);
        }

        false
    }

    pub fn execute_cycle(&mut self) {
        let cycle_start = time::get_time();
        if self.sound_timer.get_value() > 0 {
            self.sound.emit();
        }
        self.cpu.execute_instruction(&mut self.mem_bus,
                                     &mut self.display,
                                     &mut self.keyboard,
                                     &mut self.delay_timer,
                                     &mut self.sound_timer);
        // TODO: should this happen here or at the beginning of the cycle?
        self.delay_timer.cycle();
        self.sound_timer.cycle();
        let cycle_end = time::get_time();
        let cycle_dur = (cycle_end - cycle_start).num_nanoseconds().unwrap();
        debug!("cycle duration: {} micros", cycle_dur / 1000);
        if cycle_dur < CPU_SPEED_NS {
            let nanos = CPU_SPEED_NS - cycle_dur;
            thread::sleep(Duration::new(0, nanos as u32));
        }
    }

    pub fn should_exit(&self) -> bool {
        self.cpu.should_exit()
    }
}
