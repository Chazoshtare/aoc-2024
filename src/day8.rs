use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> u32 {
    let max_index = input.lines().next().unwrap().len() - 1;
    let antennas: HashMap<char, Vec<Location>> = locate_antennas(input);
    let anti_nodes: HashSet<Location> = antennas
        .iter()
        .flat_map(|(_, locations)| get_anti_nodes(locations))
        .filter(|location| location.is_valid(max_index as i32))
        .collect();
    anti_nodes.len() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    let max_index = input.lines().next().unwrap().len() - 1;
    let antennas: HashMap<char, Vec<Location>> = locate_antennas(input);
    let anti_nodes: HashSet<Location> = antennas
        .iter()
        .flat_map(|(_, locations)| get_harmonic_anti_nodes(locations, max_index as i32))
        .filter(|location| location.is_valid(max_index as i32))
        .collect();
    anti_nodes.len() as u32
}

fn locate_antennas(input: &str) -> HashMap<char, Vec<Location>> {
    let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char != '.' {
                let location = Location {
                    x: x as i32,
                    y: y as i32,
                };
                match antennas.entry(char) {
                    Entry::Vacant(entry) => {
                        entry.insert(vec![location]);
                    }
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(location);
                    }
                }
            }
        })
    });
    antennas
}

fn get_anti_nodes(antennas: &Vec<Location>) -> Vec<Location> {
    let count = antennas.len();
    let mut anti_nodes: Vec<Location> = Vec::new();
    for i in 0..count {
        for j in 0..count {
            if i != j {
                anti_nodes.push(antennas[i].mirror(&antennas[j]));
            }
        }
    }
    anti_nodes
}

fn get_harmonic_anti_nodes(antennas: &Vec<Location>, max_index: i32) -> Vec<Location> {
    let count = antennas.len();
    let mut anti_nodes: Vec<Location> = Vec::new();
    for i in 0..count {
        for j in 0..count {
            if i != j {
                let mut all_harmonics = antennas[i].all_steps(&antennas[j], max_index);
                anti_nodes.append(&mut all_harmonics);
            }
        }
    }
    anti_nodes
}

#[derive(Hash, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn is_valid(&self, max_index: i32) -> bool {
        self.x >= 0 && self.x <= max_index && self.y >= 0 && self.y <= max_index
    }

    fn add(&self, x: i32, y: i32) -> Self {
        Location {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn mirror(&self, center: &Location) -> Self {
        Location {
            x: self.x * 2 - center.x,
            y: self.y * 2 - center.y,
        }
    }

    fn all_steps(&self, other: &Self, max_index: i32) -> Vec<Self> {
        let difference_x = self.x - other.x;
        let difference_y = self.y - other.y;
        let mut locations: Vec<Location> = Vec::new();
        let mut location = other.add(difference_x, difference_y);
        while location.is_valid(max_index) {
            let next_location = location.add(difference_x, difference_y);
            locations.push(location);
            location = next_location;
        }
        locations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let result = solve_part1(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn solves_part2_example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let result = solve_part2(input);
        assert_eq!(result, 34);
    }

    #[test]
    fn solves_part2_t_example() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let result = solve_part2(input);
        assert_eq!(result, 9);
    }
}
