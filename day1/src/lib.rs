pub fn part1(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));
            let first = it.next().expect("Did not find number");
            let last = it.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("Unable to parse")
        })
        .sum::<u32>();

    output.to_string()
}

pub fn part2(input: &str) -> String {
    let output = input
        .lines()
        .inspect(|v| {
            dbg!(v);
        })
        .map(|line| {
            let mut index = 0;

            let is_num = |c: char, i| {
                if c.is_ascii_digit() {
                    c.to_digit(10)
                } else {
                    let line_slice = &line[i..];
                    dbg!(line_slice);

                    if line_slice.starts_with("one") {
                        Some(1)
                    } else if line_slice.starts_with("two") {
                        Some(2)
                    } else if line_slice.starts_with("three") {
                        Some(3)
                    } else if line_slice.starts_with("four") {
                        Some(4)
                    } else if line_slice.starts_with("five") {
                        Some(5)
                    } else if line_slice.starts_with("six") {
                        Some(6)
                    } else if line_slice.starts_with("seven") {
                        Some(7)
                    } else if line_slice.starts_with("eight") {
                        Some(8)
                    } else if line_slice.starts_with("nine") {
                        Some(9)
                    } else {
                        None
                    }
                }
            };
            let first = line
                .chars()
                .find_map(|c| {
                    let r = is_num(c, index);

                    index += 1;
                    r
                })
                .unwrap();
            dbg!(first);

            let mut index = line.len() - 1;
            let last = line
                .chars()
                .rev()
                .find_map(|c| {
                    let r = is_num(c, index);
                    if r.is_none() {
                        index -= 1;
                    }
                    r
                })
                .unwrap();

            format!("{first}{last}")
                .parse::<u32>()
                .expect("Unable to parse")
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1(input);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
6sbzfqfdm";

        let result = part2(input);
        assert_eq!(result, "347");
    }
}
