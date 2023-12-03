use std::collections::HashSet;

use fancy_regex::Regex;

fn sum_valid_numbers(input: &str) -> u32 {
    let r = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    let line_count = input.lines().count();

    let is_symbol = |c: char| c != '.' && c != '\n' && !c.is_numeric();

    for (i, line) in input.lines().enumerate() {
        for cap in r.captures_iter(line) {
            let m = cap.unwrap().get(1).unwrap();
            let num = m.as_str().parse::<u32>().unwrap();
            let (s, e) = (m.start(), m.end());
            // check all characters around the number, including diagonals
            if s > 0 && is_symbol(line.chars().nth(s - 1).unwrap()) {
                sum += num;
                continue;
            }
            if e < line.len() && is_symbol(line.chars().nth(e).unwrap()) {
                sum += num;
                continue;
            }
            if i > 0 {
                let from = if s > 0 { s - 1 } else { s };
                let to = if e < line.len() { e + 1 } else { e };
                let prev_line = &input.lines().nth(i - 1).unwrap();
                let prev_chars = &mut prev_line.chars().skip(from).take(to - from);
                if prev_chars.any(|c| is_symbol(c)) {
                    sum += num;
                    continue;
                }
            }
            if i < line_count - 1 {
                let from = if s > 0 { s - 1 } else { s };
                let to = if e < line.len() { e + 1 } else { e };
                let next_line = input.lines().nth(i + 1).unwrap();
                let next_chars = &mut next_line.chars().skip(from).take(to - from);
                if next_chars.any(|c| is_symbol(c)) {
                    sum += num;
                    continue;
                }
            }
        }
    }
    sum
}

fn gear_ratio(input: &str) -> u32 {
    let r = Regex::new(r"(\d+|\*)").unwrap();
    let captures = input
        .lines()
        .enumerate()
        .flat_map(|(line_no, line)| r.find_iter(line).map(move |m| (line_no, m.unwrap())))
        .collect::<Vec<_>>();
    let mut sum = 0 as u32;

    for (line_no, cap) in captures.iter().filter(|(_, cap)| cap.as_str() == "*") {
        let mut acc = HashSet::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let (x, y) = (cap.start() as i32 + i, *line_no as i32 + j);
                let ch = input
                    .lines()
                    .nth(y as usize)
                    .unwrap()
                    .chars()
                    .nth(x as usize)
                    .unwrap();
                if ch.is_numeric() {
                    let num = captures
                        .iter()
                        .find(|c| {
                            c.0 == y as usize && x >= c.1.start() as i32 && x < c.1.end() as i32
                        })
                        .unwrap()
                        .1
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    acc.insert(num);
                }
            }
        }
        if acc.len() == 2 {
            sum += acc.iter().product::<u32>();
        }
    }
    sum
}

const INPUT: &str = include_str!("input");

pub fn day3_1() -> u32 {
    sum_valid_numbers(INPUT)
}

pub fn day3_2() -> u32 {
    gear_ratio(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_sum_valid_numbers() {
        assert_eq!(sum_valid_numbers(TEST_INPUT), 4361);
    }

    #[test]
    fn test_gear_ratio() {
        assert_eq!(gear_ratio(TEST_INPUT), 467835);
    }
}
