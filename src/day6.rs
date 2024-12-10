const OBSTACLE: char = '#';
const MARKED: char = 'X';

pub fn solve_part1(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut location = map.get_start();
    let mut direction = Direction::NORTH;

    while map.contains_location(&location) {
        if location.will_hit_obstacle(&map, &direction) {
            direction = direction.rotate()
        } else {
            map.mark(location.x, location.y);
            location.walk(&direction)
        }
    }
    map.count_all_marked()
}

pub fn solve_part2(input: &str) -> u32 {
    0
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn parse(input: &str) -> Map {
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

    fn mark(&mut self, x: i32, y: i32) {
        self.map[y as usize][x as usize] = MARKED;
    }

    fn count_all_marked(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|&&char| char == MARKED)
            .count()
    }

    fn max_coord_index(&self) -> i32 {
        (self.map.len() - 1) as i32
    }
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn will_hit_obstacle(&self, map: &Map, direction: &Direction) -> bool {
        map.get(&self.next(direction)) == Some(OBSTACLE)
    }

    fn next(&self, direction: &Direction) -> Location {
        match direction {
            Direction::NORTH => Location {
                x: self.x,
                y: self.y - 1,
            },
            Direction::SOUTH => Location {
                x: self.x,
                y: self.y + 1,
            },
            Direction::EAST => Location {
                x: self.x + 1,
                y: self.y,
            },
            Direction::WEST => Location {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn walk(&mut self, direction: &Direction) {
        match direction {
            Direction::NORTH => self.y -= 1,
            Direction::SOUTH => self.y += 1,
            Direction::EAST => self.x += 1,
            Direction::WEST => self.x -= 1,
        }
    }
}

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH,
            Direction::WEST => Direction::NORTH,
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
