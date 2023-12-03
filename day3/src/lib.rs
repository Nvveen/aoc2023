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

const INPUT : &str = include_str!("input");

pub fn day3_1() -> u32 {
    sum_valid_numbers(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_valid_numbers() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(sum_valid_numbers(input), 4361);
    }
}
