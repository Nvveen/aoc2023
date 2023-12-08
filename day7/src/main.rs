mod part1;
mod part2;
use part1::day7_1;
use part2::day7_2;

const INPUT: &str = include_str!("input");

pub fn main() {
    println!("Day 7 solution 1: {}", day7_1(INPUT));
    println!("Day 7 solution 2: {}", day7_2(INPUT));
}