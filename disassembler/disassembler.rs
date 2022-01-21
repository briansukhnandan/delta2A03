fn read_opcode() {

    let opcode = 0x00;

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

        _ => println!("INVALID/UNIMPLEMENTED OPCODE"),

    }

}

fn main() {

    read_opcode();

    println!("End of main");
}