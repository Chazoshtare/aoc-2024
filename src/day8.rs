use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> u32 {
    let max_index = input.lines().next().unwrap().len() - 1;
    let antennas: HashMap<char, Vec<Location>> = locate_antennas(input);
    let anti_nodes: HashSet<Location> = antennas
        .iter()
        .flat_map(|(frequency, locations)| get_anti_nodes(locations))
        .filter(|location| location.is_valid(max_index as i32))
        .collect();
    anti_nodes.len() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    0
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
                    Entry::Vacant(mut entry) => {
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn is_valid(&self, max_index: i32) -> bool {
        self.x >= 0 && self.x <= max_index && self.y >= 0 && self.y <= max_index
    }

    fn mirror(&self, center: &Location) -> Location {
        Location {
            x: self.x * 2 - center.x,
            y: self.y * 2 - center.y,
        }
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
}
