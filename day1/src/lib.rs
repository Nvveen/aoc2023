pub fn day1(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // multiline string
        let input = r#"
        1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let output = day1(input);
        assert_eq!(output, 142);
    }
}
