use fancy_regex::Regex;
use std::collections::HashMap;

fn sum_digits(input: &str) -> u32 {
    // iterate over lines, filter out non-digits, get first and last, sum
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}

fn sum_digits_expanded(input: &str) -> u32 {
    let digits_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
    let r = "(?=(\\d|".to_owned()
        + &digits_map
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join("|")
        + "))";
    let regex = Regex::new(&r).expect("invalid regex");
    input
        .lines()
        .map(|line| {
            let digits: Vec<_> = regex
                .captures_iter(line)
                .filter_map(|cap| {
                    let m = cap.unwrap().get(1).unwrap().as_str();
                    if let Some(d) = digits_map.get(m) {
                        Some(*d)
                    } else if let Ok(d) = m.parse::<u32>() {
                        Some(d)
                    } else {
                        None
                    }
                })
                .collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}

const INPUT: &str = include_str!("calibration.txt");

pub fn day1_1() -> u32 {
    sum_digits(INPUT)
}

pub fn day1_2() -> u32 {
    sum_digits_expanded(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // multiline string
        let input = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;
        let output = sum_digits(input);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_2() {
        let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;
        let output = sum_digits_expanded(input);
        assert_eq!(output, 281);
    }

    #[test]
    fn test_3() {
        let input = r#"eighthree
sevenine"#;
        let output = sum_digits_expanded(input);
        assert_eq!(output, 83 + 79);
    }
}
