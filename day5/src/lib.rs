#[derive(Debug)]
struct Seed {
    loc: i64,
    mapped: bool,
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let seeds_ranges: Vec<i64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seeds: Vec<Seed> = Vec::new();

    for chunk in seeds_ranges.chunks(2) {
        let start = chunk[0];
        let end = chunk[1];
        dbg!(start, end);
        seeds.extend((start..start + end).map(|s| Seed {
            loc: s,
            mapped: false,
        }))
    }

    for line in lines {
        let category_map: [i64; 3] = match line
            .split_whitespace()
            .map_while(|s| s.parse().ok())
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(v) => v,
            Err(_) => {
                seeds.iter_mut().for_each(|s| s.mapped = false);
                continue;
            }
        };

        seeds.iter_mut().for_each(|s| {
            if category_map[1] <= s.loc && s.loc < category_map[1] + category_map[2] && !s.mapped {
                s.loc += category_map[0] - category_map[1];
                s.mapped = true;
            }
        });
    }

    seeds.iter().map(|s| s.loc).min().unwrap().to_string()
}

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut seeds: Vec<Seed> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| {
            let seed = s.parse().unwrap();
            Seed {
                loc: seed,
                mapped: false,
            }
        })
        .collect();

    for line in lines {
        let category_map: [i64; 3] = match line
            .split_whitespace()
            .map_while(|s| s.parse().ok())
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(v) => v,
            Err(_) => {
                seeds.iter_mut().for_each(|s| s.mapped = false);
                continue;
            }
        };

        seeds.iter_mut().for_each(|s| {
            if category_map[1] <= s.loc && s.loc < category_map[1] + category_map[2] && !s.mapped {
                s.loc += category_map[0] - category_map[1];
                s.mapped = true;
            }
        });
    }

    seeds.iter().map(|s| s.loc).min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "seeds: 79 14 55 13

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
56 93 4";

        let result = part1(input);
        assert_eq!(result, "35");
    }

    #[test]
    fn part2_test() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        let result = part2(input);
        assert_eq!(result, "46");
    }
}
