fn sum_valid_games(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(" ");
            let id = parts
                .nth(1)
                .unwrap()
                .trim_end_matches(":")
                .parse::<u32>()
                .unwrap();
            // Every odd word is a count, every even word is a color
            while let Some(word) = parts.next() {
                let count = word.parse::<u32>().unwrap();
                let color = parts.next().unwrap().trim_end_matches([',', ';']);
                if color == "red" && count > 12 {
                    return None;
                } else if color == "green" && count > 13 {
                    return None;
                } else if color == "blue" && count > 14 {
                    return None;
                }
            }
            Some(id)
        })
        .sum()
}

fn power_valid_games(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut parts = line.split(" ").skip(2);
        let (mut red, mut green, mut blue) = (0, 0, 0);
        while let Some(word) = parts.next() {
            let count = word.parse::<u32>().unwrap();
            let color = parts.next().unwrap().trim_end_matches([',', ';']);
            red = if color == "red" && count > red { count } else { red };
            green = if color == "green" && count > green { count } else { green };
            blue = if color == "blue" && count > blue { count } else { blue };
        }
        acc + (red * green * blue)
    })
}

const INPUT: &str = include_str!("input");

pub fn day2_1() -> u32 {
    sum_valid_games(INPUT)
}

pub fn day2_2() -> u32 {
    power_valid_games(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_valid_games() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(sum_valid_games(input), 8);
    }

    #[test]
    fn test_power_valid_games() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(power_valid_games(input), 2286);
    }
}
