use std::cmp::max;

enum NumDice {
    Red = 12,
    Green = 13,
    Blue = 14,
}

pub fn part1(input: &str) -> String {
    let output = input
        .lines()
        .filter_map(|line| {
            let mut split_str = line.split(':');

            let game_id_str = split_str.next().unwrap();
            let dice = split_str.next().unwrap();

            let game_id: u32 = game_id_str.split(' ').last().unwrap().parse().unwrap();

            if dice.split(';').all(|round| {
                round.split(',').all(|d| {
                    let it: Vec<_> = d.trim().split(' ').collect();
                    let num: u32 = it[0].parse().unwrap();
                    let color = it[1];

                    match color {
                        "red" => num <= NumDice::Red as u32,
                        "green" => num <= NumDice::Green as u32,
                        "blue" => num <= NumDice::Blue as u32,
                        _ => panic!("UNRECOGNISED COLOR"),
                    }
                })
            }) {
                Some(game_id)
            } else {
                None
            }
        })
        .inspect(|v| {
            dbg!(v);
        })
        .sum::<u32>();

    output.to_string()
}

struct MinCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl MinCubes {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

pub fn part2(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut split_str = line.split(':');

            let _ = split_str.next().unwrap();
            let dice = split_str.next().unwrap();

            let mut s = MinCubes {
                red: 0,
                green: 0,
                blue: 0,
            };

            dice.split(';').for_each(|round| {
                round.split(',').for_each(|d| {
                    let it: Vec<_> = d.trim().split(' ').collect();
                    let num: u32 = it[0].parse().unwrap();
                    let color = it[1];

                    match color {
                        "red" => s.red = max(num, s.red),
                        "green" => s.green = max(num, s.green),
                        "blue" => s.blue = max(num, s.blue),
                        _ => panic!("UNRECOGNISED COLOR"),
                    };
                })
            });

            s.power()
        })
        .inspect(|v| {
            dbg!(v);
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, "2286");
    }
}
