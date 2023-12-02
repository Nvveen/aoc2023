pub fn day2_1(input: &str) -> u32 {
    let valid_games = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(":");
            let id = parts
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let turns = parts.next().unwrap().trim().split(";");
            for turn in turns {
                let grabs = turn.trim().split(",");
                for grab in grabs {
                    let mut grab_parts = grab.trim().split(" ");
                    let count = grab_parts.next().unwrap().parse::<u32>().unwrap();
                    let color = grab_parts.next().unwrap().trim();
                    if color == "red" && count > 12 {
                        return None;
                    } else if color == "green" && count > 13 {
                        return None;
                    } else if color == "blue" && count > 14 {
                        return None;
                    }
                }
            }
            println!("Game {} is valid", id);
            return Some(id);
        })
        .sum();
    valid_games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(day2_1(input), 8);
    }
}
