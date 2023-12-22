use core::num;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpected card"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
enum HandType {
    High,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<Vec<Card>> for HandType {
    fn from(cards: Vec<Card>) -> Self {
        let mut num_cards: HashMap<Card, usize> = HashMap::new();
        for c in cards {
            *num_cards.entry(c).or_insert(1) += 1;
        }

        match num_cards.keys().len() {
            1 => Self::Five,
            2 => {
                if num_cards.values().contains(&3) {
                    if let Some(jokers) = num_cards.get(&Card::Joker) {
                        if jokers == &2 {
                            return Self::Five;
                        }
                    }

                    Self::FullHouse
                } else {
                    if let Some(jokers) = num_cards.get(&Card::Joker) {
                        if jokers == &1 {
                            return Self::Five;
                        }
                    }
                    Self::Four
                }
            }
            3 => {
                if num_cards.values().contains(&3) {
                    if let Some(jokers) = num_cards.get(&Card::Joker) {
                        if jokers == &1 {
                            return Self::Four;
                        }
                    }

                    Self::Three
                } else {
                    if let Some(jokers) = num_cards.get(&Card::Joker) {
                        if jokers == &1 {
                            return Self::FullHouse;
                        }
                    }

                    Self::Two
                }
            }
            4 => {
                if let Some(jokers) = num_cards.get(&Card::Joker) {
                    if jokers == &1 {
                        return Self::Three;
                    }
                }

                Self::One
            }
            5 => {
                if num_cards.get(&Card::Joker).is_some() {
                    return Self::One;
                }
                Self::High
            }
            _ => panic!(),
        }
    }
}

#[derive(Eq, Debug)]
struct HandBid {
    hand: Vec<Card>,
    bid: usize,
}

impl HandBid {
    fn get_handtype(&self) -> HandType {
        let mut num_cards: HashMap<Card, usize> = HashMap::new();
        for c in &self.hand {
            *num_cards.entry(c.clone()).or_insert(0) += 1;
        }

        match num_cards.keys().len() {
            1 => HandType::Five,
            2 => {
                if num_cards.get(&Card::Joker).is_some() {
                    return HandType::Five;
                }

                if num_cards.values().contains(&3) {
                    HandType::FullHouse
                } else {
                    HandType::Four
                }
            }
            3 => {
                if num_cards.values().contains(&3) {
                    if num_cards.get(&Card::Joker).is_some() {
                        return HandType::Four;
                    }

                    HandType::Three
                } else {
                    if let Some(jokers) = num_cards.get(&Card::Joker) {
                        if jokers == &1 {
                            return HandType::FullHouse;
                        } else if jokers == &2 {
                            return HandType::Four;
                        }
                    }

                    HandType::Two
                }
            }
            4 => {
                if num_cards.get(&Card::Joker).is_some() {
                    return HandType::Three;
                }

                HandType::One
            }
            5 => {
                if num_cards.get(&Card::Joker).is_some() {
                    return HandType::One;
                }
                HandType::High
            }
            _ => panic!(),
        }
    }
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(s) => s,
            None => panic!(),
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type: HandType = self.get_handtype();
        let other_hand_type: HandType = other.get_handtype();

        match hand_type.cmp(&other_hand_type) {
            core::cmp::Ordering::Equal => {
                let mut idx = 0;

                let mut ord: core::cmp::Ordering = core::cmp::Ordering::Equal;
                while ord == core::cmp::Ordering::Equal {
                    ord = self.hand[idx].cmp(&other.hand[idx]);
                    idx += 1;
                }

                println!(
                    "Comparing {:?} ({:?}) to {:?} ({:?}) is {:?}",
                    self.hand, hand_type, other.hand, other_hand_type, ord
                );

                Some(ord)
            }
            ord => {
                println!(
                    "Comparing {:?} ({:?}) to {:?} ({:?}) is {:?}",
                    self.hand, hand_type, other.hand, other_hand_type, ord
                );
                Some(ord)
            }
        }
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn part1(input: &str) -> String {
    let res: usize = input
        .lines()
        .map(|h| {
            let hand: Vec<_> = h.split_whitespace().collect();

            let r = HandBid {
                hand: hand[0].chars().map(|c| c.into()).collect(),
                bid: hand[1].parse().unwrap(),
            };
            dbg!(r.get_handtype());
            r
        })
        .sorted()
        .inspect(|f| {
            dbg!(f);
        })
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1))
        .sum();

    dbg!(res);
    res.to_string()
}

pub fn part2(input: &str) -> String {
    let res: usize = input
        .lines()
        .map(|h| {
            let hand: Vec<_> = h.split_whitespace().collect();

            let r = HandBid {
                hand: hand[0].chars().map(|c| c.into()).collect(),
                bid: hand[1].parse().unwrap(),
            };
            dbg!(r.get_handtype());
            r
        })
        .sorted()
        .inspect(|f| {
            dbg!(f);
        })
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1))
        .sum();

    dbg!(res);
    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]

    fn part1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part1(input);
        assert_eq!(result, "5905");
    }
}
