use std::error::Error;
use std::fs;

const FILE_PATH: &str = "src/example.txt";
// const FILE_PATH: &str = "src/puzzle.txt";

fn main() -> Result<(), Box<dyn Error>> {
    fs::read_to_string(FILE_PATH)?;

    Ok(())
}
