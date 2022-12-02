use std::io;

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn compute(input: &mut Vec<String>) -> i32 {
    let mut total_points = 0;
    for element in input {
        let game = element.split(' ').collect::<Vec<&str>>();
        let player1 = match game[1] {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => Result::Draw,
        };
        let player2 = match game[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => Move::Rock,
        };
        let our_move: Move;
        match (&player1, player2) {
            (Result::Win, Move::Rock) => {
                our_move = Move::Paper;
                total_points += 6;
            }
            (Result::Win, Move::Paper) => {
                our_move = Move::Scissors;
                total_points += 6;
            }
            (Result::Win, Move::Scissors) => {
                our_move = Move::Rock;
                total_points += 6;
            }

            (Result::Draw, Move::Rock) => {
                our_move = Move::Rock;
                total_points += 3;
            }
            (Result::Draw, Move::Paper) => {
                our_move = Move::Paper;
                total_points += 3;
            }
            (Result::Draw, Move::Scissors) => {
                our_move = Move::Scissors;
                total_points += 3;
            }

            (Result::Lose, Move::Rock) => {
                our_move = Move::Scissors;
                total_points += 0;
            }
            (Result::Lose, Move::Paper) => {
                our_move = Move::Rock;
                total_points += 0;
            }
            (Result::Lose, Move::Scissors) => {
                our_move = Move::Paper;
                total_points += 0;
            }
        }

        match our_move {
            Move::Rock => total_points += 1,
            Move::Paper => total_points += 2,
            Move::Scissors => total_points += 3,
        }

    }
    return total_points;
}

fn main() {
    let mut input = Vec::new();
    io::stdin().lines().for_each(|line| {
        input.push(line.unwrap());
    });

    let result = compute(&mut input);
    print!("{}", result);
}
