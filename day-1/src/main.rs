use std::error::Error;
use std::fs;

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

trait Top3Values {
    fn top3(&self) -> [i32; 3];
}

impl Top3Values for Vec<i32> {
    fn top3(&self) -> [i32; 3] {
        let mut result = [0, 0, 0]; // [a, b, c] a > b > c

        for v in self.clone().into_iter() {
            (result[0], result[1], result[2]) = match result {
                [a, b, c] if v >= a => (v, a, b),
                [a, b, c] if v >= b => (a, v, b),
                [a, b, c] if v >= c => (a, b, v),
                [a, b, c] => (a, b, c)
            }
        }

        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(FILE_PATH)?;
    let lines: Vec<i32> = content
        .split("\n")
        .map(|v| v.parse::<i32>().unwrap_or(0))
        .collect();

    let mut calories: Vec<i32> = vec![];

    let mut sum = 0;
    for v in lines.into_iter() {
        if v == 0 {
            calories.push(sum);
            sum = 0;
        } else {
            sum += v;
        }
    }
    calories.push(sum);

    println!("{:?}", calories.top3().into_iter().reduce(|a, b| a + b).unwrap_or(0));

    Ok(())
}
