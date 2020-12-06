use std::collections::{HashMap, HashSet};

static INPUT: &str = std::include_str!("../../../input/6.txt");

fn main() {
    let groups = INPUT.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", part_one(&groups));
    println!("{:?}", part_two(&groups));
}

fn part_one(groups: &[&str]) -> usize {
    groups
        .iter()
        .enumerate()
        .map(|(_, group)| {
            let mut set: HashSet<char> = HashSet::new();
            group.split_whitespace().into_iter().for_each(|answers| {
                answers.chars().for_each(|char| {
                    set.insert(char);
                });
            });
            set.len()
        })
        .sum()
}

fn part_two(groups: &[&str]) -> usize {
    let mut group_size: HashMap<usize, i32> = HashMap::new();
    groups
        .iter()
        .enumerate()
        .map(|(i, group)| {
            let mut count: HashMap<char, i32> = HashMap::new();
            let members = group.split_whitespace().collect::<Vec<&str>>();
            group_size.insert(i, members.len() as i32);
            members.into_iter().for_each(|answers| {
                answers.chars().for_each(|char| {
                    *count.entry(char).or_insert(0) += 1;
                });
            });
            count
                .values()
                .filter(|&v| *v == *group_size.get(&i).unwrap())
                .count()
        })
        .sum()
}
