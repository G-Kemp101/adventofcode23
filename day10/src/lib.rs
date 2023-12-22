#[derive(PartialEq, Debug)]
enum Floor {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Floor {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::V,
            '-' => Self::H,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("what"),
        }
    }
}

impl Floor {
    fn follows(&self, current: &Floor, offset: (isize, isize)) -> bool {
        // dbg!(self, current, offset);
        match offset {
            (0, -1) => {
                matches!(current, Floor::H | Floor::NW | Floor::SW | Floor::Start)
                    && matches!(self, Floor::H | Floor::NE | Floor::SE | Floor::Start)
            }
            (0, 1) => {
                matches!(current, Floor::H | Floor::NE | Floor::SE | Floor::Start)
                    && matches!(self, Floor::H | Floor::NW | Floor::SW | Floor::Start)
            }
            (1, 0) => {
                matches!(current, Floor::V | Floor::SW | Floor::SE | Floor::Start)
                    && matches!(self, Floor::V | Floor::NW | Floor::NE | Floor::Start)
            }
            (-1, 0) => {
                matches!(current, Floor::V | Floor::NE | Floor::NW | Floor::Start)
                    && matches!(self, Floor::V | Floor::SE | Floor::SW | Floor::Start)
            }
            _ => panic!(),
        }
    }
}

pub fn part1(input: &str) -> String {
    let floor_map: Vec<Vec<Floor>> = input
        .lines()
        .map(|l| l.chars().map(Floor::from).collect())
        .collect();

    let start = floor_map
        .iter()
        .enumerate()
        .find_map(|(i, y)| y.iter().position(|x| *x == Floor::Start).map(|y| (i, y)))
        .unwrap();

    let mut d: (usize, usize) = start;
    let mut prev = start;

    let mut steps = 0;
    loop {
        for delta in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let current = &floor_map[d.0][d.1];

            let c1 = match d.0.checked_add_signed(delta.0) {
                Some(c) => c,
                None => continue,
            };

            let c2 = match d.1.checked_add_signed(delta.1) {
                Some(c) => c,
                None => continue,
            };

            if c1 >= floor_map.len() || c2 >= floor_map[0].len() {
                continue;
            }

            // dbg!(delta, current, &floor_map[c1][c2]);
            if floor_map[c1][c2].follows(current, delta) && (c1, c2) != prev {
                println!("{:?} follows {:?}", floor_map[c1][c2], current);
                prev = d;
                d = (c1, c2);
                break;
            }
        }
        steps += 1;
        if d == start {
            break;
        }
        println!("===");
    }

    println!("{:?}", steps);
    (steps / 2).to_string()
}

pub fn part2(input: &str) -> String {
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let result = part1(input);
        assert_eq!(result, "4");

        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let result = part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_test() {
        let input = "10 13 16 21 30 45";

        let result = part2(input);
        assert_eq!(result, "5");
    }
}
