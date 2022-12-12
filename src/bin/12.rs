use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    return input.lines()
        .map(|s| String::from(s))
        .map(|s| s.into_bytes().into_iter().collect::<Vec<u8>>())
        .collect();
}

fn step_into(data: &Vec<Vec<u8>>, cur: u8, row: usize, col: usize) -> Option<(usize, usize)> {
    return data.get(row)
        .and_then(|v| {
            v.get(col)
                .and_then(|next| {
                    let next = match *next {
                        b'S' => b'a',
                        b'E' => b'z',
                        _ => *next
                    };

                    let cur = match cur {
                        b'S' => b'a',
                        b'E' => b'z',
                        _ => cur
                    };

                    if next >= cur && next - cur <= 1 || cur > next {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        });
}

fn step_up(data: &Vec<Vec<u8>>, cur: u8, row: usize, col: usize) -> Option<(usize, usize)> {
    return data.get(row)
        .and_then(|v| {
            v.get(col)
                .and_then(|next| {
                    let next = match *next {
                        b'S' => b'a',
                        b'E' => b'z',
                        _ => *next
                    };

                    let cur = match cur {
                        b'S' => b'a',
                        b'E' => b'z',
                        _ => cur
                    };

                    if cur >= next && cur - next <= 1 || next > cur {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        });
}

fn find_shortest(data: &Vec<Vec<u8>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut hs = HashMap::new();
    for (idx_row, row) in data.iter().enumerate() {
        for (idx_col, b) in row.iter().enumerate() {
            if *b == b'S' {
                queue.push_back((idx_row, idx_col));
                hs.insert((idx_row, idx_col), 0);
                break;
            }
        }
    }

    loop {
        if let Some((row, col)) = queue.pop_front() {
            let current = data[row][col];
            let path_length = *hs.get(&(row, col)).expect("100% exist");

            if current == b'E' {
                return path_length;
            }

            if let Some(next) = step_into(&data, current, row + 1, col)
                .and_then(|x| if hs.contains_key(&x) { None } else { Some(x) }) {
                queue.push_back(next);
                hs.insert(next, path_length + 1);
            }

            if row > 0 {
                if let Some(next) = step_into(&data, current, row - 1, col)
                    .and_then(|x| if hs.contains_key(&x) { None } else { Some(x) }) {
                    queue.push_back(next);
                    hs.insert(next, path_length + 1);
                }
            }

            if let Some(next) = step_into(&data, current, row, col + 1)
                .and_then(|x| if hs.contains_key(&x) { None } else { Some(x) }) {
                queue.push_back(next);
                hs.insert(next, path_length + 1);
            }

            if col > 0 {
                if let Some(next) = step_into(&data, current, row, col - 1)
                    .and_then(|x| if hs.contains_key(&x) { None } else { Some(x) }) {
                    queue.push_back(next);
                    hs.insert(next, path_length + 1);
                }
            }
        } else {
            break;
        }
    }

    return 0;
}

fn find_trail(data: &Vec<Vec<u8>>) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();
    for (i, row) in data.iter().enumerate() {
        for (j, b) in row.iter().enumerate() {
            if *b == b'E' {
                queue.push_back((i, j));
                paths.insert((i, j), 0);
                break;
            }
        }
    }

    let mut min: Option<i32> = None;

    loop {
        if let Some((row, col)) = queue.pop_front() {
            let current = data[row][col];
            let path_length = *paths.get(&(row, col)).expect("100% exist");

            if current == b'a' {
                min = min.map_or(Some(path_length), |x| Some(x.min(path_length)));
            }

            if let Some(next) = step_up(&data, current, row + 1, col)
                .and_then(|x| if paths.contains_key(&x) { None } else { Some(x) }) {
                queue.push_back(next);
                paths.insert(next, path_length + 1);
            }

            if row > 0 {
                if let Some(next) = step_up(&data, current, row - 1, col)
                    .and_then(|x| if paths.contains_key(&x) { None } else { Some(x) }) {
                    queue.push_back(next);
                    paths.insert(next, path_length + 1);
                }
            }

            if let Some(next) = step_up(&data, current, row, col + 1)
                .and_then(|x| if paths.contains_key(&x) { None } else { Some(x) }) {
                queue.push_back(next);
                paths.insert(next, path_length + 1);
            }

            if col > 0 {
                if let Some(next) = step_up(&data, current, row, col - 1)
                    .and_then(|x| if paths.contains_key(&x) { None } else { Some(x) }) {
                    queue.push_back(next);
                    paths.insert(next, path_length + 1);
                }
            }
        } else {
            break;
        }
    }

    return min;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    Some(find_shortest(&grid) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    Some(find_trail(&grid).unwrap() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
