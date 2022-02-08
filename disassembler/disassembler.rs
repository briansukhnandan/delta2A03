#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

fn read_opcode(opcode: u8) {
    match opcode {
        // 0x00 => println!("BRK impl"),
        // 0x01 => println!("ORA (indr,X)"),
        // 0x05 => println!("ORA zpg"),
        // 0x06 => println!("ASL zpg"),
        // 0x08 => println!("PHP impl"),
        // 0x09 => println!("ORA immediate"),
        // 0x0A => println!("ASL Accum"),
        // 0x0D => println!("ORA abs"),
        // 0x0E => println!("ASL abs"),

        // 0x10 => println!("BPL rel"),
        // 0x11 => println!("ORA (indr,Y)"),
        // 0x15 => println!("ORA (zpg,X)"),
        // 0x16 => println!("ASL (zpg,X)"),
        // 0x18 => println!("CLC impl"),
        0x19 => println!("ORA (abs,Y)"),
        0x1D => println!("ORA (abs,X)"),
        0x1E => println!("ASL (abs,X)"),

        // 0x20 => println!("JSR abs"),
        // 0x21 => println!("AND (indr,X)"),
        // 0x24 => println!("BIT zpg"),
        // 0x25 => println!("AND zpg"),
        // 0x26 => println!("ROL zpg"),
        // 0x28 => println!("PLP impl"),
        // 0x29 => println!("AND immediate"),
        // 0x2A => println!("ROL Accum"),
        // 0x2C => println!("BIT abs"),
        // 0x2D => println!("AND abs"),
        // 0x2E => println!("ROL abs"),

        // 0x30 => println!("BMI rel"),
        // 0x31 => println!("AND (indr,Y)"),
        // 0x35 => println!("AND (zpg,X)"),
        // 0x36 => println!("ROL (zpg,X)"),
        // 0x38 => println!("SEC impl"),
        0x39 => println!("AND (abs,Y)"),
        0x3D => println!("AND (abs,X)"),
        0x3E => println!("ROL (abs,X)"),

        // 0x40 => println!("RTI impl"),
        // 0x41 => println!("EOR (indr,X)"),
        // 0x45 => println!("EOR zpg"),
        // 0x46 => println!("LSR zpg"),
        // 0x48 => println!("PHA impl"),
        // 0x49 => println!("EOR immediate"),
        // 0x4A => println!("LSR Accum"),
        // 0x4C => println!("JMP abs"),
        // 0x4D => println!("EOR abs"),
        // 0x4E => println!("LSR abs"),

        // 0x50 => println!("BVC rel"),
        // 0x51 => println!("EOR (indr,Y)"),
        // 0x55 => println!("EOR (zpg,X)"),
        // 0x56 => println!("LSR (zpg,X)"),
        // 0x58 => println!("CLI impl"),
        0x59 => println!("EOR (abs,Y)"),
        0x5D => println!("EOR (abs,X)"),
        0x5E => println!("LSR (abs,X)"),

        // 0x60 => println!("RTS impl"),
        // 0x61 => println!("ADC (indr,X)"),
        // 0x65 => println!("ADC zpg"),
        // 0x66 => println!("ROR zpg"),
        // 0x68 => println!("PLA impl"),
        // 0x69 => println!("ADC immediate"),
        // 0x6A => println!("ROR Accum"),
        // 0x6C => println!("JMP indr"),
        // 0x6D => println!("ADC abs"),
        // 0x6E => println!("ROR abs"),

        // 0x70 => println!("BVS rel"),
        // 0x71 => println!("ADC (indr,Y)"),
        // 0x75 => println!("ADC (zpg,X)"),
        // 0x76 => println!("ROR (zpg,X)"),
        // 0x78 => println!("SEI impl"),
        0x79 => println!("ADC (abs,Y)"),
        0x7D => println!("ADC (abs,X)"),
        0x7E => println!("ROR (abs,X)"),

        // 0x81 => println!("STA (indr,X)"),
        // 0x84 => println!("STY zpg"),
        // 0x85 => println!("STA zpg"),
        // 0x86 => println!("STX zpg"),
        // 0x88 => println!("DEY impl"),
        // 0x8A => println!("TXA impl"),
        // 0x8C => println!("STY abs"),
        // 0x8D => println!("STA abs"),
        // 0x8E => println!("STX abs"),

        // 0x90 => println!("BCC rel"),
        // 0x91 => println!("STA (indr,Y)"),
        // 0x94 => println!("STY (zpg,X)"),
        // 0x95 => println!("STA (zpg,X)"),
        // 0x96 => println!("STX (zpg,Y)"),
        // 0x98 => println!("TYA impl"),
        0x99 => println!("STA (abs,Y)"),
        // 0x9A => println!("TXS impl"),
        0x9D => println!("STA (abs,X)"),

        // 0xA0 => println!("LDY immediate"),
        // 0xA1 => println!("LDA (indr,X)"),
        // 0xA2 => println!("LDX immediate"),
        // 0xA4 => println!("LDY zpg"),
        // 0xA5 => println!("LDA zpg"),
        // 0xA6 => println!("LDX zpg"),
        // 0xA8 => println!("TAY impl"),
        // 0xA9 => println!("LDA immediate"),
        // 0xAA => println!("TAX impl"),
        // 0xAC => println!("LDY abs"),
        // 0xAD => println!("LDA abs"),
        // 0xAE => println!("LDX abs"),

        // 0xB0 => println!("BCS rel"),
        // 0xB1 => println!("LDA (indr,Y)"),
        // 0xB4 => println!("LDY (zpg,X)"),
        // 0xB5 => println!("LDA (zpg,X)"),
        // 0xB6 => println!("LDX (zpg,Y)"),
        // 0xB8 => println!("CLV impl"),
        0xB9 => println!("LDA (abs,Y)"),
        // 0xBA => println!("TSX impl"),
        0xBC => println!("LDY (abs,X)"),
        0xBD => println!("LDA (abs,X)"),
        0xBE => println!("LDX (abs,Y)"),

        // 0xC0 => println!("CPY immediate"),
        // 0xC1 => println!("CMP (indr,X)"),
        // 0xC4 => println!("CPY zpg"),
        // 0xC5 => println!("CMP zpg"),
        // 0xC6 => println!("DEC zpg"),
        // 0xC8 => println!("INY impl"),
        // 0xC9 => println!("CMP immediate"),
        // 0xCA => println!("DEX impl"),
        // 0xCC => println!("CPY abs"),
        // 0xCD => println!("CMP abs"),
        // 0xCE => println!("DEC abs"),

        // 0xD0 => println!("BNE rel"),
        // 0xD1 => println!("CMP (indr,Y)"),
        // 0xD5 => println!("CMP (zpg,X)"),
        // 0xD6 => println!("DEC (zpg,X)"),
        // 0xD8 => println!("CLD impl"),
        0xD9 => println!("CMP (abs,Y)"),
        0xDD => println!("CMP (abs,X)"),
        0xDE => println!("DEC (abs,X)"),

        // 0xE0 => println!("CPX immediate"),
        // 0xE1 => println!("SBC (indr,X)"),
        // 0xE4 => println!("CPX zpg"),
        // 0xE5 => println!("SBC zpg"),
        // 0xE6 => println!("INC zpg"),
        // 0xE8 => println!("INX impl"),
        // 0xE9 => println!("SBC immediate"),
        // 0xEA => println!("NOP impl"),
        // 0xEC => println!("CPX abs"),
        // 0xED => println!("SBC abs"),
        // 0xEE => println!("INC abs"),

        // 0xF0 => println!("BEQ rel"),
        0xF1 => println!("SBC (indr,Y)"),
        // 0xF5 => println!("SBC (zpg,X)"),
        // 0xF6 => println!("INC (zpg,X)"),
        // 0xF8 => println!("SED impl"),
        0xF9 => println!("SBC (abs,Y)"),
        0xFD => println!("SBC (abs,X)"),
        0xFE => println!("INC (abs,X)"),

        _ => println!("INVALID/UNIMPLEMENTED OPCODE"),
    }
}

