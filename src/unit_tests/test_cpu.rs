pub fn test_module() {

  let mut cpu = crate::emulator::cpu::CPU { ..Default::default() };
  cpu.initialize_CPU();

  println!("\nRegister dump before test functions are called:");
  cpu.dump_registers();

  println!("\nIncrementing X register");
  cpu.x = cpu.x + 1;

  println!("\nStatus Register before any toggling: {:#08b}", cpu.p);
  cpu.toggle_n_flag();

  println!("\nRegister dump after test functions are called:");
  cpu.dump_registers();
}