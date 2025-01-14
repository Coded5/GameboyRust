use gameboy::emulator::{cpu::Cpu, memory::Memory};

fn main() {
    let cpu = Cpu::new();
    let mut memory = Memory::new();

    let _ = memory.load_rom("./roms/mgb_boot.bin");
    cpu.run_rom(&mut memory, 256);
}
