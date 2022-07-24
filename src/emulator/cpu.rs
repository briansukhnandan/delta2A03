#[allow(dead_code)]
#[allow(non_snake_case)]

// Other modules
mod memory;

// Our main struct to hold fields such as registers and whatnot.
pub struct CPU {
  // 3 8-bit general purpose registers A, X, and Y.
  pub a: u8,
  pub x: u8,
  pub y: u8,

  // Our flags register. Refer to notes for all individual bits.
  pub p: u8,

  // Memory implementation 0->65535
  pub memory: memory::Memory,

  // Stack ptr which is hardcoded between 0x100 -> 0x1ff.
  pub sp: u8,

  // 16 bit program counter.
  pub pc: u16,

  // num of cycles the cpu has done, as some instr do > 1 cycles.
  pub cycle_count: u16,

  // Current opcode
  pub opcode: u8
}

// Default values for our 6502 if we are initializing anywhere else.
impl Default for CPU {
  fn default() -> CPU {
    CPU {
      a: 0, // Accumulator.
      x: 0,
      y: 0,

      p: 0b0010_0100, // Explicitly declare flags as binary for readability.
      
      memory: memory::Memory { ..Default::default() },
      sp: 0x0u8, // sp serves as an offset from 0x0100.
      pc: 0x0u16,

      cycle_count: 0,
      opcode: 0x0u8,
    }
  }
}

// Functions for CPU.
impl CPU {
  // Initialize memory and stack ptr.
  pub fn initialize_CPU(&mut self) {
    self.memory.initialize_memory();

    println!("CPU initialized!\n")
  }

  pub fn dump_registers(&self) {
    println!("Accumulator: {}", self.a);
    println!("X register: {}", self.x);
    println!("Y register: {}", self.y);
    println!("SP: {}", self.sp);
    println!("Status register: {:#08b}", self.p);
  }

  pub fn load_data(&mut self) {

  }

  pub fn process_opcode(&mut self) {
    println!("TODO");
  }

  ////////////////////////////////////////
  //  Status Register Helper Functions  //
  ////////////////////////////////////////
  pub fn toggle_n_flag(&mut self) { self.p = self.p ^ 0b1000_0000 } // Bit 7
  pub fn toggle_v_flag(&mut self) { self.p = self.p ^ 0b0100_0000 } // Bit 6
  pub fn toggle_b_flag(&mut self) { self.p = self.p ^ 0b0001_0000 } // Bit 4
  pub fn toggle_d_flag(&mut self) { self.p = self.p ^ 0b0000_1000 } // Bit 3
  pub fn toggle_i_flag(&mut self) { self.p = self.p ^ 0b0000_0100 } // Bit 2
  pub fn toggle_z_flag(&mut self) { self.p = self.p ^ 0b0000_0010 } // Bit 1
  pub fn toggle_c_flag(&mut self) { self.p = self.p ^ 0b0000_0001 } // Bit 0

  pub fn read_n_flag(&self) -> u8 { (self.p & 0b1000_0000) >> 7 }
  pub fn read_v_flag(&self) -> u8 { (self.p & 0b0100_0000) >> 6 }
  pub fn read_b_flag(&self) -> u8 { (self.p & 0b0001_0000) >> 4 }
  pub fn read_d_flag(&self) -> u8 { (self.p & 0b0000_1000) >> 3 }
  pub fn read_i_flag(&self) -> u8 { (self.p & 0b0000_0100) >> 2 }
  pub fn read_z_flag(&self) -> u8 { (self.p & 0b0000_0010) >> 1 }
  pub fn read_c_flag(&self) -> u8 { (self.p & 0b0000_0001) }
}

// fn main() {
//   let mut cpu = CPU { ..Default::default() };
//   cpu.initialize_CPU();

//   cpu.test();
//   cpu.dump_registers();
// }