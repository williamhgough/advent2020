static INPUT: &str = std::include_str!("../../../input/3.txt");

fn main() {
    println!("Part 1: {:?}", run_calculate_for(vec![(3, 1)]));
    println!(
        "Part 2: {:?}",
        run_calculate_for(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
    );
}

fn run_calculate_for(pairs: Vec<(i32, i32)>) -> i64 {
    let grid = INPUT.lines().collect::<Vec<&str>>();
    pairs
        .iter()
        .map(|(x, y)| calculate_grid_hits(&grid, "#", *x, *y))
        .collect::<Vec<i64>>()
        .iter()
        .product()
}

fn calculate_grid_hits(grid: &[&str], target: &str, right: i32, down: i32) -> i64 {
    let (mut x, mut y, mut hits): (i32, i32, i64) = (0, 0, 0);
    while y < (grid.len() - 1) as i32 {
        x += right;
        y += down;
        let line = grid[y as usize];
        let current_x = x % (line.len() as i32);

        if line.chars().nth(current_x as usize).unwrap() == target.chars().next().unwrap() {
            hits += 1;
        }
    }
    hits
}
