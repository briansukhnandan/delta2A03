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
    // cpu.load_rom_data("/Users/brian/Documents/Git_Repositories/delta6502/test_rom/6502_functional_test.bin".to_string());
  }
}
