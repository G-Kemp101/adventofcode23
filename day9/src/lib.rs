#![feature(iter_map_windows)]

pub fn part1(input: &str) -> String {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    histories
        .iter()
        .map(|history| {
            let mut p = vec![history.clone()];

            while !p.last().unwrap().iter().all(|f| *f == 0) {
                let h: Vec<i32> = p
                    .last()
                    .unwrap()
                    .iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect();

                p.push(h);
            }

            p.iter().rev().fold(0, |acc, e| acc + e.last().unwrap())
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    histories
        .iter()
        .map(|history| {
            let mut p = vec![history.clone()];

            while !p.last().unwrap().iter().all(|f| *f == 0) {
                let h: Vec<i32> = p
                    .last()
                    .unwrap()
                    .iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect();

                p.push(h);
            }

            p.iter().rev().fold(0, |acc, e| e.first().unwrap() - acc)
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part1(input);
        assert_eq!(result, "114");
    }

    #[test]
    fn part2_test() {
        let input = "10 13 16 21 30 45";

        let result = part2(input);
        assert_eq!(result, "5");
    }
}
