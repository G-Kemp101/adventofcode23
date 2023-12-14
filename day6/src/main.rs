fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 Solution: {}", day6::part1(input));
    println!(
        "Part 2 Solution: {}",
        day6::part2(49877895, 356137815021882)
    );
}
