use std::error::Error;
use std::fs;

const OFFSET: u8 = 97;
// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut flags = 0u32;
    let mut idx = 3;
    'window: for window in fs::read_to_string(FILE_PATH)?.as_bytes().windows(4) {
        idx += 1;
        for v in window {
            if flags & (1 << (*v - OFFSET)) != 0 {
                flags &= 0;
                continue 'window;
            }
            flags |= 1 << (*v - OFFSET);
        }
        break;
    };

    println!("start of sequence: {}", idx);
    Ok(())
}
