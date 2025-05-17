use std::{fs, io};

use serde::Deserialize;

use super::serde_helper;
use crate::emulator::{cpu::Cpu, memory::Memory};

fn ei_default() -> u8 {
    0
}

//TODO: remove pub
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CpuState {
    pc: u16,
    sp: u16,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    ime: u8,

    #[serde(default = "ei_default")]
    ei: u8,

    #[serde(with = "serde_helper")]
    ram: Vec<(u16, u8)>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CpuTestCase {
    name: String,
    #[serde(rename = "initial")]
    initial_state: CpuState,
    #[serde(rename = "final")]
    final_state: CpuState,
}

fn load_test_cases() -> Result<Vec<CpuTestCase>, io::Error> {
    let path = String::from("./sm83/v1/");
    let res: Vec<Vec<CpuTestCase>> = fs::read_dir(path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "json" {
                let contents = fs::read_to_string(path).ok()?;
                serde_json::from_str::<Vec<CpuTestCase>>(&contents).ok()
            } else {
                None
            }
        })
        .collect();
    Ok(res.into_iter().flatten().collect())
}

fn load_test_cases_from_json(path: &str) -> Result<Vec<CpuTestCase>, io::Error> {
    let contents = fs::read_to_string(path)?;
    let res = serde_json::from_str::<Vec<CpuTestCase>>(&contents)?;
    Ok(res)
}

macro_rules! generate_tests {
    ($($name:ident => $file:literal),*) => {
        $(
            #[test]
            fn $name() {
                let mut cpu: Cpu = Cpu::new();
                let mut memory: Memory = Memory::new();
                let cases = load_test_cases_from_json(concat!("sm83/v1/", $file)).unwrap();

                for case in cases {
                    let test_name = case.name;
                    let initial_state = case.initial_state;
                    let final_state = case.final_state;

                    cpu.sp = initial_state.sp;
                    cpu.pc = initial_state.pc;
                    cpu.a = initial_state.a;
                    cpu.f = initial_state.f;
                    cpu.b = initial_state.b;
                    cpu.c = initial_state.c;
                    cpu.d = initial_state.d;
                    cpu.e = initial_state.e;
                    cpu.h = initial_state.h;
                    cpu.l = initial_state.l;
                    cpu.ei = initial_state.ei;
                    cpu.ime = initial_state.ime != 0;

                    for (address, value) in initial_state.ram {
                        *memory.get_mut_byte(address) = value;
                    }

                    cpu.run(&mut memory);

                    assert_eq!(cpu.pc, final_state.pc, "testing {} on PC", test_name);
                    assert_eq!(cpu.sp, final_state.sp, "testing {} on SP", test_name);
                    assert_eq!(cpu.a, final_state.a, "testing {} on register a", test_name);
                    assert_eq!(cpu.f, final_state.f, "testing {} on register f", test_name);
                    assert_eq!(cpu.b, final_state.b, "testing {} on register b", test_name);
                    assert_eq!(cpu.c, final_state.c, "testing {} on register c", test_name);
                    assert_eq!(cpu.d, final_state.d, "testing {} on register d", test_name);
                    assert_eq!(cpu.e, final_state.e, "testing {} on register e", test_name);
                    assert_eq!(cpu.h, final_state.h, "testing {} on register h", test_name);
                    assert_eq!(cpu.l, final_state.l, "testing {} on register l", test_name);
                    assert_eq!(cpu.ei, final_state.ei, "testing {} on EI", test_name);
                    assert_eq!(
                        cpu.ime,
                        final_state.ime != 0,
                        "testing {} on IME",
                        test_name
                    );

                    for (address, value) in final_state.ram {
                        assert_eq!(
                            memory.get_byte(address),
                            value,
                            "testing {} on memory address {} should be {}",
                            test_name,
                            address,
                            value
                        );
                    }
                }
            }
        )*
    };
}

