pub fn day6_1(input: &str) -> usize {
    let values = input
        .split_whitespace()
        .filter_map(|word| word.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let (time, record) = values.split_at(values.len() / 2);
    let values = time.iter().zip(record.iter()).collect::<Vec<_>>();
    values
        .iter()
        .map(|(&time, &record)| get_record_breakers(time, record))
        .product()
}

pub fn day6_2(input: &str) -> usize {
    let words = input
        .split_whitespace()
        .filter(|word| !word.ends_with(":"))
        .collect::<Vec<_>>();
    let (time, record) = words.split_at(words.len() / 2);
    let time = time.join("").parse::<u64>().unwrap();
    let record = record.join("").parse::<u64>().unwrap();
    get_record_breakers(time, record)
}

fn get_record_breakers(time: u64, record: u64) -> usize {
    (0..time)
        .map(|t| {
            let speed = t;
            let remaining_time = time - t;
            let distance = speed * remaining_time;
            distance
        })
        .filter(|&d| d > record)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_day5_1() {
        assert_eq!(day6_1(INPUT), 288);
    }

    #[test]
    fn test_day5_2() {
        assert_eq!(day6_2(INPUT), 71503);
    }
}
