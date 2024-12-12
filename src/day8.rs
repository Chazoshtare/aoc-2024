pub fn solve_part1(input: &str) -> u32 {
    0
}

pub fn solve_part2(input: &str) -> u32 {
    0
}

fn locate_antennas(input: &str) -> Vec<Antenna> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char != '.' {
                    Some(Antenna {
                        frequency: char,
                        location: Location {
                            x: x as i32,
                            y: y as i32,
                        },
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    location: Location,
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
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
        assert_eq!(result, 0);
    }
}