generate_tests!(
test_instruction_00 => "00.json",
test_instruction_01 => "01.json",
test_instruction_02 => "02.json",
test_instruction_03 => "03.json",
test_instruction_04 => "04.json",
test_instruction_05 => "05.json",
test_instruction_06 => "06.json",
test_instruction_07 => "07.json",
test_instruction_08 => "08.json",
test_instruction_09 => "09.json",
test_instruction_0a => "0a.json",
test_instruction_0b => "0b.json",
test_instruction_0c => "0c.json",
test_instruction_0d => "0d.json",
test_instruction_0e => "0e.json",
test_instruction_0f => "0f.json",
test_instruction_10 => "10.json",
test_instruction_11 => "11.json",
test_instruction_12 => "12.json",
test_instruction_13 => "13.json",
test_instruction_14 => "14.json",
test_instruction_15 => "15.json",
test_instruction_16 => "16.json",
test_instruction_17 => "17.json",
test_instruction_18 => "18.json",
test_instruction_19 => "19.json",
test_instruction_1a => "1a.json",
test_instruction_1b => "1b.json",
test_instruction_1c => "1c.json",
test_instruction_1d => "1d.json",
test_instruction_1e => "1e.json",
test_instruction_1f => "1f.json",
test_instruction_20 => "20.json",
test_instruction_21 => "21.json",
test_instruction_22 => "22.json",
test_instruction_23 => "23.json",
test_instruction_24 => "24.json",
test_instruction_25 => "25.json",
test_instruction_26 => "26.json",
test_instruction_27 => "27.json",
test_instruction_28 => "28.json",
test_instruction_29 => "29.json",
test_instruction_2a => "2a.json",
test_instruction_2b => "2b.json",
test_instruction_2c => "2c.json",
test_instruction_2d => "2d.json",
test_instruction_2e => "2e.json",
test_instruction_2f => "2f.json",
test_instruction_30 => "30.json",
test_instruction_31 => "31.json",
test_instruction_32 => "32.json",
test_instruction_33 => "33.json",
test_instruction_34 => "34.json",
test_instruction_35 => "35.json",
test_instruction_36 => "36.json",
test_instruction_37 => "37.json",
test_instruction_38 => "38.json",
test_instruction_39 => "39.json",
test_instruction_3a => "3a.json",
test_instruction_3b => "3b.json",
test_instruction_3c => "3c.json",
test_instruction_3d => "3d.json",
test_instruction_3e => "3e.json",
test_instruction_3f => "3f.json",
test_instruction_40 => "40.json",
test_instruction_41 => "41.json",
test_instruction_42 => "42.json",
test_instruction_43 => "43.json",
test_instruction_44 => "44.json",
test_instruction_45 => "45.json",
test_instruction_46 => "46.json",
test_instruction_47 => "47.json",
test_instruction_48 => "48.json",
test_instruction_49 => "49.json",
test_instruction_4a => "4a.json",
test_instruction_4b => "4b.json",
test_instruction_4c => "4c.json",
test_instruction_4d => "4d.json",
test_instruction_4e => "4e.json",
test_instruction_4f => "4f.json",
test_instruction_50 => "50.json",
test_instruction_51 => "51.json",
test_instruction_52 => "52.json",
test_instruction_53 => "53.json",
test_instruction_54 => "54.json",
test_instruction_55 => "55.json",
test_instruction_56 => "56.json",
test_instruction_57 => "57.json",
test_instruction_58 => "58.json",
test_instruction_59 => "59.json",
test_instruction_5a => "5a.json",
test_instruction_5b => "5b.json",
test_instruction_5c => "5c.json",
test_instruction_5d => "5d.json",
test_instruction_5e => "5e.json",
test_instruction_5f => "5f.json",
test_instruction_60 => "60.json",
test_instruction_61 => "61.json",
test_instruction_62 => "62.json",
test_instruction_63 => "63.json",
test_instruction_64 => "64.json",
test_instruction_65 => "65.json",
test_instruction_66 => "66.json",
test_instruction_67 => "67.json",
test_instruction_68 => "68.json",
test_instruction_69 => "69.json",
test_instruction_6a => "6a.json",
test_instruction_6b => "6b.json",
test_instruction_6c => "6c.json",
test_instruction_6d => "6d.json",
test_instruction_6e => "6e.json",
test_instruction_6f => "6f.json",
test_instruction_70 => "70.json",
test_instruction_71 => "71.json",
test_instruction_72 => "72.json",
test_instruction_73 => "73.json",
test_instruction_74 => "74.json",
test_instruction_75 => "75.json",
test_instruction_76 => "76.json",
test_instruction_77 => "77.json",
test_instruction_78 => "78.json",
test_instruction_79 => "79.json",
test_instruction_7a => "7a.json",
test_instruction_7b => "7b.json",
test_instruction_7c => "7c.json",
test_instruction_7d => "7d.json",
test_instruction_7e => "7e.json",
test_instruction_7f => "7f.json",
test_instruction_80 => "80.json",
test_instruction_81 => "81.json",
test_instruction_82 => "82.json",
test_instruction_83 => "83.json",
test_instruction_84 => "84.json",
test_instruction_85 => "85.json",
test_instruction_86 => "86.json",
test_instruction_87 => "87.json",
test_instruction_88 => "88.json",
test_instruction_89 => "89.json",
test_instruction_8a => "8a.json",
test_instruction_8b => "8b.json",
test_instruction_8c => "8c.json",
test_instruction_8d => "8d.json",
test_instruction_8e => "8e.json",
test_instruction_8f => "8f.json",
test_instruction_90 => "90.json",
test_instruction_91 => "91.json",
test_instruction_92 => "92.json",
test_instruction_93 => "93.json",
test_instruction_94 => "94.json",
test_instruction_95 => "95.json",
test_instruction_96 => "96.json",
test_instruction_97 => "97.json",
test_instruction_98 => "98.json",
test_instruction_99 => "99.json",
test_instruction_9a => "9a.json",
test_instruction_9b => "9b.json",
test_instruction_9c => "9c.json",
test_instruction_9d => "9d.json",
test_instruction_9e => "9e.json",
test_instruction_9f => "9f.json",
test_instruction_a0 => "a0.json",
test_instruction_a1 => "a1.json",
test_instruction_a2 => "a2.json",
test_instruction_a3 => "a3.json",
test_instruction_a4 => "a4.json",
test_instruction_a5 => "a5.json",
test_instruction_a6 => "a6.json",
test_instruction_a7 => "a7.json",
test_instruction_a8 => "a8.json",
test_instruction_a9 => "a9.json",
test_instruction_aa => "aa.json",
test_instruction_ab => "ab.json",
test_instruction_ac => "ac.json",
test_instruction_ad => "ad.json",
test_instruction_ae => "ae.json",
test_instruction_af => "af.json",
test_instruction_b0 => "b0.json",
test_instruction_b1 => "b1.json",
test_instruction_b2 => "b2.json",
test_instruction_b3 => "b3.json",
test_instruction_b4 => "b4.json",
test_instruction_b5 => "b5.json",
test_instruction_b6 => "b6.json",
test_instruction_b7 => "b7.json",
test_instruction_b8 => "b8.json",
test_instruction_b9 => "b9.json",
test_instruction_ba => "ba.json",
test_instruction_bb => "bb.json",
test_instruction_bc => "bc.json",
test_instruction_bd => "bd.json",
test_instruction_be => "be.json",
test_instruction_bf => "bf.json",
test_instruction_c0 => "c0.json",
test_instruction_c1 => "c1.json",
test_instruction_c2 => "c2.json",
test_instruction_c3 => "c3.json",
test_instruction_c4 => "c4.json",
test_instruction_c5 => "c5.json",
test_instruction_c6 => "c6.json",
test_instruction_c7 => "c7.json",
test_instruction_c8 => "c8.json",
test_instruction_c9 => "c9.json",
test_instruction_ca => "ca.json",
test_instruction_cb_00 => "cb 00.json",
test_instruction_cb_01 => "cb 01.json",
test_instruction_cb_02 => "cb 02.json",
test_instruction_cb_03 => "cb 03.json",
test_instruction_cb_04 => "cb 04.json",
test_instruction_cb_05 => "cb 05.json",
test_instruction_cb_06 => "cb 06.json",
test_instruction_cb_07 => "cb 07.json",
test_instruction_cb_08 => "cb 08.json",
test_instruction_cb_09 => "cb 09.json",
test_instruction_cb_0a => "cb 0a.json",
test_instruction_cb_0b => "cb 0b.json",
test_instruction_cb_0c => "cb 0c.json",
test_instruction_cb_0d => "cb 0d.json",
test_instruction_cb_0e => "cb 0e.json",
test_instruction_cb_0f => "cb 0f.json",
test_instruction_cb_10 => "cb 10.json",
test_instruction_cb_11 => "cb 11.json",
test_instruction_cb_12 => "cb 12.json",
test_instruction_cb_13 => "cb 13.json",
test_instruction_cb_14 => "cb 14.json",
test_instruction_cb_15 => "cb 15.json",
test_instruction_cb_16 => "cb 16.json",
test_instruction_cb_17 => "cb 17.json",
test_instruction_cb_18 => "cb 18.json",
test_instruction_cb_19 => "cb 19.json",
test_instruction_cb_1a => "cb 1a.json",
test_instruction_cb_1b => "cb 1b.json",
test_instruction_cb_1c => "cb 1c.json",
test_instruction_cb_1d => "cb 1d.json",
test_instruction_cb_1e => "cb 1e.json",
test_instruction_cb_1f => "cb 1f.json",
test_instruction_cb_20 => "cb 20.json",
test_instruction_cb_21 => "cb 21.json",
test_instruction_cb_22 => "cb 22.json",
test_instruction_cb_23 => "cb 23.json",
test_instruction_cb_24 => "cb 24.json",
test_instruction_cb_25 => "cb 25.json",
test_instruction_cb_26 => "cb 26.json",
test_instruction_cb_27 => "cb 27.json",
test_instruction_cb_28 => "cb 28.json",
test_instruction_cb_29 => "cb 29.json",
test_instruction_cb_2a => "cb 2a.json",
test_instruction_cb_2b => "cb 2b.json",
test_instruction_cb_2c => "cb 2c.json",
test_instruction_cb_2d => "cb 2d.json",
test_instruction_cb_2e => "cb 2e.json",
test_instruction_cb_2f => "cb 2f.json",
test_instruction_cb_30 => "cb 30.json",
test_instruction_cb_31 => "cb 31.json",
test_instruction_cb_32 => "cb 32.json",
test_instruction_cb_33 => "cb 33.json",
test_instruction_cb_34 => "cb 34.json",
test_instruction_cb_35 => "cb 35.json",
test_instruction_cb_36 => "cb 36.json",
test_instruction_cb_37 => "cb 37.json",
test_instruction_cb_38 => "cb 38.json",
test_instruction_cb_39 => "cb 39.json",
test_instruction_cb_3a => "cb 3a.json",
test_instruction_cb_3b => "cb 3b.json",
test_instruction_cb_3c => "cb 3c.json",
test_instruction_cb_3d => "cb 3d.json",
test_instruction_cb_3e => "cb 3e.json",
test_instruction_cb_3f => "cb 3f.json",
test_instruction_cb_40 => "cb 40.json",
test_instruction_cb_41 => "cb 41.json",
test_instruction_cb_42 => "cb 42.json",
test_instruction_cb_43 => "cb 43.json",
test_instruction_cb_44 => "cb 44.json",
test_instruction_cb_45 => "cb 45.json",
test_instruction_cb_46 => "cb 46.json",
test_instruction_cb_47 => "cb 47.json",
test_instruction_cb_48 => "cb 48.json",
test_instruction_cb_49 => "cb 49.json",
test_instruction_cb_4a => "cb 4a.json",
test_instruction_cb_4b => "cb 4b.json",
test_instruction_cb_4c => "cb 4c.json",
test_instruction_cb_4d => "cb 4d.json",
test_instruction_cb_4e => "cb 4e.json",
test_instruction_cb_4f => "cb 4f.json",
test_instruction_cb_50 => "cb 50.json",
test_instruction_cb_51 => "cb 51.json",
test_instruction_cb_52 => "cb 52.json",
test_instruction_cb_53 => "cb 53.json",
test_instruction_cb_54 => "cb 54.json",
test_instruction_cb_55 => "cb 55.json",
test_instruction_cb_56 => "cb 56.json",
test_instruction_cb_57 => "cb 57.json",
test_instruction_cb_58 => "cb 58.json",
test_instruction_cb_59 => "cb 59.json",
test_instruction_cb_5a => "cb 5a.json",
test_instruction_cb_5b => "cb 5b.json",
test_instruction_cb_5c => "cb 5c.json",
test_instruction_cb_5d => "cb 5d.json",
test_instruction_cb_5e => "cb 5e.json",
test_instruction_cb_5f => "cb 5f.json",
test_instruction_cb_60 => "cb 60.json",
test_instruction_cb_61 => "cb 61.json",
test_instruction_cb_62 => "cb 62.json",
test_instruction_cb_63 => "cb 63.json",
test_instruction_cb_64 => "cb 64.json",
test_instruction_cb_65 => "cb 65.json",
test_instruction_cb_66 => "cb 66.json",
test_instruction_cb_67 => "cb 67.json",
test_instruction_cb_68 => "cb 68.json",
test_instruction_cb_69 => "cb 69.json",
test_instruction_cb_6a => "cb 6a.json",
test_instruction_cb_6b => "cb 6b.json",
test_instruction_cb_6c => "cb 6c.json",
test_instruction_cb_6d => "cb 6d.json",
test_instruction_cb_6e => "cb 6e.json",
test_instruction_cb_6f => "cb 6f.json",
test_instruction_cb_70 => "cb 70.json",
test_instruction_cb_71 => "cb 71.json",
test_instruction_cb_72 => "cb 72.json",
test_instruction_cb_73 => "cb 73.json",
test_instruction_cb_74 => "cb 74.json",
test_instruction_cb_75 => "cb 75.json",
test_instruction_cb_76 => "cb 76.json",
test_instruction_cb_77 => "cb 77.json",
test_instruction_cb_78 => "cb 78.json",
test_instruction_cb_79 => "cb 79.json",
test_instruction_cb_7a => "cb 7a.json",
test_instruction_cb_7b => "cb 7b.json",
test_instruction_cb_7c => "cb 7c.json",
test_instruction_cb_7d => "cb 7d.json",
test_instruction_cb_7e => "cb 7e.json",
test_instruction_cb_7f => "cb 7f.json",
test_instruction_cb_80 => "cb 80.json",
test_instruction_cb_81 => "cb 81.json",
test_instruction_cb_82 => "cb 82.json",
test_instruction_cb_83 => "cb 83.json",
test_instruction_cb_84 => "cb 84.json",
test_instruction_cb_85 => "cb 85.json",
test_instruction_cb_86 => "cb 86.json",
test_instruction_cb_87 => "cb 87.json",
test_instruction_cb_88 => "cb 88.json",
test_instruction_cb_89 => "cb 89.json",
test_instruction_cb_8a => "cb 8a.json",
test_instruction_cb_8b => "cb 8b.json",
test_instruction_cb_8c => "cb 8c.json",
test_instruction_cb_8d => "cb 8d.json",
test_instruction_cb_8e => "cb 8e.json",
test_instruction_cb_8f => "cb 8f.json",
test_instruction_cb_90 => "cb 90.json",
test_instruction_cb_91 => "cb 91.json",
test_instruction_cb_92 => "cb 92.json",
test_instruction_cb_93 => "cb 93.json",
test_instruction_cb_94 => "cb 94.json",
test_instruction_cb_95 => "cb 95.json",
test_instruction_cb_96 => "cb 96.json",
test_instruction_cb_97 => "cb 97.json",
test_instruction_cb_98 => "cb 98.json",
test_instruction_cb_99 => "cb 99.json",
test_instruction_cb_9a => "cb 9a.json",
test_instruction_cb_9b => "cb 9b.json",
test_instruction_cb_9c => "cb 9c.json",
test_instruction_cb_9d => "cb 9d.json",
test_instruction_cb_9e => "cb 9e.json",
test_instruction_cb_9f => "cb 9f.json",
test_instruction_cb_a0 => "cb a0.json",
test_instruction_cb_a1 => "cb a1.json",
test_instruction_cb_a2 => "cb a2.json",
test_instruction_cb_a3 => "cb a3.json",
test_instruction_cb_a4 => "cb a4.json",
test_instruction_cb_a5 => "cb a5.json",
test_instruction_cb_a6 => "cb a6.json",
test_instruction_cb_a7 => "cb a7.json",
test_instruction_cb_a8 => "cb a8.json",
test_instruction_cb_a9 => "cb a9.json",
test_instruction_cb_aa => "cb aa.json",
test_instruction_cb_ab => "cb ab.json",
test_instruction_cb_ac => "cb ac.json",
test_instruction_cb_ad => "cb ad.json",
test_instruction_cb_ae => "cb ae.json",
test_instruction_cb_af => "cb af.json",
test_instruction_cb_b0 => "cb b0.json",
test_instruction_cb_b1 => "cb b1.json",
test_instruction_cb_b2 => "cb b2.json",
test_instruction_cb_b3 => "cb b3.json",
test_instruction_cb_b4 => "cb b4.json",
test_instruction_cb_b5 => "cb b5.json",
test_instruction_cb_b6 => "cb b6.json",
test_instruction_cb_b7 => "cb b7.json",
test_instruction_cb_b8 => "cb b8.json",
test_instruction_cb_b9 => "cb b9.json",
test_instruction_cb_ba => "cb ba.json",
test_instruction_cb_bb => "cb bb.json",
test_instruction_cb_bc => "cb bc.json",
test_instruction_cb_bd => "cb bd.json",
test_instruction_cb_be => "cb be.json",
test_instruction_cb_bf => "cb bf.json",
test_instruction_cb_c0 => "cb c0.json",
test_instruction_cb_c1 => "cb c1.json",
test_instruction_cb_c2 => "cb c2.json",
test_instruction_cb_c3 => "cb c3.json",
test_instruction_cb_c4 => "cb c4.json",
test_instruction_cb_c5 => "cb c5.json",
test_instruction_cb_c6 => "cb c6.json",
test_instruction_cb_c7 => "cb c7.json",
test_instruction_cb_c8 => "cb c8.json",
test_instruction_cb_c9 => "cb c9.json",
test_instruction_cb_ca => "cb ca.json",
test_instruction_cb_cb => "cb cb.json",
test_instruction_cb_cc => "cb cc.json",
test_instruction_cb_cd => "cb cd.json",
test_instruction_cb_ce => "cb ce.json",
test_instruction_cb_cf => "cb cf.json",
test_instruction_cb_d0 => "cb d0.json",
test_instruction_cb_d1 => "cb d1.json",
test_instruction_cb_d2 => "cb d2.json",
test_instruction_cb_d3 => "cb d3.json",
test_instruction_cb_d4 => "cb d4.json",
test_instruction_cb_d5 => "cb d5.json",
test_instruction_cb_d6 => "cb d6.json",
test_instruction_cb_d7 => "cb d7.json",
test_instruction_cb_d8 => "cb d8.json",
test_instruction_cb_d9 => "cb d9.json",
test_instruction_cb_da => "cb da.json",
test_instruction_cb_db => "cb db.json",
test_instruction_cb_dc => "cb dc.json",
test_instruction_cb_dd => "cb dd.json",
test_instruction_cb_de => "cb de.json",
test_instruction_cb_df => "cb df.json",
test_instruction_cb_e0 => "cb e0.json",
test_instruction_cb_e1 => "cb e1.json",
test_instruction_cb_e2 => "cb e2.json",
test_instruction_cb_e3 => "cb e3.json",
test_instruction_cb_e4 => "cb e4.json",
test_instruction_cb_e5 => "cb e5.json",
test_instruction_cb_e6 => "cb e6.json",
test_instruction_cb_e7 => "cb e7.json",
test_instruction_cb_e8 => "cb e8.json",
test_instruction_cb_e9 => "cb e9.json",
test_instruction_cb_ea => "cb ea.json",
test_instruction_cb_eb => "cb eb.json",
test_instruction_cb_ec => "cb ec.json",
test_instruction_cb_ed => "cb ed.json",
test_instruction_cb_ee => "cb ee.json",
test_instruction_cb_ef => "cb ef.json",
test_instruction_cb_f0 => "cb f0.json",
test_instruction_cb_f1 => "cb f1.json",
test_instruction_cb_f2 => "cb f2.json",
test_instruction_cb_f3 => "cb f3.json",
test_instruction_cb_f4 => "cb f4.json",
test_instruction_cb_f5 => "cb f5.json",
test_instruction_cb_f6 => "cb f6.json",
test_instruction_cb_f7 => "cb f7.json",
test_instruction_cb_f8 => "cb f8.json",
test_instruction_cb_f9 => "cb f9.json",
test_instruction_cb_fa => "cb fa.json",
test_instruction_cb_fb => "cb fb.json",
test_instruction_cb_fc => "cb fc.json",
test_instruction_cb_fd => "cb fd.json",
test_instruction_cb_fe => "cb fe.json",
test_instruction_cb_ff => "cb ff.json",
test_instruction_cc => "cc.json",
test_instruction_cd => "cd.json",
test_instruction_ce => "ce.json",
test_instruction_cf => "cf.json",
test_instruction_d0 => "d0.json",
test_instruction_d1 => "d1.json",
test_instruction_d2 => "d2.json",
test_instruction_d4 => "d4.json",
test_instruction_d5 => "d5.json",
test_instruction_d6 => "d6.json",
test_instruction_d7 => "d7.json",
test_instruction_d8 => "d8.json",
test_instruction_d9 => "d9.json",
test_instruction_da => "da.json",
test_instruction_dc => "dc.json",
test_instruction_de => "de.json",
test_instruction_df => "df.json",
test_instruction_e0 => "e0.json",
test_instruction_e1 => "e1.json",
test_instruction_e2 => "e2.json",
test_instruction_e5 => "e5.json",
test_instruction_e6 => "e6.json",
test_instruction_e7 => "e7.json",
test_instruction_e8 => "e8.json",
test_instruction_e9 => "e9.json",
test_instruction_ea => "ea.json",
test_instruction_ee => "ee.json",
test_instruction_ef => "ef.json",
test_instruction_f0 => "f0.json",
test_instruction_f1 => "f1.json",
test_instruction_f2 => "f2.json",
test_instruction_f3 => "f3.json",
test_instruction_f5 => "f5.json",
test_instruction_f6 => "f6.json",
test_instruction_f7 => "f7.json",
test_instruction_f8 => "f8.json",
test_instruction_f9 => "f9.json",
test_instruction_fa => "fa.json",
test_instruction_fb => "fb.json",
test_instruction_fe => "fe.json",
test_instruction_ff => "ff.json"
);

