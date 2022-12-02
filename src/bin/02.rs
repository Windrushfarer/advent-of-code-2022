#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn get_move_score(m: &Move) -> u32 {
    return match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn get_result_score(m: &Result) -> u32 {
    return match m {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    };
}

fn parse_enemy_move(s: &str) -> Option<Move> {
    return match s {
        "A" => Some(Move::Rock),
        "B" => Some(Move::Paper),
        "C" => Some(Move::Scissors),
        _ => None
    }
}

fn parse_my_move(s: &str) -> Option<Move> {
    return match s {
        "X" => Some(Move::Rock),
        "Y" => Some(Move::Paper),
        "Z" => Some(Move::Scissors),
        _ => None
    }
}

fn parse_move_to_result(s: &str) -> Option<Result> {
    return match s {
        "X" => Some(Result::Lose),
        "Y" => Some(Result::Draw),
        "Z" => Some(Result::Win),
        _ => None
    }
}

fn get_result(move1: &Move, move2: &Move) -> Result {
    return match move1 {
        Move::Rock => {
            match move2 {
                Move::Rock => Result::Draw,
                Move::Paper => Result::Win,
                Move::Scissors => Result::Lose,
            }
        }
        Move::Paper => {
            match move2 {
                Move::Rock => Result::Lose,
                Move::Paper => Result::Draw,
                Move::Scissors => Result::Win,
            }
        }
        Move::Scissors => {
            match move2 {
                Move::Rock => Result::Win,
                Move::Paper => Result::Lose,
                Move::Scissors => Result::Draw,
            }
        }
    }
}

fn get_move(enemy_move: &Move, result: &Result) -> Move {
    return match enemy_move {
        Move::Rock => {
            match result {
                Result::Win => Move::Paper,
                Result::Lose => Move::Scissors,
                Result::Draw => Move::Rock,
            }
        }
        Move::Paper => {
            match result {
                Result::Win => Move::Scissors,
                Result::Lose => Move::Rock,
                Result::Draw => Move::Paper,
            }
        }
        Move::Scissors => {
            match result {
                Result::Win => Move::Rock,
                Result::Lose => Move::Paper,
                Result::Draw => Move::Scissors,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves: Vec<Vec<Move>> = input
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|pair| vec![
            parse_enemy_move(pair[0]).unwrap(),
            parse_my_move(pair[1]).unwrap()
        ])
        .collect();
    let result = moves.iter()
        .fold(0, |res, pair| {
            let round_result = get_result(&pair[0], &pair[1]);
            let winner_score = get_result_score(&round_result);
            let move_score = get_move_score(&pair[1]);

            res + winner_score + move_score
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves: Vec<(Move, Result)> = input
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|pair| (parse_enemy_move(pair[0]).unwrap(), parse_move_to_result(pair[1]).unwrap()))
        .collect();

    let result = moves.iter()
        .fold(0, |res, pair| {
            let (enemy_move, round_result) = pair;
            let winner_score = get_result_score(&round_result);
            let my_move = get_move(&enemy_move, &round_result);
            let move_score = get_move_score(&my_move);

            res + winner_score + move_score
        });

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
