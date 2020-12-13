use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input/11.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let part_one_result = run(&mut Ferry::new(&input), |f| f.layout_by_adjacency_count());
    println!("res p1: {}", part_one_result);

    let part_two_result = run(&mut Ferry::new(&input), |f| f.layout_by_visible_count());
    println!("res p2: {}", part_two_result);
}

pub fn run<F>(ferry: &mut Ferry, mut cb: F) -> i32
where
    F: FnMut(&mut Ferry) -> bool,
{
    loop {
        let has_changed = cb(ferry);
        if !has_changed {
            break;
        }
    }

    ferry.occupied_seats()
}

pub struct Ferry {
    seating_layout: Vec<Vec<char>>,
}

impl Ferry {
    pub fn new(input: &[Vec<char>]) -> Ferry {
        Ferry {
            seating_layout: input.to_vec(),
        }
    }

    pub fn layout_by_adjacency_count(&mut self) -> bool {
        let mut new_grid: Vec<Vec<char>> = self.seating_layout.to_vec();
        let mut layout_changed = false;

        for (i, row) in self.seating_layout.iter().enumerate() {
            for (j, &seat) in row.iter().enumerate() {
                let adjacent_occupants = self.adjacent_occupancy(i, j);
                match seat {
                    'L' if adjacent_occupants == 0 => {
                        new_grid[i][j] = '#';
                        layout_changed = true;
                    }
                    '#' if adjacent_occupants >= 4 => {
                        new_grid[i][j] = 'L';
                        layout_changed = true;
                    }
                    _ => {}
                }
            }
        }

        if layout_changed {
            self.seating_layout = new_grid;
        }

        layout_changed
    }

    pub fn layout_by_visible_count(&mut self) -> bool {
        let mut new_grid: Vec<Vec<char>> = self.seating_layout.to_vec();
        let mut layout_changed = false;

        for (i, row) in self.seating_layout.iter().enumerate() {
            for (j, &seat) in row.iter().enumerate() {
                let visible_occupants = self.visible_occupancy(i, j);
                match seat {
                    'L' if visible_occupants == 0 => {
                        new_grid[i][j] = '#';
                        layout_changed = true;
                    }
                    '#' if visible_occupants >= 5 => {
                        new_grid[i][j] = 'L';
                        layout_changed = true;
                    }
                    _ => {}
                }
            }
        }

        if layout_changed {
            self.seating_layout = new_grid;
        }

        layout_changed
    }

    fn adjacent_occupancy(&self, row: usize, col: usize) -> usize {
        let max_rows = self.seating_layout.len() as i32;
        let max_cols = self.seating_layout[0].len() as i32;

        vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ]
        .iter()
        .map(|(i, j)| {
            let row_num = (row as i32) + i;
            let col_num = (col as i32) + j;
            if !(*i == 0 && *j == 0)
                && (row_num < max_rows)
                && (row_num >= 0)
                && (col_num < max_cols)
                && (col_num >= 0)
                && (self.seating_layout[row_num as usize][col_num as usize] == '#')
            {
                return 1;
            }
            0
        })
        .sum()
    }

    fn visible_occupancy(&self, row: usize, col: usize) -> i32 {
        let max_rows = self.seating_layout.len() as i32;
        let max_cols = self.seating_layout[0].len() as i32;
        let mut res: i32 = 0;
        let dirs: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];

        let mut dirs_seen: Vec<bool> = vec![false; dirs.len()];
        let mut layer = 1;
        loop {
            let seats: Vec<(usize, (i32, i32))> = dirs
                .iter()
                .map(|dir| (row as i32 + layer * dir.0, col as i32 + layer * dir.1))
                .enumerate()
                .filter(|(idx, (row_num, col_num))| {
                    (!dirs_seen[*idx])
                        && (*row_num >= 0)
                        && (*row_num < max_rows)
                        && (*col_num >= 0)
                        && (*col_num < max_cols)
                })
                .collect();

            for (idx, (i, j)) in &seats {
                let seat = self.seating_layout[*i as usize][*j as usize];
                match seat {
                    '#' => {
                        dirs_seen[*idx] = true;
                        res += 1;
                    }
                    'L' => {
                        dirs_seen[*idx] = true;
                    }
                    _ => {}
                };
            }

            if seats.is_empty() {
                break;
            }

            layer += 1;
        }

        res
    }

    pub fn occupied_seats(&self) -> i32 {
        self.seating_layout
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == '#').count() as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn adjacent_occupancy_works() {
        let input: Vec<Vec<char>> = vec![
            vec!['L', 'L', '#', '#'],
            vec!['.', '#', '#', '#'],
            vec!['#', '#', '#', '#'],
        ];

        let seating = Ferry::new(&input);

        assert_eq!(seating.adjacent_occupancy(1, 1), 5);
        assert_eq!(seating.adjacent_occupancy(2, 3), 3);
        assert_eq!(seating.adjacent_occupancy(0, 0), 1);
        assert_eq!(seating.adjacent_occupancy(0, 3), 3);
    }

    #[test]
    fn visible_occupancy_works() {
        let input: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '#', '#'],
            vec!['.', '.', '.', '#', '#'],
            vec!['L', '#', '.', 'L', '#'],
            vec!['L', '#', '.', '#', 'L'],
            vec!['#', '.', '.', '#', 'L'],
        ];

        let seating = Ferry::new(&input);

        assert_eq!(seating.visible_occupancy(0, 0), 2);
        assert_eq!(seating.visible_occupancy(1, 4), 4);
        assert_eq!(seating.visible_occupancy(2, 2), 4);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../../fixtures/11.txt")
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut ferry = Ferry::new(&input);
        assert_eq!(run(&mut ferry, |f| f.layout_by_adjacency_count()), 37);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../fixtures/11.txt")
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut ferry = Ferry::new(&input);
        assert_eq!(run(&mut ferry, |f| f.layout_by_visible_count()), 26);
    }
}
