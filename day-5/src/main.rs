use std::error::Error;
use std::fs;
use std::str::{FromStr};

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<u8> // 0 index is bottom crate
}

impl Stack {
    fn new() -> Self {
        Self { crates: vec![] }
    }
}

struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, Copy)]
struct ProcedureParseError;
impl FromStr for Procedure {
    type Err = ProcedureParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [_, count, _, from, _, to] = s.splitn(6, ' ').collect::<Vec<&str>>()[..]
            else { return Err(ProcedureParseError) };

        let count_num = count.parse::<usize>().map_err(|_| ProcedureParseError)?;
        let from_num = from.parse::<usize>().map_err(|_| ProcedureParseError)?;
        let to_num = to.parse::<usize>().map_err(|_| ProcedureParseError)?;

        Ok(Procedure{from: from_num - 1, to: to_num - 1, count: count_num})
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    // assume there are less than 10 stacks
    let mut stacks = vec![Box::new(Stack::new()); 9];

    let content = fs::read_to_string(FILE_PATH)?;
    let mut row: i32 = -1;

    let mut skip = 2;
    for arr in content.as_bytes().chunks(4) {
        let [_, b, _, d] = arr else { panic!("cannot parse this") };
        row += 1;
        match b {
            65..=90 => stacks[row as usize].crates.push(*b),
            49..=57 => break,
            32 => (),
            _ => panic!("unknown condition: _{}_{}", *b as char, *d as char),
        }
        if *d == 10 { // UNIX newline
            skip += 1;
            row = -1;
            continue
        }
    }

    stacks.iter_mut().for_each(|stack| stack.crates.reverse());

    content.lines().skip(skip).for_each(|line| {
        let proc = line.parse::<Procedure>().map_err(|_| panic!("cannot parse moves")).unwrap();

        let stack_len = stacks[proc.from].crates.len();
        let mut crates = stacks[proc.from].crates.drain((stack_len-proc.count)..).collect();
        stacks[proc.to].crates.append(&mut crates);
    });

    let mut result: String = String::from("");
    stacks.iter().for_each(|stack| {
        if let Some(c) = stack.crates.last() {
            result.push(*c as char);
        }
    });
    println!("{}", result);
    Ok(())
}
