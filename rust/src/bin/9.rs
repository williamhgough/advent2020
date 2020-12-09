use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

fn main() {
    let xmas_data = include_str!("../../../input/9.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let now = Instant::now();
    let part_one_result = part_one(&xmas_data);
    println!("{}", part_one_result);
    let part_two_result = part_two(&xmas_data, part_one_result);
    println!("{}", part_two_result);
    println!("Time elapsed: {}", now.elapsed().as_millis());
}

fn part_one(xmas_data: &[i64]) -> i64 {
    const PREAMBLE: usize = 25;
    let mut idx = 0;
    while idx < xmas_data.len() {
        // don't care until idx >= PREAMBLE
        if idx < PREAMBLE {
            idx += 1;
            continue;
        }
        // create set from section of the data
        let seen: HashSet<&i64> = HashSet::from_iter(&xmas_data[(idx - PREAMBLE)..idx]);

        // if no matches are found in the set for the target - x
        // return the number at idx.
        if !seen.iter().any(|&x| seen.contains(&(xmas_data[idx] - x))) {
            return xmas_data[idx];
        }
        idx += 1;
    }
    0
}

fn part_two(data: &[i64], target: i64) -> i64 {
    // only run this up to the index of the previous result
    let target_idx = data.iter().position(|&x| x == target).unwrap();

    // run in increasing batch_size, start with 2 as that's the minimum.
    for batch_size in 2..data[..target_idx].len() {
        // for each subset, if the sum of the batch is equal to the target
        // then we've found the contiguous set of data. Return the min + max
        for sub_set in data[..target_idx].windows(batch_size) {
            if target.eq(&sub_set.iter().sum::<i64>()) {
                // Also viable:
                // sub_set.sort();
                // return sub_set[0] + sub_set[sub_set.len()-1]
                return sub_set.iter().min().unwrap() + sub_set.iter().max().unwrap();
            }
        }
    }
    0
}