// #[test]
// fn singlestep_test() {
//     let mut cpu: Cpu = Cpu::new();
//     let mut memory: Memory = Memory::new();
//
//     let cases: Vec<CpuTestCase> = load_test_cases().unwrap();
//
//     println!("{}", cases.len());
//     for case in cases {
//         let test_name = case.name;
//         let initial_state = case.initial_state;
//         let final_state = case.final_state;
//
//         cpu.a = initial_state.a;
//         cpu.f = initial_state.f;
//         cpu.b = initial_state.b;
//         cpu.c = initial_state.c;
//         cpu.d = initial_state.d;
//         cpu.e = initial_state.e;
//         cpu.h = initial_state.h;
//         cpu.l = initial_state.l;
//         cpu.ei = initial_state.ei;
//         cpu.ime = initial_state.ime != 0;
//
//         for (address, value) in initial_state.ram {
//             *memory.get_mut_byte(address) = value;
//         }
//
//         cpu.run(&mut memory);
//
//         assert_eq!(cpu.a, final_state.a, "testing {} on register a", test_name);
//         assert_eq!(cpu.f, final_state.f, "testing {} on register f", test_name);
//         assert_eq!(cpu.b, final_state.b, "testing {} on register b", test_name);
//         assert_eq!(cpu.c, final_state.c, "testing {} on register c", test_name);
//         assert_eq!(cpu.d, final_state.d, "testing {} on register d", test_name);
//         assert_eq!(cpu.e, final_state.e, "testing {} on register e", test_name);
//         assert_eq!(cpu.h, final_state.h, "testing {} on register h", test_name);
//         assert_eq!(cpu.l, final_state.l, "testing {} on register l", test_name);
//         assert_eq!(cpu.ei, final_state.ei, "testing {} on EI", test_name);
//         assert_eq!(
//             cpu.ime,
//             final_state.ime != 0,
//             "testing {} on IME",
//             test_name
//         );
//
//         for (address, value) in final_state.ram {
//             assert_eq!(
//                 memory.get_byte(address),
//                 value,
//                 "testing {} on memory address {:0X} == {:0X}",
//                 test_name,
//                 address,
//                 value
//             );
//         }
//     }
// }
