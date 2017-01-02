use cpu::Cpu;
use display::Display;
use memory_bus::MemoryBus;

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
    pub fn run(rom: &[u8]) {
        let mut mem_bus = MemoryBus::new();
        mem_bus.load_rom(rom);
        let display = Display::new();
        let mut c8 = Chip8 {
            cpu: Cpu::new(),
            mem_bus: mem_bus,
            delay_timer: Timer {},
            sound_timer: Timer {},
            display: display,
            keyboard: Keyboard {},
            sound: Sound {},
        };

        loop {
        // while c8.cpu.instruction_count() < 1024 {
            c8.cpu.execute_instruction(&mut c8.mem_bus, &mut c8.display);
        }
    }
}

struct Timer {}
struct Keyboard {}
struct Sound {}
