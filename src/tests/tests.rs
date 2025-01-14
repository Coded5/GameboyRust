use crate::emulator::memory::Memory;

#[test]
fn create_memory_and_load_rom() {
    let mut memory = Memory::new();
    let res = memory.load_rom("./roms/mgb_boot.bin");

    if let Err(e) = res {
        panic!("{}", e)
    }
}

#[test]
fn test_for_test() {
    assert_eq!(2, 2);
    assert_eq!(2, 3);
    assert_eq!(4, 4);
    assert_eq!(2, 1);
}
