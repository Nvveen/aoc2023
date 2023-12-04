fn get_scratch_points(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let words = line.split(" ").skip(2).collect::<Vec<_>>();
            let sets = words
                .split(|&word| word == "|")
                .map(|part| {
                    let parts = part
                        .iter()
                        .filter_map(|p| p.parse::<u32>().ok())
                        .collect::<Vec<_>>();
                    parts
                })
                .collect::<Vec<_>>();
            (sets.get(0).cloned().unwrap(), sets.get(1).cloned().unwrap())
        })
        .fold(0, |sum, (winning, nums)| {
            let winning_count = nums.iter().filter(|&num| winning.contains(&num)).count();
            if winning_count == 0 {
                return sum;
            }
            sum + (1 << winning_count - 1)
        })
}

const INPUT: &str = include_str!("input");

pub fn day4_1() -> u32 {
    get_scratch_points(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let result = get_scratch_points(input);
        assert_eq!(result, 13);
    }
}
