pub fn {{crate_name}}_1(input: &str) -> u64 {
    0
}

pub fn {{crate_name}}_2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_day5_1() {
        assert_eq!({{crate_name}}_1(INPUT), 1);
    }

    #[test]
    fn test_day5_2() {
        assert_eq!({{crate_name}}_1(INPUT), 1);
    }
}