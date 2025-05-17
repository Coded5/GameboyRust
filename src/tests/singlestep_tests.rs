use std::{fs, io};

use serde::Deserialize;

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

    ram: Vec<Vec<u16>>,
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
    println!("{:?}", res);
    Ok(res.into_iter().flatten().collect())
}

#[test]
fn singlestep_test() {
    let mut cpu: Cpu = Cpu::new();
    let mut memory: Memory = Memory::new();

    let cases: Vec<CpuTestCase> = load_test_cases().unwrap();

    for case in cases {
        let initial_state = case.initial_state;
        let final_state = case.final_state;

        cpu.a = initial_state.a;
        cpu.f = initial_state.f;
        cpu.b = initial_state.b;
        cpu.c = initial_state.c;
        cpu.d = initial_state.d;
        cpu.e = initial_state.e;
        cpu.h = initial_state.h;
        cpu.l = initial_state.l;
    }
}
