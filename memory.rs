#[allow(dead_code)]
#[allow(non_snake_case)]

pub struct Memory {

    // Memory map ranges from 0x0 -> 0xFFFF.
    // Since we must have 0xFFFF + 1 total bytes allocated,
    // use u32.
    pub TOTAL_MEMORY_SIZE: u32,

    // RAM ranges between 0x0 -> 0x0800 (non-inclusive meaning 0->0x07FF).
    pub RAM_START_ADDRESS: u16,
    pub RAM_END_ADDRESS: u16,

    // Each page is 256 bytes, and there are also
    // 256 pages in total: (0xFF + 0x1) * (0xFF + 0x1) = (0xFFFF + 1) 
    pub PAGE_SIZE: u16,

    // Our allocated memory.
    pub DATA: Vec<u8>,

}

// Default values for our Memory.
impl Default for Memory {
    fn default() -> Memory {
        Memory {
            TOTAL_MEMORY_SIZE: 0xFFFF + 1, // 65536
            RAM_START_ADDRESS: 0x0,
            RAM_END_ADDRESS: 0x0800, // 2048
            PAGE_SIZE: 0xFF + 1, // 256
            DATA: Vec::new()
        }
    }
}

// Initialization/helper functions to interact with our memory.
impl Memory {

    // Initializes 256 pages each 256 bytes long with value 0.
    pub fn initialize_memory(&mut self) {        
        // self.DATA = vec![0, (0xFFFF)+1] // Throws error for some reason.
        for _i in 0..self.TOTAL_MEMORY_SIZE {
            self.DATA.push(0x0);
        }
        println!("Memory initialized!");
    }

    pub fn read_from_memory_address(&self, address: usize) -> u8 {
        return self.DATA[address];
    }

    pub fn write_to_memory_address(&mut self, address: usize, value: u8) {
        self.DATA[address] = value;
    }

    // Zero's out memory addr 0x0 -> 0x7FF.
    pub fn clear_RAM(&mut self) {
        for i in self.RAM_START_ADDRESS..self.RAM_END_ADDRESS {
            self.write_to_memory_address(i as usize, 0x0);
        }
    }

}

fn main() {

    let mut memory = Memory { ..Default::default() };
    memory.initialize_memory();
    
    // println!("{}", memory.DATA[0x0]);
    // println!("{}", memory.read_from_memory_address(0xFFFF));
    // memory.write_to_memory_address(0xFFFF, 0x45);
    // println!("{}", memory.read_from_memory_address(0xFFFF));

    memory.write_to_memory_address(0x07FF, 0x45);
    println!("{}", memory.read_from_memory_address(0x07FF));
    memory.clear_RAM();
    println!("{}", memory.read_from_memory_address(0x07FF));

}