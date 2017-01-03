use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory_bus::MemoryBus;
use sound::Sound;
use timer::Timer;

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
        let keyboard = Keyboard::new();
        let delay_timer = Timer::new();
        let sound_timer = Timer::new();

        let mut c8 = Chip8 {
            cpu: Cpu::new(),
            mem_bus: mem_bus,
            delay_timer: delay_timer,
            sound_timer: sound_timer,
            display: display,
            keyboard: keyboard,
            sound: Sound {},
        };

        while c8.cpu.instruction_count() < 1 {
            c8.execute_cycle();
            if c8.should_exit() {
                debug!("Exiting");
                return;
            }
        }
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
