use crate::Output;

enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn parse_play(play: &char) -> Option<Play> {
    return match play {
        'A' | 'X' => Some(Play::Rock),
        'B' | 'Y' => Some(Play::Paper),
        'C' | 'Z' => Some(Play::Scissors),
        _ => None,
    };
}

fn win_lose_draw(opponent: &Play, player: &Play) -> Outcome {
    return match opponent {
        Play::Rock => match player {
            Play::Rock => Outcome::Draw,
            Play::Paper => Outcome::Win,
            Play::Scissors => Outcome::Lose,
        },
        Play::Paper => match player {
            Play::Rock => Outcome::Lose,
            Play::Paper => Outcome::Draw,
            Play::Scissors => Outcome::Win,
        },
        Play::Scissors => match player {
            Play::Rock => Outcome::Win,
            Play::Paper => Outcome::Lose,
            Play::Scissors => Outcome::Draw,
        },
    };
}

fn predict_play(play: &Play, outcome: &Outcome) -> Play {
    return match play {
        Play::Rock => match outcome {
            Outcome::Lose => Play::Scissors,
            Outcome::Draw => Play::Rock,
            Outcome::Win => Play::Paper,
        },
        Play::Paper => match outcome {
            Outcome::Lose => Play::Rock,
            Outcome::Draw => Play::Paper,
            Outcome::Win => Play::Scissors,
        },
        Play::Scissors => match outcome {
            Outcome::Lose => Play::Paper,
            Outcome::Draw => Play::Scissors,
            Outcome::Win => Play::Rock,
        },
    };
}

pub fn run(input: &str) -> Output {
    // part 1
    let mut part1: i32 = 0;
    for ln in input.trim().lines() {
        let opponent_code = match ln.chars().nth(0) {
            None => continue,
            Some(code) => code,
        };
        let player_code = match ln.chars().nth(2) {
            None => continue,
            Some(code) => code,
        };
        let opponent_play = match parse_play(&opponent_code) {
            None => continue,
            Some(play) => play,
        };
        let player_play = match parse_play(&player_code) {
            None => continue,
            Some(play) => play,
        };
        let outcome = win_lose_draw(&opponent_play, &player_play);

        // score
        // +1 for player Rock (regardless)
        // +2 for player Paper (regardless)
        // +3 for player Scissors (regardless)
        // +0 for player lose
        // +3 for player draw
        // +6 for player win
        match player_play {
            Play::Rock => part1 += 1,
            Play::Paper => part1 += 2,
            Play::Scissors => part1 += 3,
        }
        match outcome {
            Outcome::Lose => {}
            Outcome::Draw => part1 += 3,
            Outcome::Win => part1 += 6,
        }
    }

    // part 2
    let mut part2: i32 = 0;
    for ln in input.trim().lines() {
        let opponent_code = match ln.chars().nth(0) {
            None => continue,
            Some(code) => code,
        };
        let player_code = match ln.chars().nth(2) {
            None => continue,
            Some(code) => code,
        };
        let opponent_play = match parse_play(&opponent_code) {
            None => continue,
            Some(play) => play,
        };
        let desired_outcome = match player_code {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => Outcome::Lose,
        };
        let player_play = predict_play(&opponent_play, &desired_outcome);
        // score
        // +1 for player Rock (regardless)
        // +2 for player Paper (regardless)
        // +3 for player Scissors (regardless)
        // +0 for player lose
        // +3 for player draw
        // +6 for player win
        match player_play {
            Play::Rock => part2 += 1,
            Play::Paper => part2 += 2,
            Play::Scissors => part2 += 3,
        }
        match desired_outcome {
            Outcome::Lose => {}
            Outcome::Draw => part2 += 3,
            Outcome::Win => part2 += 6,
        }
    }

    return Output {
        part1: part1.to_string().to_owned(),
        part2: part2.to_string().to_owned(),
    };
}
