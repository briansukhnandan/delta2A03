#[allow(dead_code)]
#[allow(non_snake_case)]

// Other modules
mod flag_ops;

// Our main struct to hold fields such as registers and whatnot.
struct CPU {
    // 3 8-bit general purpose registers A, X, and Y.
    a: u8,
    x: u8,
    y: u8,

    // Our flags register. Refer to notes for all individual bits.
    p: u8,

    // Stack ptr which is hardcoded between 0x100 -> 0x1ff.
    sp: u16,

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
            
            sp: 0x100u16,
            pc: 0x0u16,

            cycle_count: 0,
            opcode: 0x0u8
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

    fn test(&mut self) {
        println!("incrementing X register");
        self.x = self.x + 1;

        println!("Flipping flags/testing module");
        self.p = flag_ops::toggle_N_flag(self.p);
        println!("{:#08b}", self.p);
    }

    fn process_opcode(&self) {
        println!("TODO");
    }

}

fn main() {
    let mut cpu = CPU { ..Default::default() };
    cpu.test();
    cpu.dump_registers();
}