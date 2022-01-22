use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

#[allow(non_snake_case)]

// Pipe the output of this program to a file using bash.
// i.e. (./ROM_dump_tool) > mem_dump.txt
fn dump_ROM_hex(path: String) -> io::Result<()> {

    // We will simply open the file, create a buffer
    // with default sizing of reading 1 byte at a time.
    // Then read the value from the buffer.
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let mut counter = 0;
    for value in buffer {

        counter = counter + 1;        
        print!("{:#02x} ", value);

        if counter % 5 == 0 {
            println!("");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let _res = dump_ROM_hex(String::from("/Users/brian/Documents/NES Test ROMs/instr_test_v5/01-basics.nes"));
    Ok(())
}
