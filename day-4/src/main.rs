use std::error::Error;
use std::fs;
use std::str::FromStr;

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

// both side inclusive range
#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct RangePair(Range, Range);

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn partial_overlap(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
            || (other.start <= self.start && self.start <= other.end)
            || (other.start <= self.end && self.end <= other.end)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;
impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseRangeError)?;

        let start_num = start.parse::<usize>().map_err(|_| ParseRangeError)?;
        let end_num = end.parse::<usize>().map_err(|_| ParseRangeError)?;

        Ok(Range {
            start: start_num,
            end: end_num,
        })
    }
}

impl FromStr for RangePair {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').ok_or(ParseRangeError)?;
        Ok(RangePair(left.parse()?, right.parse()?))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ranges: Vec<RangePair> = fs::read_to_string(FILE_PATH)?
        .lines()
        .map(|line| {
            line.parse::<RangePair>()
                .map_err(|err| panic!("cannot parse range pair \"{}\": {:?}", line, err))
                .unwrap()
        })
        .collect();

    let mut count = 0;
    ranges.into_iter().for_each(|pair| {
        count += if pair.0.partial_overlap(&pair.1) {
            1
        } else {
            0
        }
    });

    println!("fully contained ranges: {}", count);
    Ok(())
}
