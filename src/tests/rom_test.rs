use crate::emulator::gameboy::Gameboy;

macro_rules! rom_test {
    ($($name:ident => $file:literal),*) => {
        $(
            #[test]
            fn $name() {
                let mut gameboy = Gameboy::new(&format!("./roms/{}", $file)).unwrap();

                loop {
                    gameboy.tick();

                    if gameboy.memory.read_byte(gameboy.cpu.pc) == 0x40 {
                        break;
                    }
                }

                assert_eq!(gameboy.cpu.b, 3);
                assert_eq!(gameboy.cpu.c, 5);
                assert_eq!(gameboy.cpu.d, 8);
                assert_eq!(gameboy.cpu.e, 13);
                assert_eq!(gameboy.cpu.h, 21);
                assert_eq!(gameboy.cpu.l, 34);
            }
        )*
    };
}

rom_test!(add_sp_e_timing => "acceptance/add_sp_e_timing");
rom_test!(boot_div_dmg0 => "acceptance/boot_div-dmg0");
rom_test!(boot_div_dmg_abcmgb => "acceptance/boot_div-dmgABCmgb");
rom_test!(boot_hwio_dmg0 => "acceptance/boot_hwio-dmg0");
rom_test!(boot_hwio_dmg_abcmgb => "acceptance/boot_hwio-dmgABCmgb");
rom_test!(boot_regs_dmg0 => "acceptance/boot_regs-dmg0");
rom_test!(boot_regs_dmg_abc => "acceptance/boot_regs-dmgABC");
rom_test!(call_cc_timing => "acceptance/call_cc_timing");
rom_test!(call_cc_timing2 => "acceptance/call_cc_timing2");
rom_test!(call_timing => "acceptance/call_timing");
rom_test!(call_timing2 => "acceptance/call_timing2");
rom_test!(div_timing => "acceptance/div_timing");
rom_test!(ei_sequence => "acceptance/ei_sequence");
rom_test!(ei_timing => "acceptance/ei_timing");
rom_test!(halt_ime0_ei => "acceptance/halt_ime0_ei");
rom_test!(halt_ime0_nointr_timing => "acceptance/halt_ime0_nointr_timing");
rom_test!(halt_ime1_timing => "acceptance/halt_ime1_timing");
rom_test!(if_ie_registers => "acceptance/if_ie_registers");
rom_test!(intr_timing => "acceptance/intr_timing");
rom_test!(jp_cc_timing => "acceptance/jp_cc_timing");
rom_test!(jp_timing => "acceptance/jp_timing");
rom_test!(ld_hl_sp_e_timing => "acceptance/ld_hl_sp_e_timing");
rom_test!(oam_dma_restart => "acceptance/oam_dma_restart");
rom_test!(oam_dma_start => "acceptance/oam_dma_start");
rom_test!(oam_dma_timing => "acceptance/oam_dma_timing");
rom_test!(pop_timing => "acceptance/pop_timing");
rom_test!(push_timing => "acceptance/push_timing");
rom_test!(rapid_di_ei => "acceptance/rapid_di_ei");
rom_test!(ret_cc_timing => "acceptance/ret_cc_timing");
rom_test!(ret_timing => "acceptance/ret_timing");
rom_test!(reti_intr_timing => "acceptance/reti_intr_timing");
rom_test!(reti_timing => "acceptance/reti_timing");
rom_test!(rst_timing => "acceptance/rst_timing");

// #[test]
// fn timer_div_write() {
//     let mut gameboy = Gameboy::new("./roms/acceptance/timer/div_write.gb").unwrap();
//
//     loop {
//         gameboy.tick();
//
//         if gameboy.memory.read_byte(gameboy.cpu.pc) == 0x40 {
//             break;
//         }
//     }
//
//     assert_eq!(gameboy.cpu.b, 3);
//     assert_eq!(gameboy.cpu.c, 5);
//     assert_eq!(gameboy.cpu.d, 8);
//     assert_eq!(gameboy.cpu.e, 13);
//     assert_eq!(gameboy.cpu.h, 21);
//     assert_eq!(gameboy.cpu.l, 34);
// }
