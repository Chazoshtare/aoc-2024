use std::collections::HashSet;

pub fn solve_part1(input: &str) -> u32 {
    let map = Map::parse(input);
    let bottoms = map.get_bottoms();
    bottoms
        .iter()
        .flat_map(|location| location.connects_to_tops_single_route(&map))
        .count() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    let map = Map::parse(input);
    let bottoms = map.get_bottoms();
    bottoms
        .iter()
        .flat_map(|location| location.connects_to_tops_multiple_route(&map))
        .count() as u32
}

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Map { map }
    }

    fn get(&self, location: &Location) -> Option<u8> {
        if self.contains(location) {
            Some(self.map[location.y as usize][location.x as usize])
        } else {
            None
        }
    }

    fn get_bottoms(&self) -> Vec<Location> {
        let mut bottoms = vec![];
        let max_index = self.map.len();
        for i in 0..max_index {
            for j in 0..max_index {
                if self.map[i][j] == 0 {
                    bottoms.push(Location {
                        x: j as i8,
                        y: i as i8,
                    });
                }
            }
        }
        bottoms
    }

    fn get_adjacent_locations_with_height(
        &self,
        current_location: &Location,
        expected_height: u8,
    ) -> Vec<Location> {
        let north = current_location.move_by(0, -1);
        let south = current_location.move_by(0, 1);
        let east = current_location.move_by(1, 0);
        let west = current_location.move_by(-1, 0);
        vec![north, south, east, west]
            .into_iter()
            .filter(|location| self.get(location) == Some(expected_height))
            .collect()
    }

    fn contains(&self, location: &Location) -> bool {
        let max_index = self.map.len() as i8 - 1;
        (location.x <= max_index && location.y <= max_index) && (location.x >= 0 && location.y >= 0)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Location {
    x: i8,
    y: i8,
}

impl Location {
    fn move_by(&self, x: i8, y: i8) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn connects_to_tops_single_route(&self, map: &Map) -> HashSet<Location> {
        let height = map.get(self).unwrap();
        if height >= 9 {
            return HashSet::from([self.clone()]);
        }
        let adjacent = map.get_adjacent_locations_with_height(self, height + 1);
        adjacent
            .iter()
            .flat_map(|location| location.connects_to_tops_single_route(map))
            .collect()
    }

    fn connects_to_tops_multiple_route(&self, map: &Map) -> Vec<Location> {
        let height = map.get(self).unwrap();
        if height >= 9 {
            return vec![self.clone()];
        }
        let adjacent = map.get_adjacent_locations_with_height(self, height + 1);
        adjacent
            .iter()
            .flat_map(|location| location.connects_to_tops_multiple_route(map))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = solve_part1(input);
        assert_eq!(result, 36);
    }

    #[test]
    fn solves_part2_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = solve_part2(input);
        assert_eq!(result, 81);
    }

    #[test]
    fn finds_two_top_connections() {
        let input = "5550555
5551555
5552555
6543456
7555557
8555558
9555559";
        let result = solve_part1(input);
        assert_eq!(result, 2);
    }
}
