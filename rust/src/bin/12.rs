use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input/12.txt")
        .split('\n')
        .collect_vec();
    let mut ship = Ship::new(&input);
    let part_one_result = ship.journey();
    println!("Manhattan distance: {}", part_one_result)
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn change(&self, left: bool, units: i32) -> Direction {
        match self {
            Direction::North => match units {
                90 if left => return Direction::West,
                90 if !left => return Direction::East,
                180 => return Direction::South,
                270 if left => return Direction::East,
                270 if !left => return Direction::West,
                _ => {}
            },
            Direction::East => match units {
                90 if left => return Direction::North,
                90 if !left => return Direction::South,
                180 => return Direction::West,
                270 if left => return Direction::South,
                270 if !left => return Direction::North,
                _ => {}
            },
            Direction::South => match units {
                90 if left => return Direction::East,
                90 if !left => return Direction::West,
                180 => return Direction::North,
                270 if left => return Direction::West,
                270 if !left => return Direction::East,
                _ => {}
            },
            Direction::West => match units {
                90 if left => return Direction::South,
                90 if !left => return Direction::North,
                180 => return Direction::East,
                270 if left => return Direction::North,
                270 if !left => return Direction::South,
                _ => {}
            },
        }
        Direction::East
    }
}

struct Ship<'a> {
    route: &'a [&'a str],
    direction: Direction,
    units_north: i32,
    units_east: i32,
    units_south: i32,
    units_west: i32,
}

impl<'a> Ship<'a> {
    fn new(input: &'a [&str]) -> Self {
        Self {
            route: input,
            direction: Direction::East,
            units_north: 0,
            units_east: 0,
            units_south: 0,
            units_west: 0,
        }
    }

    fn journey(&mut self) -> usize {
        for &instruction in self.route {
            let instr = instruction.chars().next().unwrap();
            let units = &instruction[1..].parse::<i32>().unwrap();
            match instr {
                'N' => self.units_north += units,
                'S' => self.units_south += units,
                'E' => self.units_east += units,
                'W' => self.units_west += units,
                'L' => self.direction = self.direction.change(true, *units),
                'R' => self.direction = self.direction.change(false, *units),
                'F' => match self.direction {
                    Direction::North => self.units_north += units,
                    Direction::East => self.units_east += units,
                    Direction::South => self.units_south += units,
                    Direction::West => self.units_west += units,
                },
                _ => {}
            }
        }
        self.manhattan_distance()
    }

    /// Calculates the manhattan distance between the two provided grid cells
    pub fn manhattan_distance(&self) -> usize {
        let x_diff = if self.units_west < self.units_east {
            self.units_east - self.units_west
        } else {
            self.units_west - self.units_east
        };
        let y_diff = if self.units_north < self.units_south {
            self.units_south - self.units_north
        } else {
            self.units_north - self.units_south
        };
        (x_diff + y_diff) as usize
    }
}
