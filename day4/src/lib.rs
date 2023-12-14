use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn part1(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| {
            let split_str = line.split(':');

            let all_cards = split_str.last().unwrap();

            let mut it = all_cards.split('|');
            let winning_nums = it.next().unwrap().trim();
            let my_nums = it.next().unwrap().trim();

            let winning_set: HashSet<u32> = HashSet::from_iter(
                winning_nums
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.trim().parse::<u32>().unwrap()),
            );

            let my_set: HashSet<u32> = HashSet::from_iter(
                my_nums
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.trim().parse::<u32>().unwrap()),
            );

            let intersection = winning_set.intersection(&my_set).collect::<Vec<_>>().len();

            if intersection > 0 {
                return 2u32.pow(intersection as u32 - 1);
            }
            intersection as u32
        })
        .inspect(|v| {
            dbg!(v);
        })
        .sum();

    output.to_string()
}

pub fn part2(input: &str) -> String {
    let mut copies: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let mut split_str = line.split(':');

        let card_num: u32 = split_str
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let reps = copies.entry(card_num).or_insert(1);

        let all_cards = split_str.last().unwrap();

        let mut it = all_cards.split('|');
        let winning_nums = it.next().unwrap().trim();
        let my_nums = it.next().unwrap().trim();

        let winning_set: HashSet<u32> = HashSet::from_iter(
            winning_nums
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap()),
        );

        let my_set: HashSet<u32> = HashSet::from_iter(
            my_nums
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap()),
        );

        let intersection = winning_set.intersection(&my_set).collect::<Vec<_>>().len() as u32;

        for _ in 0..*reps {
            for c in card_num + 1..card_num + intersection + 1 {
                *copies.entry(c).or_insert(1) += 1;
            }
        }

        dbg!(&copies);
    });

    let output: u32 = copies.values().sum();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1(input);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, "30");
    }
}
