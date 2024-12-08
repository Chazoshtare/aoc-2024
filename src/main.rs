use std::fs;
use std::path::Path;

mod day1;
mod day2;

fn main() {
    let day1_input = read_input(Path::new("inputs/day1.txt"));
    let solution1_1 = day1::solve_part1(&day1_input);
    println!("Day 1, part 1 solution: {}", solution1_1);
    let solution1_2 = day1::solve_part2(&day1_input);
    println!("Day 1, part 2 solution: {}", solution1_2);

    let day2_input = read_input(Path::new("inputs/day2.txt"));
    let solution2_1 = day2::solve_part1(&day2_input);
    println!("Day 2, part 1 solution: {}", solution2_1);
    let solution2_2 = day2::solve_part2(&day2_input);
    println!("Day 2, part 2 solution: {}", solution2_2);
}

fn read_input(path: &Path) -> String {
    fs::read_to_string(path).expect(format!("couldn't read input file {:?}", path).as_str())
}
