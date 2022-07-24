#[allow(dead_code)]
#[allow(non_snake_case)]

pub struct Memory {
  // Memory map ranges from 0x0 -> 0xFFFF.
  // Since we must have 0xFFFF + 1 total bytes allocated,
  // use u32.
  pub TOTAL_MEMORY_SIZE: u32,

  // RAM ranges between 0x0 <= RAM < 0x0800 (non-inclusive meaning 0->0x07FF).
  pub RAM_START_ADDRESS: u16,
  pub RAM_END_ADDRESS: u16,

  // Stack ptr is between 0x0100 <= SP < 0x0200 (non-inclusive meaning 0->0x01FF).
  pub STACK_POINTER_START_BOUND: u16,
  pub STACK_POINTER_END_BOUND: u16,

  // Each page is 256 bytes, and there are also
  // 256 pages in total: (0xFF + 0x1) * (0xFF + 0x1) = (0xFFFF + 1) 
  pub PAGE_SIZE: u16,

  // Our allocated memory.
  pub DATA: Vec<u8>,
}

// Default values for our Memory.
impl Default for Memory {
  fn default() -> Memory {
    Memory {
      TOTAL_MEMORY_SIZE: 0xFFFF + 1, // 65536
      RAM_START_ADDRESS: 0x0,
      RAM_END_ADDRESS: 0x0800, // 2048
      STACK_POINTER_START_BOUND: 0x0100,
      STACK_POINTER_END_BOUND: 0x0200,
      PAGE_SIZE: 0xFF + 1, // 256
      DATA: Vec::new()
    }
  }
}

// Initialization/helper functions to interact with our memory.
impl Memory {
  // Initializes 256 pages each 256 bytes long with value 0.
  pub fn initialize_memory(&mut self) {        
    // self.DATA = vec![0, (0xFFFF)+1] // Throws error for some reason.
    for _i in 0..self.TOTAL_MEMORY_SIZE {
      self.DATA.push(0x0);
    }
    println!("Memory initialized!");
  }

  pub fn read_from_memory_address(&self, address: usize) -> u8 {
    return self.DATA[address];
  }

  pub fn write_to_memory_address(&mut self, address: usize, value: u8) {
    // If we are in the range 0x0 -> 0x07FF we will mirror
    // 3 times until we reach 0x2000. For each bit of memory set, we
    // will mirror this at the bit 0x800 over.
    if address < self.RAM_END_ADDRESS.into() {
      let mut mirrored_bit = address + 0x0800;

      while mirrored_bit < 0x2000 {
        self.DATA[mirrored_bit] = value;
        mirrored_bit += 0x0800;
      }
    }

    self.DATA[address] = value;
  }

  // Zero's out memory addr 0x0 -> 0x7FF.
  pub fn clear_ram(&mut self) {
    for i in self.RAM_START_ADDRESS..self.RAM_END_ADDRESS {
      self.write_to_memory_address((i as usize).try_into().unwrap(), 0x0);
    }
  }
}