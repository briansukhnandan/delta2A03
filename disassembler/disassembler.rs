#[allow(unused_variables)]
#[allow(unused_imports)]

use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;

// A helpful utility function to display the datatype
// of a variable.
#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

// Pipe the output of this program to a file using your shell.
// i.e. 
// $ rustc disassembler.rs
// $ (./disassembler) > disassembler_output.txt
fn disassemble(_path: String) -> io::Result<()> {

  // UNCOMMENT BELOW 4 LINES TO READ ROM DATA.
  let f = File::open(_path)?;
  let mut reader = BufReader::new(f);
  let mut buffer = Vec::new();
  reader.read_to_end(&mut buffer)?;

  // Testing with specific instructions.
  // let mut buffer = Vec::new();
  // buffer.push(0x09);
  // buffer.push(0x1A);
  // buffer.push(0x4E);
  // buffer.push(0x4C);
  // buffer.push(0x32);
  // buffer.push(0x40);

  // buffer.push(0x4E);
  // buffer.push(0x45);
  // buffer.push(0x53);
  // buffer.push(0x1A);
  // buffer.push(0x02);
  // buffer.push(0x01);
  // buffer.push(0x0);
  // buffer.push(0x1A);
  // buffer.push(0x1A);
  // buffer.push(0x1A);
  // buffer.push(0x1A);
  // buffer.push(0x1A);
  // buffer.push(0x1A);
  // buffer.push(0x1A);


  let mut opcode_idx = 0;
  while opcode_idx < buffer.len() {

    print!("{:#02x}: ", buffer[opcode_idx]);
    io::stdout().flush().unwrap();

    // print_type_of(&buffer[opcode_idx]);

      
    match buffer[opcode_idx] {


      ////////////////////////////////
      // ACCUMULATOR ADDRESSING MODE
      ////////////////////////////////
      0x0A => println!("ASL A"),
      0x2A => println!("ROL A"),
      0x4A => println!("LSR A"),
      0x6A => println!("ROR A"),


      ///////////////////////////////
      // INDIRECT ADDRESSING MODES
      ///////////////////////////////
      0x6C => {
        // Keep in mind that for indirect, we take the next 2 bytes
        // and use it as an ABSOLUTE address to jump to.
        // Ex: JMP 0x9000 will set the PC to the memory stored
        // at that absolute address.
        // 6c 00 90 -> JMP 0x9000
        // If the memory at 0x9000 looked like this:
        // 0x9000: 52 3a 04 d3 ... 97
        // When we jump to cell 0x9000, the PC is set to 0x3a52.
        println!("JMP {:#04x} indr", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },
        

      ///////////////////////////////
      // INDEXED INDIRECT (indr,X)
      ///////////////////////////////
      0x01 => {
        println!("ORA {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x21 => {
        println!("AND {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x41 => {
        println!("EOR {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x61 => {
        println!("ADC {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x81 => {
        println!("STA {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA1 => {
        println!("LDA {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC1 => {
        println!("CMP {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE1 => {
        println!("SBC {:#02x} (indr,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },
        

      ///////////////////////////////
      // INDEXED INDIRECT (indr,Y)
      ///////////////////////////////
      0x11 => {
        println!("ORA {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x31 => {
        println!("AND {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x51 => {
        println!("EOR {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x71 => {
        println!("ADC {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x91 => {
        println!("STA {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xB1 => {
        println!("LDA {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xD1 => {
        println!("CMP {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xF1 => {
        println!("SBC {:#02x} (indr,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },


      ///////////////////////////////
      // IMMEDIATE ADDRESSING MODES
      ///////////////////////////////
      0x09 => {
        println!("ORA # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x29 => {
        println!("AND # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x49 => {
        println!("EOR # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x69 => {
        println!("ADC # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA0 => {
        println!("LDY # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA2 => {
        println!("LDX # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA9 => {
        println!("LDA # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC0 => {
        println!("CPY # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC9 => {
        println!("CMP # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE0 => {
        println!("CPX # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE9 => {
        println!("SBC # {:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },


      ///////////////////////////////
      // IMPLIED ADDRESSING MODES
      //////////////////////////////
      0x00 => println!("BRK"),
      0x08 => println!("PHP"),
      0x18 => println!("CLC"),
      0x28 => println!("PLP"),
      0x38 => println!("SEC"),
      0x40 => println!("RTI"),
      0x48 => println!("PHA"),
      0x58 => println!("CLI"),
      0x60 => println!("RTS"),
      0x68 => println!("PLA"),
      0x78 => println!("SEI"),
      0x88 => println!("DEY"),
      0x8A => println!("TXA"),
      0x98 => println!("TYA"),
      0x9A => println!("TXS"),
      0xA8 => println!("TAY"),
      0xAA => println!("TAX"),
      0xB8 => println!("CLV"),
      0xBA => println!("TSX"),
      0xC8 => println!("INY"),
      0xCA => println!("DEX"),
      0xD8 => println!("CLD"),
      0xE8 => println!("INX"),
      0xEA => println!("NOP"),
      0xF8 => println!("SED"),

        
      ////////////////////////////////
      // RELATIVE ADDRESSING MODES
      ////////////////////////////////
      0x10 => {
        println!("Branch BPL, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x30 => {
        println!("Branch BMI, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x50 => {
        println!("Branch BVC, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x70 => {
        println!("Branch BVS, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x90 => {
        println!("Branch BCC, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xB0 => {
        println!("Branch BCS, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xD0 => {
        println!("Branch BNE, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xF0 => {
        println!("Branch BEQ, IF TAKEN: PC=PC+{:#02x}", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },


      ///////////////////////////////
      // ABSOLUTE ADDRESSING MODES
      ///////////////////////////////
      0x0D => {
        // Remember that 6502 is a Little-Endian machine.
        println!("ORA {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x0E => {
        println!("ASL {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x20 => {
        println!("JSR {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x2C => {
        println!("BIT {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x2D => {
        println!("AND {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x2E => {
        println!("ROL {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;      
      },

      0x4C => {
        println!("JMP {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x4D => {
        println!("EOR {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x4E => {
        println!("LSR {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x6D => {
        println!("ADC {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x6E => {
        println!("ROR {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x8C => {
        println!("STY {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x8D => {
        println!("STA {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x8E => {
        println!("STX {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xAC => {
        println!("LDY {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xAD => {
        println!("LDA {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xAE => {
        println!("LDX {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xCC => {
        println!("CPY {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xCD => {
        println!("CMP {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xCE => {
        println!("DEC {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xEC => {
        println!("CPX {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xED => {
        println!("SBC {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xEE => {
        println!("INC {:#04x} abs", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      ///////////////////////////////////////
      // ABSOLUTE INDEXED ADDRESSING MODES
      ///////////////////////////////////////
      0x1D => {
        println!("ORA {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x1E => {
        println!("ASL {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x3D => {
        println!("AND {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x3E => {
        println!("ROL {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x5D => {
        println!("EOR {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x5E => {
        println!("LSR {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x7D => {
        println!("ADC {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x7E => {
        println!("ROR {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x9D => {
        println!("STA {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xBC => {
        println!("LDY {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xBD => {
        println!("LDA {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xDD => {
        println!("CMP {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xDE => {
        println!("DEC {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xFD => {
        println!("SBC {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xFE => {
        println!("INC {:#04x} (abs,X)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x19 => {
        println!("ORA {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x39 => {
        println!("AND {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x59 => {
        println!("EOR {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x79 => {
        println!("ADC {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0x99 => {
        println!("STA {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xB9 => {
        println!("LDA {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xBE => {
        println!("LDX {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xD9 => {
        println!("CMP {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      0xF9 => {
        println!("SBC {:#04x} (abs,Y)", ((buffer[opcode_idx+2] as u16) << 8) | (buffer[opcode_idx+1] as u16));
        opcode_idx = opcode_idx+3;
        continue;
      },

      ////////////////////////////////
      // ZERO-PAGE ADDRESSING MODES
      ////////////////////////////////
      0x05 => {
        // Zero pg addressing mode is abs addressing but for only
        // first 256 bytes of memory. So we only take the next byte
        // from memory and increment op_idx by 2.
        println!("ORA {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x06 => {
        println!("ASL {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x24 => {
        println!("BIT {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x25 => {
        println!("AND {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x26 => {
        println!("ROL {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x45 => {
        println!("EOR {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x46 => {
        println!("LSR {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x65 => {
        println!("ADC {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x66 => {
        println!("ROR {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x84 => {
        println!("STY {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x85 => {
        println!("STA {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x86 => {
        println!("STX {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA4 => {
        println!("LDY {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA5 => {
        println!("LDA {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xA6 => {
        println!("LDX {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC4 => {
        println!("CPY {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC5 => {
        println!("CMP {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xC6 => {
        println!("DEC {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE4 => {
        println!("CPX {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE5 => {
        println!("SBC {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xE6 => {
        println!("INC {:#02x} zpg", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },


      ////////////////////////////////////////
      // ZERO-PAGE INDEXED ADDRESSING MODES
      ////////////////////////////////////////
      0x15 => {
        // This addressing mode works just like absolute indexed
        // but the target addr is limited to the first 0xFF bytes.
        // Ex: LDA 0xC0,X where X=0x60, then the target addr will
        // be 0x20 since C0+60=120 but since this is no carry we
        // drop the most sig hex so we get 0x20 as target address. 
        println!("ORA {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x16 => {
        println!("ASL {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      }

      0x35 => {
        println!("AND {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x36 => {
        println!("ROL {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x55 => {
        println!("EOR {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x56 => {
        println!("LSR {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x75 => {
        println!("ADC {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x76 => {
        println!("ROR {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x94 => {
        println!("STY {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x95 => {
        println!("STA {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0x96 => {
        println!("STX {:#02x} (zpg,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xB4 => {
        println!("LDY {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xB5 => {
        println!("LDA {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xB6 => {
        println!("LDX {:#02x} (zpg,Y)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xD5 => {
        println!("CMP {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xD6 => {
        println!("DEC {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xF5 => {
        println!("SBC {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },

      0xF6 => {
        println!("INC {:#02x} (zpg,X)", buffer[opcode_idx+1]);
        opcode_idx = opcode_idx+2;
        continue;
      },


      ///////////////////
      // DEFAULT CASE
      ///////////////////
      _ => println!("Invalid/Unimplemented Opcode"),

    }
      
    opcode_idx = opcode_idx + 1;
  }

  Ok(())
}

fn main() {
  //read_opcode(0x81);
  let _res = disassemble(String::from("/Users/brian/Documents/NES Test ROMs/instr_test_v5/01-basics.nes"));
}