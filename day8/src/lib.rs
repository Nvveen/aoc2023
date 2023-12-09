use std::collections::HashMap;

use fancy_regex::Regex;

pub fn day8_1(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let r = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let steps = lines
        .skip(1)
        .map(|line| {
            let caps = r.captures(line).unwrap().unwrap();
            let (from, to1, to2) = (
                caps[1].to_string(),
                caps[2].to_string(),
                caps[3].to_string(),
            );
            (from, (to1, to2))
        })
        .collect::<HashMap<_, _>>();
    let mut position = "AAA".to_string();
    let mut step_count = 0;
    while let Some(step) = steps.get(&position) {
        position = match instructions.get(step_count % instructions.len()) {
            Some('L') => step.0.clone(),
            Some('R') => step.1.clone(),
            _ => panic!("Invalid instruction"),
        };
        step_count += 1;
        if position == "ZZZ" {
            break;
        }
    }
    step_count
}

pub fn day8_2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let r = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    // get all steps ending in A
    let steps = lines
        .skip(1)
        .map(|line| {
            let caps = r.captures(line).unwrap().unwrap();
            let (from, to1, to2) = (
                caps[1].to_string(),
                caps[2].to_string(),
                caps[3].to_string(),
            );
            (from, (to1, to2))
        })
        .collect::<HashMap<_, _>>();
    // get all positions keys ending with A to start with
    let start = steps
        .iter()
        .filter(|(k, _v)| k.ends_with("A"))
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();
    let counts = start
        .iter()
        .cloned()
        .map(|pos| {
            let mut next = pos;
            for (step, i) in instructions.iter().cycle().enumerate() {
                if next.ends_with("Z") {
                    return step;
                }
                next = match i {
                    'L' => steps.get(&next).unwrap().0.clone(),
                    'R' => steps.get(&next).unwrap().1.clone(),
                    _ => panic!("Invalid instruction"),
                }
            }
            0
        })
        .collect::<Vec<_>>();
    counts.iter().fold(1, |acc, x| lcm(acc, x))
}

fn lcm(a: usize, b: &usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: &usize) -> usize {
    if *b == 0 {
        a
    } else {
        gcd(*b, &(a % b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_1() {
        let input1 = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

        let input2 = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(day8_1(input1), 2);
        assert_eq!(day8_1(input2), 6);
    }

    #[test]
    fn test_day8_2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(day8_2(input), 6);
    }
}
