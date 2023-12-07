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
    fn test_{{crate_name}}_1() {
        assert_eq!({{crate_name}}_1(INPUT), 1);
    }

    #[test]
    fn test_{{crate_name}}_2() {
        assert_eq!({{crate_name}}_2(INPUT), 1);
    }
}