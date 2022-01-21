fn read_opcode(opcode: u8) {

    match opcode {

        0x00 => println!("BRK impl"),
        0x01 => println!("ORA (indr,X)"),
        0x05 => println!("ORA zpg"),
        0x06 => println!("ASL zpg"),
        0x08 => println!("PHP impl"),
        0x09 => println!("ORA immediate"),
        0x0A => println!("ASL Accum"),
        0x0D => println!("ORA abs"),
        0x0E => println!("ASL abs"),

        0x10 => println!("BPL rel"),
        0x11 => println!("ORA (indr,Y)"),
        0x15 => println!("ORA (zpg,X)"),
        0x16 => println!("ASL (zpg,X)"),
        0x18 => println!("CLC impl"),
        0x19 => println!("ORA (abs,Y)"),
        0x1D => println!("ORA (abs,X)"),
        0x1E => println!("ASL (abs,X)"),

        0x20 => println!("JSR abs"),
        0x21 => println!("AND (indr,X)"),
        0x24 => println!("BIT zpg"),
        0x25 => println!("AND zpg"),
        0x26 => println!("ROL zpg"),
        0x28 => println!("PLP impl"),
        0x29 => println!("AND immediate"),
        0x2A => println!("ROL Accum"),
        0x2C => println!("BIT abs"),
        0x2D => println!("AND abs"),
        0x2E => println!("ROL abs"),

        0x30 => println!("BMI rel"),
        0x31 => println!("AND (indr,Y)"),
        0x35 => println!("AND (zpg,X)"),
        0x36 => println!("ROL (zpg,X)"),
        0x38 => println!("SEC impl"),
        0x39 => println!("AND (abs,Y)"),
        0x3D => println!("AND (abs,X)"),
        0x3E => println!("ROL (abs,X)"),

        0x40 => println!("RTI impl"),
        0x41 => println!("EOR (indr,X)"),
        0x45 => println!("EOR zpg"),
        0x46 => println!("LSR zpg"),
        0x48 => println!("PHA impl"),
        0x49 => println!("EOR immediate"),
        0x4A => println!("LSR Accum"),
        0x4C => println!("JMP abs"),
        0x4D => println!("EOR abs"),
        0x4E => println!("LSR abs"),

        0x50 => println!("BVC rel"),
        0x51 => println!("EOR (indr,Y)"),
        0x55 => println!("EOR (zpg,X)"),
        0x56 => println!("LSR (zpg,X)"),
        0x58 => println!("CLI impl"),
        0x59 => println!("EOR (abs,Y)"),
        0x5D => println!("EOR (abs,X)"),
        0x5E => println!("LSR (abs,X)"),

        0x60 => println!("RTS impl"),
        0x61 => println!("ADC (indr,X)"),
        0x65 => println!("ADC zpg"),
        0x66 => println!("ROR zpg"),
        0x68 => println!("PLA impl"),
        0x69 => println!("ADC immediate"),
        0x6A => println!("ROR Accum"),
        0x6C => println!("JMP indr"),
        0x6D => println!("ADC abs"),
        0x6E => println!("ROR abs"),

        0x70 => println!("BVS rel"),
        0x71 => println!("ADC (indr,Y)"),
        0x75 => println!("ADC (zpg,X)"),
        0x76 => println!("ROR (zpg,X)"),
        0x78 => println!("SEI impl"),
        0x79 => println!("ADC (abs,Y)"),
        0x7D => println!("ADC (abs,X)"),
        0x7E => println!("ROR (abs,X)"),

        0x81 => println!("STA (indr,X)"),
        0x84 => println!("STY zpg"),
        0x85 => println!("STA zpg"),
        0x86 => println!("STX zpg"),
        0x88 => println!("DEY impl"),
        0x8A => println!("TXA impl"),
        0x8C => println!("STY impl"),
        0x8D => println!("STA impl"),
        0x8E => println!("STX impl"),

        _ => println!("INVALID/UNIMPLEMENTED OPCODE"),

    }

}

fn main() {

    read_opcode(0x81);

    println!("End of main");
}