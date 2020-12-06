static INPUT: &str = std::include_str!("../../../input/5.txt");

fn main() {
    let mut results = part_one(INPUT.split_whitespace().collect::<Vec<&str>>().as_ref());

    println!("{:?}", results.iter().max().unwrap());
    println!("{:?}", part_two(&mut results));
}

fn part_one(instructions: &[&str]) -> Vec<i32> {
    instructions
        .iter()
        .map(|x| {
            let out = x
                .replace("F", "0")
                .replace("L", "0")
                .replace("B", "1")
                .replace("R", "1");
            let row = isize::from_str_radix(&out[..7], 2).unwrap();
            let col = isize::from_str_radix(&out[7..], 2).unwrap();
            (row * 8 + col) as i32
        })
        .collect::<Vec<i32>>()
}

fn part_two(results: &mut Vec<i32>) -> i32 {
    results.sort_unstable();
    results[results
        .iter()
        .enumerate()
        .find_map(|(i, _)| {
            if results[i] + 1 < results[i + 1] {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()]
        + 1
}
