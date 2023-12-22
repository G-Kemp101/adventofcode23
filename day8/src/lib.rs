use num::integer::lcm;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

pub fn part1(input: &str) -> String {
    let mut it = input.lines();
    let instructions = it.next().unwrap();

    let nodes: HashMap<String, Node> = it
        .skip(1)
        .map(|n| {
            (
                n[0..3].to_string(),
                Node {
                    left: n[7..10].to_string(),
                    right: n[12..15].to_string(),
                },
            )
        })
        .collect();

    let mut v: String = "AAA".to_string();
    let mut steps = 0;
    while v != *"ZZZ" {
        for instruction in instructions.chars() {
            let node = nodes.get(&v).unwrap();

            if instruction == 'R' {
                v = node.right.clone();
            } else {
                v = node.left.clone();
            }

            steps += 1;
            if v == *"ZZZ" {
                break;
            }
        }
    }

    dbg!(nodes);

    steps.to_string()
}

pub fn part2(input: &str) -> String {
    let mut it = input.lines();
    let instructions = it.next().unwrap();
    dbg!(instructions);
    let nodes: HashMap<String, Node> = it
        .skip(1)
        .map(|n| {
            (
                n[0..3].to_string(),
                Node {
                    left: n[7..10].to_string(),
                    right: n[12..15].to_string(),
                },
            )
        })
        .collect();

    let a_nodes: Vec<String> = nodes
        .clone()
        .into_keys()
        .filter(|f| f.ends_with('A'))
        .collect();

    let mut steps = vec![0; a_nodes.len()];

    // for v in a_nodes {
    //     for instruction in instructions.chars() {
    //         let node = nodes.get(&v).unwrap();

    //         if instruction == 'R' {
    //             v = node.right.clone();
    //         } else {
    //             v = node.left.clone();
    //         }

    //         steps += 1;
    //         if v == *"ZZZ" {
    //             break;
    //         }
    //     }
    // }
    for (i, v) in a_nodes.iter().enumerate() {
        let mut n = v.clone();
        while !n.ends_with('Z') {
            for instruction in instructions.chars() {
                let node = nodes.get(&n).unwrap();

                if instruction == 'R' {
                    n = node.right.clone();
                } else {
                    n = node.left.clone();
                }
                steps[i] += 1;
                if n.ends_with('Z') {
                    break;
                }
            }
        }
    }

    steps.into_iter().reduce(lcm).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let result = part1(input);
        assert_eq!(result, "2");

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = part1(input);
        assert_eq!(result, "6");
    }

    #[test]
    fn part2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = part2(input);
        assert_eq!(result, "6");
    }
}
