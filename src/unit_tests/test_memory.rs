use rand::prelude::*;

pub fn test_component() {
  println!("-----------------Beginning Memory Test-------------------");

  let mut rng = thread_rng();

  let mut cpu = crate::emulator::cpu::CPU { ..Default::default() };
  cpu.initialize_cpu();

  // Be sure to test mirroring first.
  for _i in 0..100 {
    let rand_8bit_address = rng.gen_range(0..255);

    cpu.memory.write_to_memory_address(rand_8bit_address, 0xFF);

    for _j in (rand_8bit_address..0x2000).step_by(0x800) {
      assert!(cpu.memory.read_from_memory_address(_j) == cpu.memory.read_from_memory_address(rand_8bit_address));
    }

    cpu.memory.clear_ram();
    for _j in cpu.memory.RAM_START_ADDRESS..cpu.memory.RAM_END_ADDRESS {
      assert!(cpu.memory.read_from_memory_address(_j.into()) == 0);
    }
  }

  println!("\nTest complete!");
  println!("-----------------End Memory Test-------------------\n");
}