use std::io;
// use std::io::Read;
// use std::io::BufReader;
// use std::fs::File;
use std::io::Write;

fn disassemble(_path: String) -> io::Result<()> {

    // UNCOMMENT BELOW 4 LINES TO READ ROM DATA.
    // let f = File::open(path)?;
    // let mut reader = BufReader::new(f);
    // let mut buffer = Vec::new();
    // reader.read_to_end(&mut buffer)?;

    // Testing with specific instructions.
    let mut buffer = Vec::new();
    // buffer.push(0x09);
    // buffer.push(0x1A);
    // buffer.push(0x4E);
    buffer.push(0x4C);
    buffer.push(0x32);
    buffer.push(0x40);

    let mut opcode_idx = 0;
    while opcode_idx < buffer.len() {

        print!("{:#02x}: ", buffer[opcode_idx]);
        io::stdout().flush().unwrap();

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
                println!("JMP {:#04x} indr", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
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
                println!("ORA {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x0E => {
                println!("ASL {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x20 => {
                println!("JSR {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x2C => {
                println!("BIT {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x2D => {
                println!("AND {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x2E => {
                println!("ROL {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;      
            },

            0x4C => {
                println!("JMP {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x4D => {
                println!("EOR {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x4E => {
                println!("LSR {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x6D => {
                println!("ADC {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x6E => {
                println!("ROR {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x8C => {
                println!("STY {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x8D => {
                println!("STA {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0x8E => {
                println!("STX {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xAC => {
                println!("LDY {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xAD => {
                println!("LDA {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xAE => {
                println!("LDX {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xCC => {
                println!("CPY {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xCD => {
                println!("CMP {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xCE => {
                println!("DEC {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xEC => {
                println!("CPX {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xED => {
                println!("SBC {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
                opcode_idx = opcode_idx+3;
                continue;
            },

            0xEE => {
                println!("INC {:#04x} abs", (buffer[opcode_idx+2] << 8) | buffer[opcode_idx+1]);
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