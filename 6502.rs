#[allow(dead_code)]
#[allow(non_snake_case)]

// Our main struct to hold fields such as registers and whatnot.
struct CPU {
    // 3 8-bit general purpose registers A, X, and Y.
    a: u8,
    x: u8,
    y: u8,

    // Our flags register. Refer to notes for all individual bits.
    p: i8,

    // Stack ptr which is hardcoded between 0x100 -> 0x1ff.
    sp: u16,

    // 16 bit program counter.
    pc: u16

}

// Default values for our 6502 if we are initializing anywhere else.
impl Default for CPU {
    fn default() -> CPU {
        CPU {
            a: 0, // Also known as the accumulator.
            x: 0,
            y: 0,

            p: 0b0000_0000, // Explicitly declare flags as binary for simplicity.
            
            sp: 0x100u16,
            pc: 0x0u16,

        }
    }
}

// Functions for CPU.
impl CPU {

    fn dump_registers(&self) {
        println!("Accumulator: {}", self.a);
        println!("X register: {}", self.x);
        println!("Y register: {}", self.y);
    }

    fn test_increment(&mut self) {
        println!("incrementing a register");
        self.x = self.x + 1;
    }
}

fn main() {
    let mut cpu = CPU { ..Default::default() };
    cpu.test_increment();

    // println!("{}", cpu.a)
    cpu.dump_registers();

}