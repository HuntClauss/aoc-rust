use std::fs;
use std::error::Error;

// const FILE_PATH: &str = "src/example.txt";
const FILE_PATH: &str = "src/puzzle.txt";

#[derive(Copy, Clone)]
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
    fn response(&self, _: State) -> Choice;
}

trait Strategies {
    fn win(self) -> Choice;
    fn lose(self) -> Choice;
    fn draw(self) -> Choice;
}

impl Strategies for Choice {
    fn win(self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn lose(self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn draw(self) -> Choice {
        self
    }
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

    fn response(&self, action: State) -> Choice {
        match action {
            State::Draw => self.draw(),
            State::Won => self.win(),
            State::Lose => self.lose(),
            _ => panic!("unimplemented state of game"),
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

fn convert_response(symbol: char) -> State {
    match symbol {
        'X' => State::Lose,
        'Y' => State::Draw,
        'Z' => State::Won,
        _ => panic!("unimplemented state of game"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut own_points = 0;
    let mut opponents_points = 0;

    fs::read_to_string(FILE_PATH)?.lines().map(|line| {
        let mut chars = line.chars();

        let player1 = convert_symbols(chars.next().expect("cannot get 0 index char"));
        let _ = chars.next();
        let response = convert_response(chars.next().expect("cannot get 2 index char"));

        (player1, response)
    }).for_each(|(opp, me)| {
        let resp = opp.response(me);
        let (a, b) = resp.play(&opp);
        own_points += a;
        opponents_points += b;
    });

    println!("Mine points: {}\nOpponent's points: {}", own_points, opponents_points);

    Ok(())
}
