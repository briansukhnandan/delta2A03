#[allow(dead_code)]
#[allow(non_snake_case)]

struct Memory {

    // Memory map ranges from 0x0 -> 0xFFFF.
    // Since we must have 0xFFFF + 1 total bytes allocated,
    // use u32.
    TOTAL_MEMORY_SIZE: u32,

    // Each page is 256 bytes, and there are also
    // 256 pages in total: (0xFF + 0x1) * (0xFF + 0x1) = (0xFFFF + 1) 
    PAGE_SIZE: u16,

    // Our allocated memory.
    DATA: Vec<u8>,

}

// Default values for our Memory.
impl Default for Memory {
    fn default() -> Memory {
        Memory {
            TOTAL_MEMORY_SIZE: 0xFFFF + 1, // 65536
            PAGE_SIZE: 0xFF + 1, // 256
            DATA: Vec::new()
        }
    }
}

impl Memory {

    // Initializes 256 pages each 256 bytes long with value 0.
    fn initialize_memory(&mut self) {        
        // self.DATA = vec![0, (0xFFFF)+1] // Throws error for some reason.
        for _i in 0..(0xFFFF+1) {
            self.DATA.push(0x0);
        }
    }

    fn read_from_memory_address(&self, address: usize) -> u8 {
        return self.DATA[address];
    }

    fn write_to_memory_address(&mut self, address: usize, value: u8) {
        self.DATA[address] = value;
    }

}

fn main() {

    let mut memory = Memory { ..Default::default() };
    memory.initialize_memory();
    
    println!("{}", memory.DATA[0x0]);
    println!("{}", memory.read_from_memory_address(0xFFA1));
    memory.write_to_memory_address(0xFFA1, 0x45);
    println!("{}", memory.read_from_memory_address(0xFFA1));

}