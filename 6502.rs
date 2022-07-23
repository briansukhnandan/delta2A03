#[allow(dead_code)]
#[allow(non_snake_case)]

// Other modules
mod flag_ops;
mod memory;

// Our main struct to hold fields such as registers and whatnot.
struct CPU {
  // 3 8-bit general purpose registers A, X, and Y.
  a: u8,
  x: u8,
  y: u8,

  // Our flags register. Refer to notes for all individual bits.
  p: u8,

  // Memory implementation 0->65535
  memory: memory::Memory,

  // Stack ptr which is hardcoded between 0x100 -> 0x1ff.
  sp: u8,

  // 16 bit program counter.
  pc: u16,

  // num of cycles the cpu has done, as some instr do > 1 cycles.
  cycle_count: u16,

  // Current opcode
  opcode: u8
}

// Default values for our 6502 if we are initializing anywhere else.
impl Default for CPU {
  fn default() -> CPU {
    CPU {
      a: 0, // Also known as the accumulator.
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
  fn initialize_CPU(&mut self) {
    self.memory.initialize_memory();

    println!("CPU initialized!\n")
  }

  fn dump_registers(&self) {
    println!("Accumulator: {}", self.a);
    println!("X register: {}", self.x);
    println!("Y register: {}", self.y);
    println!("SP: {}", self.sp);
    println!("Status register: {:#08b}", self.p);
  }

  fn test(&mut self) {
    // println!("incrementing X register");
    self.x = self.x + 1;

    // println!("Flipping flags/testing module");
    self.p = flag_ops::toggle_N_flag(self.p);
  }

  fn load_data(&mut self) {

  }

  fn process_opcode(&mut self) {
      println!("TODO");
  }
}

fn main() {
  let mut cpu = CPU { ..Default::default() };
  cpu.initialize_CPU();

  cpu.test();
  cpu.dump_registers();
}