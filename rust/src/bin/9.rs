use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let xmas_data = std::include_str!("../../../input/9.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let part_one_result = part_one(&xmas_data);
    println!("{}", part_one_result);
    let part_two_result = part_two(&xmas_data, part_one_result);
    println!("{}", part_two_result);
}

fn part_one(xmas_data: &[i64]) -> i64 {
    const PREAMBLE: usize = 25;
    let mut idx = 0;
    while idx < xmas_data.len() {
        if idx < PREAMBLE {
            idx += 1;
            continue;
        }
        let seen: HashSet<&i64> = HashSet::from_iter(&xmas_data[(idx - PREAMBLE)..idx]);
        if !seen.iter().any(|&x| seen.contains(&(xmas_data[idx] - x))) {
            return xmas_data[idx];
        }
        idx += 1;
    }
    0
}

fn part_two(data: &[i64], target: i64) -> i64 {
    let res = data
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == target);

    if res.is_some() {
        if let Some(tmp) = res {
            let x = vec![tmp.0, tmp.1, tmp.2];
            return *x.iter().min().unwrap() + *x.iter().max().unwrap();
        }
    }

    0
}
