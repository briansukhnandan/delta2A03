#[allow(dead_code)]
#[allow(non_snake_case)]

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;

// Other modules
mod memory; // RAM model
mod mappings; // Mappings for our ROM, PPU, etc.

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
  pub opcode: u8,

  // Current ROM specified
  pub rom_path: String,

  // ROM format.
  pub rom_format: mappings::Rom_Format
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
      rom_path: String::from(""),
      rom_format: mappings::Rom_Format::INES2_0
    }
  }
}

// Functions for CPU.
impl CPU {
  // Initialize memory and stack ptr.
  pub fn initialize_cpu(&mut self) {
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

  // Very nice page describing header modes and general ROM-parsing: 
  // https://bheisler.github.io/post/nes-rom-parser-with-nom/
  pub fn load_rom_data(&mut self, _path: &String) -> io::Result<()> {
    let f = File::open(&_path);

    match File::open(_path) {
      Ok(f) => {
        self.rom_path = String::from(_path);
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer);

        // Parse ROM headers.
        // The first 4 bytes of headers are the ASCII representation of 'N', 'E', 'S', followed by 0x1A.
        // If this condition fails the ROM headers are invalid.
        let rom_headers = &buffer[0..16];
        assert!(rom_headers[0..4] == [0x4e, 0x45, 0x53, 0x1a], "ROM headers are invalid");

        println!("{:?}", rom_headers);
        println!("{}", self.rom_path);

        let prg_rom = rom_headers[4];
        let chr_rom = rom_headers[5];
        let byte6 = rom_headers[6];
        let byte7 = rom_headers[7];

        // NES 2.0 mode is set if the 2nd bit is 0, and 3rd bit is set.
        if byte7 & 0b0000_1100 == 0b0000_1000 {
          self.configure_iNES2_0_mapping();
        }
        else {
          self.configure_iNES_mapping();
        }

        Ok(())
      }
      Err(err) => Err(err),
    }

  }

  //////////////////////////
  //  ROM Memory Mapping  //
  //////////////////////////
  /* ROMs have two common formats: iNES and NES 2.0 */
  pub fn configure_iNES2_0_mapping(&mut self) {
    self.rom_format = mappings::Rom_Format::INES2_0;
    println!("iNES2.0 {}", self.rom_format);
  }

  pub fn configure_iNES_mapping(&mut self) {
    self.rom_format = mappings::Rom_Format::INES;
    println!("iNES {}", self.rom_format);
  }

  ////////////////////////////////////////
  //  Register Getter/Setter Functions  //
  ////////////////////////////////////////
  /* Note - Status register updates are handled in the next section. */
  pub fn get_a_register(&self) -> u8 { self.a }
  pub fn get_x_register(&self) -> u8 { self.x }
  pub fn get_y_register(&self) -> u8 { self.y }

  pub fn set_a_register(&mut self, value: u8) { self.a = value; }
  pub fn set_x_register(&mut self, value: u8) { self.x = value; }
  pub fn set_y_register(&mut self, value: u8) { self.y = value; }

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
  pub fn read_c_flag(&self) -> u8 { self.p & 0b0000_0001 }

  ///////////////////////////////////
  //  Opcode processing Functions  //
  ///////////////////////////////////

  // TODO
  pub fn process_opcode(&mut self) {
    println!("TODO");
  }

}