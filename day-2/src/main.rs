use std::fs;
use std::error::Error;

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum State {
    Won,
    Draw,
    Lose,
    Undecided,
}

trait Game {
    fn points(&self) -> i32;
    fn play(&self, _: &Choice) -> (i32, i32);
}

impl Game for Choice {
    fn points(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn play(&self, opponent: &Choice) -> (i32, i32) {
        let result = match (self, opponent) {
            (Choice::Rock, Choice::Paper) => State::Lose,
            (Choice::Rock, Choice::Scissors) => State::Won,
            (Choice::Rock, Choice::Rock) => State::Draw,
            (Choice::Paper, Choice::Scissors) => State::Lose,
            (Choice::Paper, Choice::Paper) => State::Draw,
            (Choice::Scissors, Choice::Scissors) => State::Draw,
            _ => State::Undecided,
        };

        match result {
            State::Won => (6 + self.points(), opponent.points()),
            State::Draw => (3 + self.points(), 3 + opponent.points()),
            State::Lose => (self.points(), 6 + opponent.points()),
            State::Undecided => {
                let (b, a) = opponent.play(self);
                (a, b)
            },
        }
    }
}

fn convert_symbols(symbol: char) -> Choice {
    match symbol {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!("unexpected symbol: {}", symbol),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut own_points = 0;
    let mut opponents_points = 0;

    fs::read_to_string(FILE_PATH)?.lines().map(|line| {
        let mut chars = line.chars();

        let player1 = convert_symbols(chars.next().expect("cannot get 0 index char"));
        let _ = chars.next();
        let player2 = convert_symbols(chars.next().expect("cannot get 2 index char"));

        (player1, player2)
    }).for_each(|(opp, me)| {
        let (a, b) = me.play(&opp);
        own_points += a;
        opponents_points += b;
    });

    println!("Mine points: {}\nOpponent's points: {}", own_points, opponents_points);

    Ok(())
}
