pub fn day9_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|nums| {
            let mut tails: Vec<i32> = Vec::new();
            let mut nums = nums;
            while nums.iter().any(|&n| n != 0) {
                tails.push(nums.last().unwrap().clone());
                let diff = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
                nums = diff;
            }
            tails.iter().sum::<i32>()
        })
        .sum::<i32>()
}

pub fn day9_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|nums| {
            let mut heads: Vec<i32> = Vec::new();
            let mut nums = nums;
            while nums.iter().any(|&n| n != 0) {
                heads.push(nums.first().unwrap().clone());
                let diff = nums.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();
                nums = diff;
            }
            heads.iter().sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_day9_1() {
        assert_eq!(day9_1(INPUT), 114);
    }

    #[test]
    fn test_day9_2() {
        assert_eq!(day9_2(INPUT), 2);
    }
}
