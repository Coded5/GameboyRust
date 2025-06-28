use std::{
    cell::{Ref, RefCell},
    io,
    rc::Rc,
};

use super::{
    bus::Bus, cartridge::load_cartridge, cpu::Cpu, interrupt::InterruptState, joypad::Joypad,
    ppu::Ppu, timer::Timer,
};

pub type Shared<T> = Rc<RefCell<T>>;

pub struct Gameboy {
    pub cpu: Shared<Cpu>,
    pub ppu: Shared<Ppu>,
    pub interrupt: Shared<InterruptState>,
    pub timer: Shared<Timer>,
    pub joypad: Shared<Joypad>,
    pub bus: Bus,

    pub accum_cycle: u128,
    pub can_render: bool,
}

impl Gameboy {
    pub fn new(path: &str) -> io::Result<Self> {
        let cpu = Rc::new(RefCell::new(Cpu::new()));
        let ppu = Rc::new(RefCell::new(Ppu::default()));
        let timer = Rc::new(RefCell::new(Timer::default()));
        let joypad = Rc::new(RefCell::new(Joypad::default()));
        let interrupt = Rc::new(RefCell::new(InterruptState::default()));
        let bus = Bus::new(
            load_cartridge(path)?,
            Rc::clone(&interrupt),
            Rc::clone(&ppu),
            Rc::clone(&timer),
            Rc::clone(&joypad),
        );

        Ok(Gameboy {
            cpu,
            interrupt,
            ppu,
            timer,
            joypad,
            bus,

            accum_cycle: 0u128,
            can_render: false,
        })
    }

    //TODO: Change to reference
    pub fn get_frame_buffer(&self) -> [u8; 160 * 144] {
        self.ppu.borrow().frame_buffer
    }

    pub fn tick(&mut self) {
        let cycle = self
            .cpu
            .borrow_mut()
            .step(&mut self.bus, self.interrupt.clone());

        self.timer.borrow_mut().update(cycle, &mut self.bus);
        self.ppu.borrow_mut().update(cycle, &mut self.bus);
        self.joypad.borrow_mut().update(&mut self.bus);

        self.accum_cycle += cycle as u128;

        if self.ppu.borrow().finish_frame {
            self.can_render = true;
            self.ppu.borrow_mut().finish_frame = false;
        }
    }

    pub fn no_bootrom_init(&mut self) {
        let mut cpu = self.cpu.borrow_mut();

        cpu.set_af(0x01B0);
        cpu.set_bc(0x0013);
        cpu.set_de(0x00D8);
        cpu.set_hl(0x014D);

        cpu.pc = 0x100;
        cpu.sp = 0xFFFE;

        self.bus.write_byte(0xFF05, 0x00);
        self.bus.write_byte(0xFF06, 0x00);
        self.bus.write_byte(0xFF07, 0x00);
        self.bus.write_byte(0xFF10, 0x80);
        self.bus.write_byte(0xFF11, 0xBF);
        self.bus.write_byte(0xFF12, 0xF3);
        self.bus.write_byte(0xFF14, 0xBF);
        self.bus.write_byte(0xFF16, 0x3F);
        self.bus.write_byte(0xFF17, 0x00);
        self.bus.write_byte(0xFF19, 0xBF);
        self.bus.write_byte(0xFF1A, 0x7F);
        self.bus.write_byte(0xFF1B, 0xFF);
        self.bus.write_byte(0xFF1C, 0x9F);
        self.bus.write_byte(0xFF1E, 0xBF);
        self.bus.write_byte(0xFF20, 0xFF);
        self.bus.write_byte(0xFF21, 0x00);
        self.bus.write_byte(0xFF22, 0x00);
        self.bus.write_byte(0xFF23, 0xBF);
        self.bus.write_byte(0xFF24, 0x77);
        self.bus.write_byte(0xFF26, 0xF1);
        self.bus.write_byte(0xFF25, 0xF3);
        self.bus.write_byte(0xFF40, 0x91);
        self.bus.write_byte(0xFF42, 0x00);
        self.bus.write_byte(0xFF43, 0x00);
        self.bus.write_byte(0xFF45, 0x00);
        self.bus.write_byte(0xFF47, 0xFC);
        self.bus.write_byte(0xFF48, 0xFF);
        self.bus.write_byte(0xFF49, 0xFF);
        self.bus.write_byte(0xFF4A, 0);
        self.bus.write_byte(0xFF4B, 0);
        self.bus.write_byte(0xFFFF, 0x00);
        self.bus.write_byte(0xFF50, 1);
    }
}
