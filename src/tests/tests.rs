use crate::emulator::memory::Memory;

#[test]
fn create_memory_and_load_rom() {
    let mut memory = Memory::new();
    let res = memory.load_rom("./roms/mgb_boot.bin");

    if let Err(e) = res {
        panic!("{}", e)
    }
}
