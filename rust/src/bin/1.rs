use itertools::Itertools;

static INPUT: &str = std::include_str!("../../../input/1.txt");

fn main() {
    println!("Part 1 IterTools: {:?}", part_one());
    println!("Part 2 IterTools: {:?}", part_two());
}

fn part_one() -> i64 {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let pair = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .unwrap();

    pair.0 * pair.1
}

fn part_two() -> i64 {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let triple = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .unwrap();

    triple.0 * triple.1 * triple.2
}
