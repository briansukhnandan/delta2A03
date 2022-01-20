#[allow(dead_code)]
struct CPU {
    // 3 8-bit general purpose registers A, X, and Y.
    a: i8,
    x: i8,
    y: i8,

    // Stack ptr which is hardcoded between 0x100 -> 0x1ff.
    sp: u16,

}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            
            // Stack ptr starts @ 0x100 (256) -> ends @ 0x1ff (511)
            sp: 0x100u16,
        }
    }
}

impl CPU {
    fn test_increment(&mut self) {
        println!("incrementing a register");
        self.a = self.a + 1;
    }
}

fn main() {
    let mut cpu = CPU { ..Default::default() };
    cpu.test_increment();

    println!("{}", cpu.a)
}