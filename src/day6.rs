use std::collections::HashSet;

const GROUND: char = '.';
const OBSTACLE: char = '#';
const MARKED: char = 'X';

pub fn solve_part1(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut location = map.get_start();
    let mut direction = Direction::North;

    while map.contains_location(&location) {
        while location.will_hit_obstacle(&map, &direction) {
            direction = direction.rotate_clockwise();
        }
        map.mark(&location);
        location = location.next(&direction);
    }
    map.count_all_marked()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut map = Map::parse(input);
    let mut location = map.get_start();
    let mut direction = Direction::North;
    let mut visited_locations: HashSet<Location> = HashSet::new();

    while map.contains_location(&location) {
        visited_locations.insert(location.clone());
        while location.will_hit_obstacle(&map, &direction) {
            direction = direction.rotate_clockwise();
        }
        location = location.next(&direction);
    }

    visited_locations.remove(&map.get_start());
    visited_locations.iter()
        .filter(|visited| {
            map.place_obstacle(visited);
            let loops = loops_on_a_map(&map, map.get_start(), Direction::North);
            map.remove_obstacle(visited);
            loops
        }).count() as u32
}

fn loops_on_a_map(map: &Map, mut location: Location, mut direction: Direction) -> bool {
    let mut hit_obstacles: HashSet<(Location, Direction)> = HashSet::new();

    while map.contains_location(&location) {
        while location.will_hit_obstacle(&map, &direction) {
            if hit_obstacles.contains(&(location.next(&direction), direction.clone())) {
                return true;
            }
            hit_obstacles.insert((location.next(&direction), direction.clone()));
            direction = direction.rotate_clockwise();
        }
        location = location.next(&direction);
    }
    false
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        Map {
            map: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn get_start(&self) -> Location {
        self.map
            .iter()
            .enumerate()
            .find_map(|(row, chars)| {
                chars.iter().enumerate().find_map(|(column, &char)| {
                    if char == '^' {
                        Some(Location {
                            x: column as i32,
                            y: row as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .expect("no initial coordinates found")
    }

    fn get(&self, location: &Location) -> Option<char> {
        if self.contains_location(location) {
            Some(self.map[location.y as usize][location.x as usize])
        } else {
            None
        }
    }

    fn contains_location(&self, location: &Location) -> bool {
        let max_index = (self.map.len() - 1) as i32;
        (location.x <= max_index && location.y <= max_index) && (location.x >= 0 && location.y >= 0)
    }

    fn place_obstacle(&mut self, location: &Location) {
        self.map[location.y as usize][location.x as usize] = OBSTACLE;
    }

    fn remove_obstacle(&mut self, location: &Location) {
        self.map[location.y as usize][location.x as usize] = GROUND;
    }

    fn mark(&mut self, location: &Location) {
        self.map[location.y as usize][location.x as usize] = MARKED;
    }

    fn count_all_marked(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|&&char| char == MARKED)
            .count()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn will_hit_obstacle(&self, map: &Map, direction: &Direction) -> bool {
        map.get(&self.next(direction)) == Some(OBSTACLE)
    }

    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Location {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Location {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Location {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Location {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = solve_part1(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn solves_part2_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = solve_part2(input);
        assert_eq!(result, 6);
    }
}
