use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    // parse out the adapters
    let adapters = include_str!("../../../input/10.txt")
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let mut now = Instant::now();
    let part_one_result = part_one(&adapters);
    println!(
        "Part 1: {:?}. Duration: {}us",
        part_one_result,
        now.elapsed().as_micros()
    );

    now = Instant::now();
    let part_two_result = part_two(&adapters);
    println!(
        "Part 2: {:?}. Duration: {}us",
        part_two_result,
        now.elapsed().as_micros()
    );
}

fn part_one(data: &[u64]) -> u64 {
    let mut adapters = data.to_vec();
    // stick 0 in there to start from.
    adapters.push(0);
    // sort it before processing.
    adapters.sort_unstable();

    (adapters
        .windows(2)
        .into_iter()
        .filter(|chunk| chunk[1] - chunk[0] == 1)
        .count()
        * (adapters
            .windows(2)
            .into_iter()
            .filter(|chunk| chunk[1] - chunk[0] == 3)
            .count()
            + 1)) as u64
}

fn part_two(data: &[u64]) -> usize {
    let mut adapters = data.to_vec();
    adapters.sort_unstable();
    let max_rating = data.iter().max().unwrap() + 3;
    let mut cache: FxHashMap<u64, usize> = FxHashMap::default();
    different_chains(0, max_rating, &adapters, &mut cache)
}

fn different_chains(
    initial_rating: u64,
    max_rating: u64,
    adapters: &[u64],
    cache: &mut FxHashMap<u64, usize>,
) -> usize {
    // check cache to see if we already have the total for the
    // current starting index. If we don't do this it will take
    // forever to execute.
    if cache.contains_key(&initial_rating) {
        return cache[&initial_rating];
    }

    // start at 0 or 1 for our final adapter, since it can only connect
    // to our device.
    let mut total = (max_rating - initial_rating <= 3) as usize;
    adapters
        .iter()
        .take_while(|&&rating| rating - initial_rating <= 3) // take all with delta <= 3
        .enumerate()
        .for_each(|(idx, &rating)| {
            // for each adapter, recursively add total value from the chain after this point.
            total += different_chains(rating, max_rating, &adapters[idx + 1..], cache)
        });
    cache.insert(initial_rating, total);
    total
}
