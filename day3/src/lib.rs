use std::{collections::HashMap, usize};

pub fn part1(input: &str) -> String {
    let input_b: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let mut part_numbers = Vec::<u32>::new();

    input_b.iter().enumerate().for_each(|(r, line)| {
        let mut curr_num = String::new();
        let mut is_part = false;

        line.iter().enumerate().for_each(|(col, c)| {
            if c.is_ascii_digit() {
                curr_num.push(*c as char);
                dbg!(*c as char);
                // Search
                for col_d in -1i16..2 {
                    for row_d in -1i16..2 {
                        let y_coord: usize = match (r as i16 + row_d).try_into() {
                            Ok(d) => d,
                            Err(e) => continue,
                        };
                        let x_coord: usize = match (col as i16 + col_d).try_into() {
                            Ok(d) => d,
                            Err(e) => continue,
                        };

                        if x_coord >= input_b.len() || y_coord >= line.len() {
                            continue;
                        }
                        let coord = input_b[y_coord][x_coord] as char;

                        if coord == '.' {
                            continue;
                        } else if coord.is_ascii_punctuation() {
                            dbg!(coord);
                            is_part = true;
                        }
                    }
                }
            } else if !curr_num.is_empty() {
                if is_part {
                    part_numbers.push(curr_num.parse().unwrap());
                }
                curr_num = String::new();
                is_part = false;
            }

            if col == line.len() - 1 && is_part {
                part_numbers.push(curr_num.parse().unwrap());
            }
        });
    });

    let output: u32 = part_numbers.iter().sum();
    dbg!(part_numbers);

    output.to_string()
}

struct Gear {
    num: u8,
    ratio: usize,
}

pub fn part2(input: &str) -> String {
    let input_b: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();

    let mut gear_ratios: HashMap<(usize, usize), Gear> = HashMap::<(usize, usize), Gear>::new();

    input_b.iter().enumerate().for_each(|(r, line)| {
        let mut curr_num = String::new();
        let mut gear_coord: Option<(usize, usize)> = None;

        line.iter().enumerate().for_each(|(col, c)| {
            if c.is_ascii_digit() {
                curr_num.push(*c as char);
                // Search
                for col_d in -1i16..2 {
                    for row_d in -1i16..2 {
                        let y_coord: usize = match (r as i16 + row_d).try_into() {
                            Ok(d) => d,
                            Err(e) => continue,
                        };
                        let x_coord: usize = match (col as i16 + col_d).try_into() {
                            Ok(d) => d,
                            Err(e) => continue,
                        };

                        if x_coord >= input_b.len() || y_coord >= line.len() {
                            continue;
                        }
                        let coord = input_b[y_coord][x_coord] as char;

                        if coord == '.' {
                            continue;
                        } else if coord == '*' {
                            gear_coord = Some((x_coord, y_coord))
                        }
                    }
                }
            } else if !curr_num.is_empty() {
                if let Some(gear_coord) = gear_coord {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        gear_ratios.entry(gear_coord)
                    {
                        let gear_ratio = Gear {
                            num: 1,
                            ratio: curr_num.parse().unwrap(),
                        };
                        e.insert(gear_ratio);
                    } else {
                        let gear_ratio = gear_ratios.get_mut(&gear_coord).unwrap();

                        gear_ratio.num += 1;
                        gear_ratio.ratio *= curr_num.parse::<usize>().unwrap();
                    }
                }

                curr_num = String::new();
                gear_coord = None;
            }

            if col == line.len() - 1 {
                if let Some(gear_coord) = gear_coord {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        gear_ratios.entry(gear_coord)
                    {
                        let gear_ratio = Gear {
                            num: 1,
                            ratio: curr_num.parse().unwrap(),
                        };
                        e.insert(gear_ratio);
                    } else {
                        let gear_ratio: &mut Gear = gear_ratios.get_mut(&gear_coord).unwrap();

                        gear_ratio.num += 1;
                        gear_ratio.ratio *= curr_num.parse::<usize>().unwrap();
                    }
                }
            }
        });
    });

    let output: usize = gear_ratios
        .values()
        .filter_map(|v| if v.num == 2 { Some(v.ratio) } else { None })
        .sum();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
...$.*...-
.664.598..";

        let result = part1(input);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part2(input);
        assert_eq!(result, "467835");
    }
}
