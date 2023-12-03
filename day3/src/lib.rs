fn sum_valid_numbers(input: &str) -> u32 {
    let r = Regex::new("([0-9]+)").unwrap();
    for line in input.lines() {
        for m in r.matches(line) {
            println!("{}", m.as_str());
        }
    }
    0
}

pub fn day3_1() -> u32 {
    0
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
