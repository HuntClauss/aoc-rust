use std::fs;
use std::error::Error;

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for char {
    // return value in range 1..=52
    // accepted chars a..=z and A..=Z
    fn priority(&self) -> u8 {
        let val: u8 = self.to_string().as_bytes()[0];
        match val {
            97..=122 => val - 96,
            65..=90 => val - 65 + 27,
            _ => panic!("")
        }
    }
}

fn from_priority(val: u8) -> char {
    match val {
        1..=26 => (val + 96) as char,
        27..=52 => (val - 27 + 65) as char,
        _ => panic!("cannot convert '{}' to char (invalid priority)", val),
    }
}

fn find_error(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    let mut store1 = 0u64;
    let mut store2 = 0u64;
    let mut store3 = 0u64;

    rucksack1.chars().for_each(|v| store1 |= 1 << (v.priority() - 1));
    rucksack2.chars().for_each(|v| store2 |= 1 << (v.priority() - 1));
    rucksack3.chars().for_each(|v| store3 |= 1 << (v.priority() - 1));

    let common = store1 & store2 & store3;
    if common == 0 {
        panic!("there is no common item in rucksacks");
    }

    let mut idx = 1;
    while common >> idx != 0 {
        idx += 1;
    }

    from_priority(idx as u8)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut sum_priorities: u64 = 0;
    fs::read_to_string(FILE_PATH)?.lines().collect::<Vec<&str>>().chunks(3).for_each(|chunk| {
        let [line1, line2, line3] = chunk[..3] else { panic!("something went really wrong") };
        let err = find_error(line1, line2, line3);
        sum_priorities += err.priority() as u64;

        // let (line1, line2, line3) = chunk.get;
        // let len = line.len() / 2;
        // let (rucksack1, rucksack2) = (&line[..len], &line[len..]);
        // let err = find_error(rucksack1, rucksack2);
        // sum_priorities += err.priority() as u64;
        // // println!("error: {}", err);
    });

    println!("Sum of priorities: {}", sum_priorities);

    Ok(())
}
