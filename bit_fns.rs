#[allow(non_snake_case)]

fn toggle_N_flag(p: u8) -> u8 { p ^ 0b1000_0000 } // Bit 7
fn toggle_V_flag(p: u8) -> u8 { p ^ 0b0100_0000 } // Bit 6
fn toggle_B_flag(p: u8) -> u8 { p ^ 0b0001_0000 } // Bit 4
fn toggle_D_flag(p: u8) -> u8 { p ^ 0b0000_1000 } // Bit 3
fn toggle_I_flag(p: u8) -> u8 { p ^ 0b0000_0100 } // Bit 2
fn toggle_Z_flag(p: u8) -> u8 { p ^ 0b0000_0010 } // Bit 1
fn toggle_C_flag(p: u8) -> u8 { p ^ 0b0000_0001 } // Bit 0

fn main() {
    let mut p = 0b0000_0000;
    p = toggle_N_flag(p);

    println!("{:#08b}", p);
}