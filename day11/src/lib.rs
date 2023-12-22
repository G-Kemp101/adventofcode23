use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let mut coords: Vec<(usize, usize)> = vec![];
    for (x, l) in lines.iter().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                coords.push((x, y))
            }
        }
    }

    let empty_rows: Vec<_> = input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            if l.chars().all(|f| f == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    dbg!(&empty_rows);
    let empty_columns: Vec<_> = (0..lines.len())
        .filter(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(*i))
                .all(|c| c == Some('.'))
        })
        .collect();

    let pairs: Vec<_> = coords.iter().combinations(2).collect();

    let mut sum = 0;
    for pair in pairs {
        println!("[{:?}, {:?}]", &pair[0], &pair[1]);
        let mut temp_sum = 0;
        let d_x = pair[0].0.min(pair[1].0)..pair[0].0.max(pair[1].0);
        let d_y = pair[0].1.min(pair[1].1)..pair[0].1.max(pair[1].1);

        temp_sum += pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1);
        dbg!(&d_x, &d_y);
        let added_rows: usize = d_x.filter(|f| empty_rows.contains(f)).count();
        let added_cols: usize = d_y.filter(|f| empty_columns.contains(f)).count();

        dbg!(added_rows, added_cols);
        temp_sum += added_cols + added_rows;
        dbg!(temp_sum);

        sum += temp_sum;
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let mut coords: Vec<(usize, usize)> = vec![];
    for (x, l) in lines.iter().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == '#' {
                coords.push((x, y))
            }
        }
    }

    let empty_rows: Vec<_> = input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            if l.chars().all(|f| f == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    dbg!(&empty_rows);
    let empty_columns: Vec<_> = (0..lines.len())
        .filter(|i| {
            lines
                .iter()
                .map(|l| l.chars().nth(*i))
                .all(|c| c == Some('.'))
        })
        .collect();

    let pairs: Vec<_> = coords.iter().combinations(2).collect();

    let mut sum = 0;
    for pair in pairs {
        let mut temp_sum = 0;
        let d_x = pair[0].0.min(pair[1].0)..pair[0].0.max(pair[1].0);
        let d_y = pair[0].1.min(pair[1].1)..pair[0].1.max(pair[1].1);

        temp_sum += pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1);
        let added_rows: usize = d_x.filter(|f| empty_rows.contains(f)).count() * 999999;
        let added_cols: usize = d_y.filter(|f| empty_columns.contains(f)).count() * 999999;

        temp_sum += added_cols + added_rows;

        sum += temp_sum;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(result, "374");
    }

    #[test]
    fn part2_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = part2(input);
        assert_eq!(result, "8410");
    }
}
