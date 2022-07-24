use std::env;

mod emulator;
mod unit_tests;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mode = &args[1];

  if mode == "tests" {
    unit_tests::test_cpu::test_component();
    unit_tests::test_memory::test_component();
  }

  else if mode == "emulator" {
    let mut cpu = emulator::cpu::CPU { ..Default::default() };
    cpu.initialize_cpu();
    cpu.load_rom_data(&"/Users/brian/Documents/NES Test ROMs/instr_test_v5/01-basics.nes".to_string());
    // println!("Testing if program terminates here");
  }
}
