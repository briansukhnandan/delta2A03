use rand::prelude::*;

pub fn test_component() {

  println!("-----------------Beginning CPU Test-------------------");

  let mut rng = thread_rng();
  let mut cpu = crate::emulator::cpu::CPU { ..Default::default() };
  
  cpu.initialize_cpu();
  let original_status_register_value = cpu.p;

  println!("Register dump before test functions are called:");
  cpu.dump_registers();

  for _i in 0..100 {
    let rand_8bit_value = rng.gen_range(0..255);
    cpu.set_a_register(rand_8bit_value);
    cpu.set_x_register(rand_8bit_value);
    cpu.set_y_register(rand_8bit_value);

    assert!(cpu.get_a_register() == rand_8bit_value, "Something went wrong setting/getting A register.");
    assert!(cpu.get_x_register() == rand_8bit_value, "Something went wrong setting/getting X register.");
    assert!(cpu.get_y_register() == rand_8bit_value, "Something went wrong setting/getting Y register.");

    cpu.toggle_n_flag();
    cpu.toggle_v_flag();
    cpu.toggle_b_flag();
    cpu.toggle_d_flag();
    cpu.toggle_i_flag();
    cpu.toggle_z_flag();
    cpu.toggle_c_flag();

    assert!(cpu.p == 0b1111_1011, "Something went wrong with toggling flags on Status Register!");

    cpu.toggle_n_flag();
    cpu.toggle_v_flag();
    cpu.toggle_b_flag();
    cpu.toggle_d_flag();
    cpu.toggle_i_flag();
    cpu.toggle_z_flag();
    cpu.toggle_c_flag();

    assert!(cpu.p == original_status_register_value, "Something went wrong with toggling flags on Status Register!");
  }

  println!("\nRegister dump after test functions are called:");
  cpu.dump_registers();

  println!("\nTest complete!");
  println!("-----------------End CPU Test-------------------\n");
}