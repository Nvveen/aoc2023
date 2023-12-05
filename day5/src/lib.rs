use std::collections::HashMap;

pub fn day5_1(input: &str) -> u64 {
    //split by empty lines
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (name_mapping, mappings) = create_mappings(sections);
    calculate_lowest(seeds, &name_mapping, &mappings)
}

pub fn day5_2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let words = sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (name_mapping, mappings) = create_mappings(sections);

    let seed_ranges = words
        .chunks(2)
        .map(|window| (window[0], window[0] + window[1]))
        .collect::<Vec<_>>();

    // reverse name_mapping
    let reverse_name_mapping = name_mapping
        .iter()
        .map(|(from, to)| (*to, *from))
        .collect::<HashMap<_, _>>();
    let mut lowest = 0;
    loop {
        let mut to = "location";
        let mut code = lowest;
        while let Some(from) = reverse_name_mapping.get(to) {
            let name = format!("{}-to-{}", from, to);
            let mapping = mappings.get(&name.as_str()).unwrap();
            code = mapping
                .iter()
                .find_map(
                    |(dest, src, len)| {
                        if code >= *dest && code < *dest + *len {
                            let diff = code - *dest;
                            Some(*src + diff)
                        } else {
                            None
                        }
                    },
                )
                .unwrap_or(code);
            to = from;
        }
        if seed_ranges
            .iter()
            .any(|(start, end)| code >= *start && code < *end)
        {
            return lowest;
        }
        lowest += 1;
    }
}

fn calculate_lowest(
    seeds: Vec<u64>,
    name_mapping: &HashMap<&str, &str>,
    mappings: &HashMap<&str, Vec<(u64, u64, u64)>>,
) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            let mut from = "seed";
            let mut code = *seed;
            while let Some(to) = name_mapping.get(from) {
                let name = format!("{}-to-{}", from, to);
                let mapping = mappings.get(&name.as_str()).unwrap();
                code = mapping
                    .iter()
                    .find_map(|(dest, src, len)| {
                        if code >= *src && code < *src + *len {
                            let diff = code - *src;
                            Some(*dest + diff)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(code);
                from = to;
            }
            code
        })
        .min()
        .unwrap()
}

fn create_mappings<'a>(
    sections: std::str::Split<'a, &str>,
) -> (
    HashMap<&'a str, &'a str>,
    HashMap<&'a str, Vec<(u64, u64, u64)>>,
) {
    let mut name_mapping: HashMap<&str, &str> = HashMap::new();
    let mappings = sections
        .map(|section| {
            let mut section_lines = section.lines();
            let name = section_lines
                .next()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap();
            let (first, second) = name.split_once("-to-").unwrap();
            name_mapping.insert(first, second);
            let mapping = section_lines
                .map(|line| {
                    let mut items = line.split_whitespace().map(|l| l.parse::<u64>().unwrap());
                    (
                        items.next().unwrap(),
                        items.next().unwrap(),
                        items.next().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            (name, mapping)
        })
        .collect::<HashMap<_, _>>();
    (name_mapping, mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_day5_1() {
        assert_eq!(day5_1(INPUT), 35);
    }

    #[test]
    fn test_day5_2() {
        assert_eq!(day5_2(INPUT), 46);
    }
}
