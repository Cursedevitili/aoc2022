use std::process;
use aoc2022::shared;
use crate::MatchOutcome::{Lose, Tie, Win};
use crate::PlayerMove::{Paper, Rock, Scissors};

#[derive(Debug, Copy, Clone)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors
}

impl PlayerMove {
    fn decode_enemy  (move_to_decode: &str) -> PlayerMove {
        match move_to_decode {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => Scissors
        }
    }

    fn decode_my_move (move_to_decode: &str) -> PlayerMove {
        match move_to_decode {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => Scissors
        }
    }

    fn get_value (&self) -> i32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissors => 3
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MatchOutcome {
    Win,
    Lose,
    Tie
}

#[derive(Debug, Copy, Clone)]
struct Match {
    your_move: PlayerMove,
    enemy_move: PlayerMove,
    outcome: Option<MatchOutcome>
}

impl Match {
    fn new(your_move: PlayerMove, enemy_move: PlayerMove) -> Match {
        Match {
            your_move,
            enemy_move,
            outcome: None
        }
    }

    fn get_outcome(&mut self) {
        match self.your_move {
            PlayerMove::Rock => {
                match self.enemy_move {
                    PlayerMove::Rock => { self.outcome = Some(Tie) }
                    PlayerMove::Paper => { self.outcome = Some(Lose) }
                    PlayerMove::Scissors => { self.outcome = Some(Win)}
                }
            }
            PlayerMove::Paper => {
                match self.enemy_move {
                    PlayerMove::Rock => { self.outcome = Some(Win) }
                    PlayerMove::Paper => { self.outcome = Some(Tie) }
                    PlayerMove::Scissors => { self.outcome = Some(Lose)}
                }
            }
            PlayerMove::Scissors => {
                match self.enemy_move {
                    PlayerMove::Rock => { self.outcome = Some(Lose) }
                    PlayerMove::Paper => { self.outcome = Some(Win) }
                    PlayerMove::Scissors => { self.outcome = Some(Tie)}
                }
            }
        }
    }

    fn get_score (&self) -> i32 {
        match &self.outcome {
            Some(res) => {
                match res {
                    Win => {
                        6 + self.your_move.get_value()
                    }
                    Lose => { self.your_move.get_value()}
                    Tie => { self.your_move.get_value() + 3}
                }
            }
            None => {-1000000}
        }
    }
}


fn main() {
    let input = shared::load_input("day2demo.txt").unwrap_or_else(|_|
        {
            process::exit(1)
        }
    );
    let mut score = 0;
    for line in input.lines() {
        let mut  iter = line.char_indices();
        let (_, enemy_move) = iter.next().unwrap();
        println!("Enemy move {}", enemy_move);
        iter.next();
        let (_, my_move) = iter.next().unwrap();
        println!("My move {}", my_move);
        let mut setup = Match::new(
            PlayerMove::decode_my_move(&my_move.to_string()),
            PlayerMove::decode_enemy(&enemy_move.to_string())
        );
        setup.get_outcome();
        println!("{:?}", setup);
        let score_from_round = setup.get_score();
        println!("Round score:{}", score_from_round);
        score += score_from_round;
    }

    println!("{}", score);
}