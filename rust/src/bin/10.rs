use itertools::Itertools;

fn main() {
    // parse out the adapters
    let adapters = include_str!("../../../input/10.txt")
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    println!("{:?}", part_one(&adapters));
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
