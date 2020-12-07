use regex::Regex;
use std::collections::HashMap;

static INPUT: &str = std::include_str!("../../../input/7.txt");

fn main() {
    let parent_colour_regex = Regex::new(r"^(?P<name>\w+?\s\w+\b?)").unwrap();
    let nested_colour_regex =
        Regex::new(r"(?P<count>[0-9]+?)\s(?P<name>\b\w+?\b\s\b\w+\b?)").unwrap();

    let mut parent_bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
    INPUT.split('\n').into_iter().for_each(|line| {
        let parent_colour = parent_colour_regex.captures(line).unwrap()["name"].to_string();
        let contains = parent_bags
            .entry(parent_colour)
            .or_insert_with(HashMap::new);
        nested_colour_regex
            .captures_iter(line)
            .for_each(|nested_colour| {
                contains.insert(
                    nested_colour["name"].to_string(),
                    nested_colour["count"].parse().unwrap(),
                );
            });
    });

    let part_1_result = parent_bags
        .keys()
        .filter(|key| contains_target_colour(&parent_bags, key, "shiny gold"))
        .count();
    let part_2_result = bags_required_for_target(&parent_bags, "shiny gold");
    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}

fn contains_target_colour(
    parent_bags: &HashMap<String, HashMap<String, usize>>,
    container_colour: &str,
    target: &str,
) -> bool {
    let mut flag = false;
    if let Some(bags) = parent_bags.get(container_colour) {
        if bags.contains_key(target) {
            flag = true;
        }
        bags.keys().into_iter().for_each(|colour| {
            if contains_target_colour(parent_bags, colour, target) {
                flag = true;
            }
        });
    }
    flag
}

fn bags_required_for_target(
    parent_bags: &HashMap<String, HashMap<String, usize>>,
    target: &str,
) -> usize {
    parent_bags
        .get(target)
        .unwrap()
        .iter()
        .map(|(colour, n)| n + (n * bags_required_for_target(parent_bags, colour)))
        .sum::<usize>()
}
