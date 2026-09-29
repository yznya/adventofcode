use std::fs;

#[derive(Debug, PartialEq)]
enum GameOutcome {
    Won,
    Draw,
    Lost,
}

#[derive(Debug, PartialEq)]
enum Jajanken {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct Round {
    opponent_choice: Jajanken,
    desired_outcome: GameOutcome,
}

fn main() {
    let data = fs::read_to_string("problem.txt").unwrap();
    let rounds: Vec<_> = data
        .split('\n')
        .map(|round_str| {
            let mut round_choices = round_str.split(' ');
            let opponent_choice = match round_choices.next() {
                Some("A") => Jajanken::Rock,
                Some("B") => Jajanken::Paper,
                Some("C") => Jajanken::Scissor,
                _ => unreachable!(),
            };
            let desired_outcome = match round_choices.next() {
                Some("X") => GameOutcome::Lost,
                Some("Y") => GameOutcome::Draw,
                Some("Z") => GameOutcome::Won,
                _ => unreachable!(),
            };
            Round {
                opponent_choice,
                desired_outcome,
            }
        })
        .collect();

    let mut score = 0;

    for round in &rounds {
        let mut outcome = GameOutcome::Lost;
        if round.desired_outcome == GameOutcome::Lost {
            score += 1;
            if round.opponent_choice == Jajanken::Scissor {
                outcome = GameOutcome::Won;
            } else if round.opponent_choice == Jajanken::Rock {
                outcome = GameOutcome::Draw;
            }
        } else if round.desired_outcome == GameOutcome::Draw {
            score += 2;
            if round.opponent_choice == Jajanken::Rock {
                outcome = GameOutcome::Won;
            } else if round.opponent_choice == Jajanken::Paper {
                outcome = GameOutcome::Draw;
            }
        } else if round.desired_outcome == GameOutcome::Won {
            score += 3;
            if round.opponent_choice == Jajanken::Paper {
                outcome = GameOutcome::Won;
            } else if round.opponent_choice == Jajanken::Scissor {
                outcome = GameOutcome::Draw;
            }
        }

        score += match outcome {
            GameOutcome::Won => 6,
            GameOutcome::Draw => 3,
            GameOutcome::Lost => 0,
        };
    }

    println!("Score with misunderstood instructions: {}", score);

    score = 0;
    for round in &rounds {
        let play = match round.opponent_choice {
            Jajanken::Rock => match round.desired_outcome {
                GameOutcome::Lost => Jajanken::Scissor,
                GameOutcome::Draw => Jajanken::Rock,
                GameOutcome::Won => Jajanken::Paper,
            },

            Jajanken::Paper => match round.desired_outcome {
                GameOutcome::Lost => Jajanken::Rock,
                GameOutcome::Draw => Jajanken::Paper,
                GameOutcome::Won => Jajanken::Scissor,
            },

            Jajanken::Scissor => match round.desired_outcome {
                GameOutcome::Lost => Jajanken::Paper,
                GameOutcome::Draw => Jajanken::Scissor,
                GameOutcome::Won => Jajanken::Rock,
            },
        };

        score += match play {
            Jajanken::Rock => 1,
            Jajanken::Paper => 2,
            Jajanken::Scissor => 3,
        };

        score += match round.desired_outcome {
            GameOutcome::Lost => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Won => 6,
        }
    }

    println!("Score: {}", score);
}
