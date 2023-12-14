pub fn part1(input: &str) -> String {
    let mut it = input.lines();
    let times: Vec<usize> = it
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse().unwrap())
        .collect();

    let record_distances: Vec<usize> = it
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse().unwrap())
        .collect();

    let output: usize = times
        .iter()
        .zip(&record_distances)
        .map(|(t, d)| {
            let mut distances: Vec<usize> = Vec::new();

            for speed in 0..*t {
                let total_distance = speed * (t - speed);
                if total_distance > *d {
                    distances.push(total_distance);
                }
            }

            distances.len()
        })
        .product();

    output.to_string()
}

pub fn part2(t: usize, d: usize) -> String {
    let mut distances: Vec<usize> = Vec::new();

    for speed in 0..t {
        let total_distance = speed * (t - speed);
        if total_distance > d {
            distances.push(total_distance);
        }
    }

    distances.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]

    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = part1(input);
        assert_eq!(result, "288");
    }

    #[test]
    fn part2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(71530, 940200);
        assert_eq!(result, "71503");
    }
}
