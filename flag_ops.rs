#![allow(non_snake_case)]

pub fn toggle_N_flag(p: u8) -> u8 { p ^ 0b1000_0000 } // Bit 7
pub fn toggle_V_flag(p: u8) -> u8 { p ^ 0b0100_0000 } // Bit 6
pub fn toggle_B_flag(p: u8) -> u8 { p ^ 0b0001_0000 } // Bit 4
pub fn toggle_D_flag(p: u8) -> u8 { p ^ 0b0000_1000 } // Bit 3
pub fn toggle_I_flag(p: u8) -> u8 { p ^ 0b0000_0100 } // Bit 2
pub fn toggle_Z_flag(p: u8) -> u8 { p ^ 0b0000_0010 } // Bit 1
pub fn toggle_C_flag(p: u8) -> u8 { p ^ 0b0000_0001 } // Bit 0

pub fn read_N_flag(p: u8) -> u8 { (p & 0b1000_0000) >> 7 }
pub fn read_V_flag(p: u8) -> u8 { (p & 0b0100_0000) >> 6 }
pub fn read_B_flag(p: u8) -> u8 { (p & 0b0001_0000) >> 4 }
pub fn read_D_flag(p: u8) -> u8 { (p & 0b0000_1000) >> 3 }
pub fn read_I_flag(p: u8) -> u8 { (p & 0b0000_0100) >> 2 }
pub fn read_Z_flag(p: u8) -> u8 { (p & 0b0000_0010) >> 1 }
pub fn read_C_flag(p: u8) -> u8 { (p & 0b0000_0001) }


fn main() {
  // Bit 5 is always set to 1
  let mut p = 0b0010_0000;
  p = toggle_N_flag(p);
  p = toggle_V_flag(p);
  p = toggle_B_flag(p);
  p = toggle_D_flag(p);
  p = toggle_I_flag(p);
  p = toggle_Z_flag(p);
  p = toggle_C_flag(p);

  println!("{:#08b}", p);
  println!("{}", read_N_flag(p));
  p = toggle_N_flag(p);
  println!("{}", read_N_flag(p));
  p = toggle_N_flag(p);
  println!("{}", read_N_flag(p));
}