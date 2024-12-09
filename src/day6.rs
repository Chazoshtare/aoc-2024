const OBSTACLE: char = '#';
const MARKED: char = 'X';

pub fn solve_part1(input: &str) -> usize {
    let mut map = parse_to_map(input);
    let mut location = Location::initialize(&map);
    let mut direction = Direction::NORTH;

    let max_index = (map.len() - 1) as i32;
    while location.is_on_map(max_index) {
        if location.will_hit_obstacle(&map, &direction) {
            direction = direction.rotate()
        } else {
            map[location.y as usize][location.x as usize] = MARKED;
            location.walk(&direction)
        }
    }
    count_all_marked(&map)
}

pub fn solve_part2(input: &str) -> u32 {
    0
}

fn parse_to_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_all_marked(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .flatten()
        .filter(|&&char| char == MARKED)
        .count()
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn initialize(map: &Vec<Vec<char>>) -> Location {
        map.iter()
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

    fn will_hit_obstacle(&self, map: &Vec<Vec<char>>, direction: &Direction) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;
        if self.will_leave_map(map.len() as i32 - 1, direction) {
            false
        } else {
            match direction {
                Direction::NORTH => map[y - 1][x] == OBSTACLE,
                Direction::SOUTH => map[y + 1][x] == OBSTACLE,
                Direction::EAST => map[y][x + 1] == OBSTACLE,
                Direction::WEST => map[y][x - 1] == OBSTACLE,
            }
        }
    }

    fn will_leave_map(&self, max_index: i32, direction: &Direction) -> bool {
        match direction {
            Direction::NORTH => self.y - 1 < 0,
            Direction::SOUTH => self.y + 1 > max_index,
            Direction::EAST => self.x + 1 > max_index,
            Direction::WEST => self.x - 1 < 0,
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

    fn is_on_map(&self, max_index: i32) -> bool {
        (self.x <= max_index && self.y <= max_index) && (self.x >= 0 && self.y >= 0)
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
        assert_eq!(result, 0);
    }
}
