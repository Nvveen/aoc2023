use {{ crate_name }}::{ {{ crate_name }}_1, {{ crate_name }}_2 };

const INPUT: &str = include_str!("input");
{% assign day = crate_name | remove: "day" %}
pub fn main() {
    println!("Day {{ day }} solution 1: {}", {{ crate_name }}_1(INPUT));
    println!("Day {{ day }} solution 2: {}", {{ crate_name }}_2(INPUT));
}