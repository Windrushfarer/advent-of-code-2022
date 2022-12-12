use std::ops::RangeInclusive;

enum Command {
    ADD(i32),
    NOOP,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut values: Vec<i32> = vec![1];

    input.split("\n")
        .map(|line| {
            let words = line.split_once(" ");

            match words {
                None => Command::NOOP,
                Some((_, val)) => Command::ADD(val.parse::<i32>().unwrap())
            }
        })
        .for_each(|cmd| {
            let register = values[values.len() - 1];

            match cmd {
                Command::ADD(val) => {
                    values.push(register);
                    values.push(register + val);
                }
                Command::NOOP => {
                    values.push(register);
                }
            };
        });

    let result: i32 = [20, 60, 100, 140, 180, 220].iter()
        .map(|n| (*n as i32) * values[*n -1])
        .sum();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut values: Vec<i32> = vec![1];

    input.split("\n")
        .map(|line| {
            let words = line.split_once(" ");

            match words {
                None => Command::NOOP,
                Some((_, val)) => Command::ADD(val.parse::<i32>().unwrap())
            }
        })
        .for_each(|cmd| {
            let register = values[values.len() - 1];

            match cmd {
                Command::ADD(val) => {
                    values.push(register);
                    values.push(register + val);
                }
                Command::NOOP => {
                    values.push(register);
                }
            };
        });


    for time in 0..6  {
        let mut result = String::new();

        for i in 0..39 {
            let modifier = 40 * time;
            let index = i + modifier;
            let sprite_start = values[index] - 1;
            let range = RangeInclusive::new(sprite_start, sprite_start + 2);
            if range.contains(&(i as i32)) {
                result += "#"
            } else {
                result += "."
            }
        }

        println!("{:?}", result);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
