#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

static INPUT: &str = std::include_str!("../../../input/2.txt");

fn main() {
    println!("Part 1: {:?}", part_one());
    println!("Part 2: {:?}", part_two());
}

fn part_one() -> usize {
    let passwords = INPUT.lines();
    passwords
        .into_iter()
        .filter_map(|x| parse(x))
        .filter(|(min, max, c, pass)| {
            (*min..=*max).contains(&(*pass).chars().filter(|x| x == c).count())
        })
        .count()
}

fn part_two() -> usize {
    let passwords = INPUT.lines();
    passwords
        .into_iter()
        .filter_map(|line| {
            let (min, max, c, pass) = parse(line)?;
            // return the chars at the specified min and max indexes, and what they should be
            Some((pass.chars().nth(min - 1)?, pass.chars().nth(max - 1)?, c))
        })
        .filter(|(i, j, c)| (*i == *c) != (*j == *c)) // filter where the indexes don't match
        .count()
}

fn parse(line: &str) -> Option<(usize, usize, char, String)> {
    let result = RE.captures(line)?;
    Some((
        result.get(1)?.as_str().parse().ok()?,
        result.get(2)?.as_str().parse().ok()?,
        result.get(3)?.as_str().chars().next()?,
        result.get(4)?.as_str().to_string(),
    ))
}
