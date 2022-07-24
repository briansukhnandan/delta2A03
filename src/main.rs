mod emulator;
mod unit_tests;

fn main() {

  let test_mode = true;

  if test_mode {
    unit_tests::test_cpu::test_module();
    // unit_tests::test_memory::test_module();
  }

  println!("Hello, world!");

  let mut cpu = emulator::cpu::CPU { ..Default::default() };
  cpu.initialize_CPU();

  // cpu.test();
  cpu.dump_registers();
}
